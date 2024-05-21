use std::{collections::HashSet, str::FromStr};

use super::{
    model::{CategoryModel, PropertyModel, PropertyType},
    response::{CategoryData, CategoryListResponse, CategoryResponse, SingleCategoryResponse},
    schema::{CreateCategorySchema, UpdateCategorySchema},
};
use crate::{
    db::error::Error as DBError,
    models::{
        base::{self, MongoBMC},
        error::{Error::*, Result},
    },
};
use chrono::prelude::*;
use mongodb::bson::{self, Bson};
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::{bson::Document, Database};
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct CategoryBMC;

impl MongoBMC for CategoryBMC {
    const COLL_NAME: &'static str = "categories";
    const DOC_COLL_NAME: &'static str = "categories";
    type Model = CategoryModel;
    type ModelResponse = CategoryResponse;

    fn convert_doc_to_response(category: &CategoryModel) -> Result<CategoryResponse> {
        let category_response = CategoryResponse {
            user: category.user,
            id: category.id.to_hex(),
            name: category.name,
            color: category.color,
            properties: category.properties,
            createdAt: category.createdAt,
            updatedAt: category.updatedAt,
        };
        Ok(category_response)
    }

    fn create_doc<CreateCategorySchema: Serialize>(
        user: &Uuid,
        body: &CreateCategorySchema,
    ) -> Result<Document> {
        let serialized_data =
            bson::to_bson(body).map_err(|e| DB(DBError::MongoSerializeBsonError(e)))?;
        let document = serialized_data.as_document().unwrap();

        let properties = [PropertyModel::new(
            ObjectId::new(),
            "Tags".to_string(),
            PropertyType::MultiSelect,
            None,
        )];
        let serialized_properties =
            bson::to_bson(&properties).map_err(|e| DB(DBError::MongoSerializeBsonError(e)))?;
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

impl CategoryBMC {
    //mongodb에서 category를 가져옴.
    pub async fn fetch_categorys(
        db: &Database,
        limit: i64,
        page: i64,
        user: &Uuid,
    ) -> Result<CategoryListResponse> {
        let categorys_result = base::fetch::<Self>(db, limit, page, user)
            .await
            .expect("category 응답을 받아오지 못했습니다.");

        Ok(CategoryListResponse {
            status: "success",
            results: categorys_result.len(),
            categories: categorys_result,
        })
    }

    pub async fn create_category(
        db: &Database,
        body: &CreateCategorySchema,
        user: &Uuid,
    ) -> Result<SingleCategoryResponse> {
        let category_result = base::create::<Self, CreateCategorySchema>(db, body, user)
            .await
            .expect("category 생성에 실패했습니다.");

        Ok(SingleCategoryResponse {
            status: "success",
            data: CategoryData {
                category: category_result,
            },
        })
    }

    pub async fn create_property(
        db: &Database,
        category_id: &str,
        body: &PropertyModel,
        user: &Uuid,
    ) -> Result<SingleCategoryResponse> {
        // todo: category_id으로 category를 찾고, category의 properties배열에 입력받은 body를 추가하는 함수

        Ok(SingleCategoryResponse {
            status: "success",
            data: CategoryData {
                category: category_result,
            },
        })
    }

    pub async fn get_category(
        db: &Database,
        id: &str,
        user: &Uuid,
    ) -> Result<SingleCategoryResponse> {
        let category_result = base::get::<Self>(db, id, user)
            .await
            .expect("category를 가져오는데 실패했습니다.");

        Ok(SingleCategoryResponse {
            status: "success",
            data: CategoryData {
                category: category_result,
            },
        })
    }

    pub async fn update_category(
        db: &Database,
        id: &str,
        body: &UpdateCategorySchema, //color, name
        user: &Uuid,
    ) -> Result<SingleCategoryResponse> {
        let category_result = base::update::<Self, UpdateCategorySchema>(db, id, body, user)
            .await
            .expect("category 업데이트에 실패했습니다.");

        // TODO: category를 포함하는 task들의 category정보 변경

        Ok(SingleCategoryResponse {
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
            ObjectId::from_str(delete_category_id).map_err(|e| DBError::MongoGetOidError(e))?;
        let target_category_id =
            ObjectId::from_str(target_category_id).map_err(|e| DBError::MongoGetOidError(e))?;

        // Step 1: Get the properties of the category to be deleted
        let delete_category = match coll.find_one(doc! {"_id": delete_category_id}, None).await {
            Ok(Some(doc)) => doc,
            Ok(None) => return Err(NotFoundError(delete_category_id.to_string())),
            Err(e) => return Err(DB(DBError::MongoQueryError(e))),
        };

        let properties_to_move = delete_category.properties;

        // Step 2: Get the target category's existing properties
        let target_category = match coll.find_one(doc! {"_id": target_category_id}, None).await {
            Ok(Some(doc)) => doc,
            Ok(None) => return Err(NotFoundError(delete_category_id.to_string())),
            Err(e) => return Err(DB(DBError::MongoQueryError(e))),
        };

        // 이름 중복 여부를 확인하는 작업의 효율성을 높이기 위해 HashSet 사용
        // TODO: 어떤 option을 복제할지 역시 입력으로 받아와야함.
        let existing_property_names: HashSet<_> = target_category
            .properties
            .iter()
            .map(|prop| &prop.name)
            .collect();
        let mut existing_property_options: HashSet<String> = HashSet::new();

        for prop in &target_category.properties {
            if let Some(options) = &prop.options {
                existing_property_options.extend(options.clone());
            }
        }

        // Filter and process properties to move
        let mut filtered_properties_to_move = Vec::new();

        for prop in properties_to_move {
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
                        filtered_properties_to_move.push(updated_prop);
                    }
                }
            } else {
                filtered_properties_to_move.push(prop);
            }
        }

        // Convert filtered properties to Bson
        let properties_to_move_bson: Vec<Bson> = filtered_properties_to_move
            .into_iter()
            .map(|prop| bson::to_bson(&prop).unwrap())
            .collect();

        // Step 3: Add these filtered properties to the target category
        if !properties_to_move_bson.is_empty() {
            coll.update_one(
                doc! {"_id": target_category_id},
                doc! {"$push": {"properties": {"$each": properties_to_move_bson}}},
                None,
            )
            .await
            .map_err(|e| DBError::MongoQueryError(e))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::{db::MongoDB, models::category::model::StatusType};
    use dotenv::dotenv;
    use mongodb::options::UpdateOptions;

    async fn setup() -> Database {
        dotenv().ok();
        std::env::set_var("RUST_BACKTRACE", "0");
        let mongodb = MongoDB::init_test().await.unwrap();

        // 시드 데이터 생성
        let user = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let seeds = vec![
            CategoryModel {
                id: ObjectId::from_str("507f1f77bcf86cd799439011").unwrap(),
                user,
                name: "Work".to_string(),
                color: "Red".to_string(),
                status: StatusType::InProgress,
                properties: vec![
                    PropertyModel::new(
                        ObjectId::new(),
                        "Tags".to_string(),
                        PropertyType::MultiSelect,
                        Some(vec!["FE".to_string(), "BE".to_string(), "OP".to_string()]),
                    ),
                    PropertyModel::new(
                        ObjectId::new(),
                        "Deadline".to_string(),
                        PropertyType::DateTime,
                        None,
                    ),
                ],
                createdAt: Utc::now(),
                updatedAt: Utc::now(),
            },
            CategoryModel {
                id: ObjectId::from_str("507f1f77bcf86cd799439013").unwrap(),
                user,
                name: "Personal".to_string(),
                color: "Blue".to_string(),
                status: StatusType::Archived,
                properties: vec![
                    PropertyModel::new(
                        ObjectId::new(),
                        "Tags".to_string(),
                        PropertyType::MultiSelect,
                        Some(vec!["FE".to_string(), "BE".to_string(), "OP".to_string()]),
                    ),
                    PropertyModel::new(
                        ObjectId::new(),
                        "Summary".to_string(),
                        PropertyType::Text,
                        None,
                    ),
                ],
                createdAt: Utc::now(),
                updatedAt: Utc::now(),
            },
        ];

        // 시드 데이터를 MongoDB에 삽입
        for seed in seeds {
            let filter = doc! { "_id": seed.id };
            let update = doc! { "$setOnInsert": bson::to_bson(&seed).unwrap() };
            let options = UpdateOptions::builder().upsert(true).build();

            let result = mongodb
                .db
                .collection::<CategoryModel>("categorys")
                .update_one(filter, update, options)
                .await
                .expect("cannot insert seed data");

            // if result.upserted_id.is_some() {
            //     println!(
            //         "✅ 새로운 노트 시드 데이터가 추가되었습니다. ID: {}",
            //         seed.id
            //     );
            // } else {
            //     println!("이미 존재하는 노트 시드 데이터입니다. ID: {}", seed.id);
            // }
        }

        mongodb.db
    }

    #[tokio::test]
    async fn test_create_category() {
        let db = setup().await;
        let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let body = CreateCategorySchema {
            name: "Test Category1".to_string(),
            color: "red".to_string(),
        };

        let res = CategoryBMC::create_category(&db, &body, &user_id).await;
        claim::assert_ok!(&res);
        let res = res.unwrap();
        claim::assert_matches!(res.status, "success");
        assert_eq!(res.data.category.title, body.name);
    }

    #[tokio::test]
    async fn test_fetch_categorys() {
        let db = setup().await;
        let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let limit = 10;
        let page = 1;

        let res = CategoryBMC::fetch_categorys(&db, limit, page, &user_id).await;
        claim::assert_ok!(&res);
        let res = res.unwrap();
        claim::assert_matches!(res.status, "success");
    }

    #[tokio::test]
    async fn test_get_category() {
        let db = setup().await;
        let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let category_id = "507f1f77bcf86cd799439013";

        let res = CategoryBMC::get_category(&db, category_id, &user_id).await;
        claim::assert_ok!(&res);
        let res = res.unwrap();
        claim::assert_matches!(res.status, "success");
        assert_eq!(res.data.category.id, category_id);
    }

    #[tokio::test]
    async fn test_update_category() {
        let db = setup().await;
        let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let category_id = "507f1f77bcf86cd799439013";
        let body = UpdateCategorySchema {
            name: Some("Updated Category".to_string()),
            color: None,
            status: Some(StatusType::Archived),
        };

        let res = CategoryBMC::update_category(&db, category_id, &body, &user_id).await;
        claim::assert_ok!(&res);
        let res = res.unwrap();
        claim::assert_matches!(res.status, "success");
        assert_eq!(res.data.category.title, body.title.unwrap());
        // if let Some(content) = body.content{
        //     assert_eq!(res.data.category.content, content);
        // }
        // else {dbg!(res.data.category.content);} //기존값 유지
    }

    #[tokio::test]
    async fn test_delete_category() {
        let db = setup().await;
        let category_id = "507f1f77bcf86cd799439011";
        let target_id = "507f1f77bcf86cd799439013";

        let res = CategoryBMC::delete_category(&db, category_id, Some(target_id)).await;
        claim::assert_ok!(&res);
    }
}
