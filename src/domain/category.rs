use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::{collections::HashSet, str::FromStr};

use super::sub::property::PropertyModel;
use crate::infra::types::{PropertyType, QueryFilterOptions, StatusType};
use crate::{
    domain::error::{Error::*, Result},
    domain::repo::base::{self, MongoRepo},
    infra::db::error::Error as DBError,
    interface::dto::category::{
        req::{CreateCategoryReq, UpdateCategoryReq},
        res::{CategoryData, CategoryListRes, CategoryRes, SingleCategoryRes},
    },
};
use mongodb::{
    bson::{self, doc, oid::ObjectId, Bson, Document},
    Database,
};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub user: Uuid,
    pub name: String,                                 
    pub color: String,
    pub status: StatusType,
    pub props: Vec<PropertyModel>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct CategoryService;

impl MongoRepo for CategoryService {
    const COLL_NAME: &'static str = "categories";
    type Model = CategoryModel;
    type ModelResponse = CategoryRes;
    fn convert_doc_to_response(category: &CategoryModel) -> Result<CategoryRes> {
        let category_response = CategoryRes {
            user: category.user,
            id: category.id.to_hex(),
            name: category.name.to_owned(),
            color: category.color.to_owned(),
            props: category.props.to_owned(),
            createdAt: category.createdAt,
            updatedAt: category.updatedAt,
            status: category.status.to_owned(),
        };
        Ok(category_response)
    }

    fn create_doc<CreateCategoryReq: Serialize>(
        user: &Uuid,
        body: &CreateCategoryReq,
    ) -> Result<Document> {
        let serialized_data =
            bson::to_bson(body).map_err(|e| DB(DBError::MongoSerializeBsonError(e)))?;
        let document = serialized_data.as_document().unwrap();

        let props = [PropertyModel::new(
            ObjectId::new(),
            "Tags".to_string(),
            PropertyType::MultiSelect,
            None,
        )];
        let _serialized_props =
            bson::to_bson(&props).map_err(|e| DB(DBError::MongoSerializeBsonError(e)))?;
        let datetime = Utc::now();

        let mut doc_with_dates = doc! {
            "user": user,
            "status": "InProgress",
            "createdAt": datetime,
            "updatedAt": datetime,
        };
        doc_with_dates.extend(document.clone());
        Ok(doc_with_dates)
    }
}

impl CategoryService {
    //mongodb에서 category를 가져옴.
    pub async fn fetch_categories(
        db: &Database,
        limit: i64,
        page: i64,
        user: &Uuid,
    ) -> Result<CategoryListRes> {
        let filter_opts = QueryFilterOptions {
            find_filter: None,
            proj_opts: None,
            limit,
            page,
        };
        let categories_result = base::fetch::<Self>(db, filter_opts, user)
            .await
            .expect("category 응답을 받아오지 못했습니다.");

        Ok(CategoryListRes {
            status: "success",
            results: categories_result.len(),
            categories: categories_result,
        })
    }

    pub async fn create_category(
        db: &Database,
        body: &CreateCategoryReq,
        user: &Uuid,
    ) -> Result<SingleCategoryRes> {
        let category_result = base::create::<Self, CreateCategoryReq>(db, body, user, None)
            .await
            .expect("category 생성에 실패했습니다.");

        Ok(SingleCategoryRes {
            status: "success",
            data: CategoryData {
                category: category_result,
            },
        })
    }

    pub async fn get_category(db: &Database, id: &str, user: &Uuid) -> Result<SingleCategoryRes> {
        let category_result = base::get::<Self>(db, id, user)
            .await
            .expect("category를 가져오는데 실패했습니다.");

        Ok(SingleCategoryRes {
            status: "success",
            data: CategoryData {
                category: category_result,
            },
        })
    }

    pub async fn update_category(
        db: &Database,
        id: &str,
        body: &UpdateCategoryReq, //color, name
        user: &Uuid,
    ) -> Result<SingleCategoryRes> {
        let category_result = base::update::<Self, UpdateCategoryReq>(db, id, body, user)
            .await
            .expect("category 업데이트에 실패했습니다.");

        // TODO: category를 포함하는 task들의 category정보 변경

        Ok(SingleCategoryRes {
            status: "success",
            data: CategoryData {
                category: category_result,
            },
        })
    }

    pub async fn delete_category(db: &Database, id: &str, target_id: Option<&str>) -> Result<()> {
        if let Some(target_id) = target_id {
            Self::move_prop_to_target_category(db, id, target_id)
                .await
                .expect("property 복제에 실패했습니다.");
        }
        base::delete::<Self>(db, id).await
    }

    async fn move_prop_to_target_category(
        db: &Database,
        delete_category_id: &str,
        target_category_id: &str,
    ) -> Result<()> {
        let coll = db.collection::<CategoryModel>(Self::COLL_NAME);
        let delete_category_id =
            ObjectId::from_str(delete_category_id).map_err(DBError::MongoGetOidError)?;
        let target_category_id =
            ObjectId::from_str(target_category_id).map_err(DBError::MongoGetOidError)?;

        // Step 1: Get the props of the category to be deleted
        let delete_category = match coll.find_one(doc! {"_id": delete_category_id}, None).await {
            Ok(Some(doc)) => doc,
            Ok(None) => return Err(NotFoundError(delete_category_id.to_string())),
            Err(e) => return Err(DB(DBError::MongoQueryError(e))),
        };

        let props_to_move = delete_category.props;

        // Step 2: Get the target category's existing props
        let target_category = match coll.find_one(doc! {"_id": target_category_id}, None).await {
            Ok(Some(doc)) => doc,
            Ok(None) => return Err(NotFoundError(delete_category_id.to_string())),
            Err(e) => return Err(DB(DBError::MongoQueryError(e))),
        };

        // 이름 중복 여부를 확인하는 작업의 효율성을 높이기 위해 HashSet 사용
        // TODO: 어떤 option을 복제할지 역시 입력으로 받아와야함.
        let existing_property_names: HashSet<_> = target_category
            .props
            .iter()
            .map(|prop| &prop.name)
            .collect();
        let mut existing_property_options: HashSet<String> = HashSet::new();

        for prop in &target_category.props {
            if let Some(options) = &prop.options {
                existing_property_options.extend(options.clone());
            }
        }

        // Filter and process props to move
        let mut filtered_props_to_move = Vec::new();

        for prop in props_to_move {
            if existing_property_names.contains(&prop.name) {
                if let Some(options) = &prop.options {
                    let new_options: Vec<String> = options
                        .iter()
                        .filter(|option| !existing_property_options.contains(*option))
                        .cloned()
                        .collect();

                    if !new_options.is_empty() {
                        let mut updated_prop = prop.clone();
                        updated_prop.options = Some(new_options);
                        filtered_props_to_move.push(updated_prop);
                    }
                }
            } else {
                filtered_props_to_move.push(prop);
            }
        }

        // Convert filtered props to Bson
        let props_to_move_bson: Vec<Bson> = filtered_props_to_move
            .into_iter()
            .map(|prop| bson::to_bson(&prop).unwrap())
            .collect();

        // Step 3: Add these filtered props to the target category
        if !props_to_move_bson.is_empty() {
            coll.update_one(
                doc! {"_id": target_category_id},
                doc! {"$push": {"props": {"$each": props_to_move_bson}}},
                None,
            )
            .await
            .map_err(DBError::MongoQueryError)?;
        }

        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use std::str::FromStr;

//     use super::*;
//     use crate::{domain::types::StatusType, infra::db::MongoDB};
//     use dotenv::dotenv;
//     use mongodb::options::UpdateOptions;

//     async fn setup() -> Database {
//         dotenv().ok();
//         std::env::set_var("RUST_BACKTRACE", "0");
//         let mongodb = MongoDB::init_test().await.unwrap();

//         // 시드 데이터 생성
//         let user = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
//         let seeds = vec![
//             CategoryModel {
//                 id: ObjectId::from_str("507f1f77bcf86cd799439011").unwrap(),
//                 user,
//                 name: "Work".to_string(),
//                 color: "Red".to_string(),
//                 status: StatusType::InProgress,
//                 props: vec![
//                     PropertyModel::new(
//                         ObjectId::new(),
//                         "Tags".to_string(),
//                         PropertyType::MultiSelect,
//                         Some(vec!["FE".to_string(), "BE".to_string(), "OP".to_string()]),
//                     ),
//                     PropertyModel::new(
//                         ObjectId::new(),
//                         "Deadline".to_string(),
//                         PropertyType::DateTime,
//                         None,
//                     ),
//                 ],
//                 createdAt: Utc::now(),
//                 updatedAt: Utc::now(),
//             },
//             CategoryModel {
//                 id: ObjectId::from_str("507f1f77bcf86cd799439013").unwrap(),
//                 user,
//                 name: "Personal".to_string(),
//                 color: "Blue".to_string(),
//                 status: StatusType::Archived,
//                 props: vec![
//                     PropertyModel::new(
//                         ObjectId::new(),
//                         "Tags".to_string(),
//                         PropertyType::MultiSelect,
//                         Some(vec!["FE".to_string(), "BE".to_string(), "OP".to_string()]),
//                     ),
//                     PropertyModel::new(
//                         ObjectId::new(),
//                         "Summary".to_string(),
//                         PropertyType::Text,
//                         None,
//                     ),
//                 ],
//                 createdAt: Utc::now(),
//                 updatedAt: Utc::now(),
//             },
//         ];

//         // 시드 데이터를 MongoDB에 삽입
//         for seed in seeds {
//             let filter = doc! { "_id": seed.id };
//             let update = doc! { "$setOnInsert": bson::to_bson(&seed).unwrap() };
//             let options = UpdateOptions::builder().upsert(true).build();

//             let result = mongodb
//                 .db
//                 .collection::<CategoryModel>("categories")
//                 .update_one(filter, update, options)
//                 .await
//                 .expect("cannot insert seed data");

//             // if result.upserted_id.is_some() {
//             //     println!(
//             //         "✅ 새로운 노트 시드 데이터가 추가되었습니다. ID: {}",
//             //         seed.id
//             //     );
//             // } else {
//             //     println!("이미 존재하는 노트 시드 데이터입니다. ID: {}", seed.id);
//             // }
//         }

//         mongodb.db
//     }

//     #[tokio::test]
//     async fn test_create_category() {
//         let db = setup().await;
//         let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
//         let body = CreateCategoryReq {
//             name: "Test Category1".to_string(),
//             color: "red".to_string(),
//         };

//         let res = CategoryService::create_category(&db, &body, &user_id).await;
//         claim::assert_ok!(&res);
//         let res = res.unwrap();
//         claim::assert_matches!(res.status, "success");
//         assert_eq!(res.data.category.name, body.name);
//     }

//     #[tokio::test]
//     async fn test_fetch_categories() {
//         let db = setup().await;
//         let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
//         let limit = 10;
//         let page = 1;

//         let res = CategoryService::fetch_categories(&db, limit, page, &user_id).await;
//         claim::assert_ok!(&res);
//         let res = res.unwrap();
//         claim::assert_matches!(res.status, "success");
//     }

//     #[tokio::test]
//     async fn test_get_category() {
//         let db = setup().await;
//         let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
//         let category_id = "507f1f77bcf86cd799439013";

//         let res = CategoryService::get_category(&db, category_id, &user_id).await;
//         claim::assert_ok!(&res);
//         let res = res.unwrap();
//         claim::assert_matches!(res.status, "success");
//         assert_eq!(res.data.category.id, category_id);
//     }

//     #[tokio::test]
//     async fn test_update_category() {
//         let db = setup().await;
//         let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
//         let category_id = "507f1f77bcf86cd799439013";
//         let body = UpdateCategoryReq {
//             name: Some("Updated Category".to_string()),
//             color: None,
//             status: Some(StatusType::Archived),
//         };

//         let res = CategoryService::update_category(&db, category_id, &body, &user_id).await;
//         claim::assert_ok!(&res);
//         let res = res.unwrap();
//         claim::assert_matches!(res.status, "success");
//         assert_eq!(res.data.category.name, body.name.unwrap());
//         // if let Some(content) = body.content{
//         //     assert_eq!(res.data.category.content, content);
//         // }
//         // else {dbg!(res.data.category.content);} //기존값 유지
//     }

//     #[tokio::test]
//     async fn test_delete_category() {
//         let db = setup().await;
//         let category_id = "507f1f77bcf86cd799439011";
//         let target_id = "507f1f77bcf86cd799439013";

//         let res = CategoryService::delete_category(&db, category_id, Some(target_id)).await;
//         claim::assert_ok!(&res);
//     }
// }
