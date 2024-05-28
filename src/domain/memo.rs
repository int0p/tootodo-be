use chrono::{DateTime, Utc};
use mongodb::bson::doc;
use mongodb::bson::{self, oid::ObjectId};
use mongodb::{bson::Document, Database};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::interface::dto::memo::{
    req::{CreateMemoReq, UpdateMemoReq},
    res::{MemoData, MemoListRes, MemoRes, SingleMemoRes},
};

use crate::{
    domain::error::{Error::*, Result},
    domain::repo::base::{self, MongoRepo},
    infra::db::error::Error as DBError,
};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemoModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub user: Uuid,
    pub title: String,
    pub content: String,
    pub color: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>,
}

pub struct MemoService;

impl MongoRepo for MemoService {
    const COLL_NAME: &'static str = "memos";
    const DOC_COLL_NAME: &'static str = "memos";
    type Model = MemoModel;
    type ModelResponse = MemoRes;

    fn convert_doc_to_response(memo: &MemoModel) -> Result<MemoRes> {
        Ok(MemoRes::from_model(memo))
    }

    fn create_doc<CreateMemoReq: Serialize>(user: &Uuid, body: &CreateMemoReq) -> Result<Document> {
        let serialized_data =
            bson::to_bson(body).map_err(|e| DB(DBError::MongoSerializeBsonError(e)))?;
        let document = serialized_data.as_document().unwrap();
        let datetime = Utc::now();
        let mut doc_with_dates = doc! {
            "user": user,
            "createdAt": datetime,
            "updatedAt": datetime,
            "content": "",
        };
        doc_with_dates.extend(document.clone());
        Ok(doc_with_dates)
    }
}

impl MemoService {
    //mongodb에서 memo를 가져옴.
    pub async fn fetch_memos(
        db: &Database,
        limit: i64,
        page: i64,
        user: &Uuid,
    ) -> Result<MemoListRes> {
        let memos_result = base::fetch::<Self>(db, limit, page, user)
            .await
            .expect("memo 응답을 받아오지 못했습니다.");

        Ok(MemoListRes {
            status: "success",
            results: memos_result.len(),
            memos: memos_result,
        })
    }

    pub async fn create_memo(
        db: &Database,
        body: &CreateMemoReq,
        user: &Uuid,
    ) -> Result<SingleMemoRes> {
        let memo_result = base::create::<Self, CreateMemoReq>(db, body, user)
            .await
            .expect("memo 생성에 실패했습니다.");

        Ok(SingleMemoRes {
            status: "success",
            data: MemoData { memo: memo_result },
        })
    }

    pub async fn get_memo(db: &Database, id: &str, user: &Uuid) -> Result<SingleMemoRes> {
        let memo_result = base::get::<Self>(db, id, user)
            .await
            .expect("memo를 가져오는데 실패했습니다.");

        Ok(SingleMemoRes {
            status: "success",
            data: MemoData { memo: memo_result },
        })
    }

    pub async fn update_memo(
        db: &Database,
        id: &str,
        body: &UpdateMemoReq,
        user: &Uuid,
    ) -> Result<SingleMemoRes> {
        let memo_result = base::update::<Self, UpdateMemoReq>(db, id, body, user)
            .await
            .expect("memo 업데이트에 실패했습니다.");

        Ok(SingleMemoRes {
            status: "success",
            data: MemoData { memo: memo_result },
        })
    }

    pub async fn delete_memo(db: &Database, id: &str) -> Result<()> {
        base::delete::<Self>(db, id).await
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::infra::db::MongoDB;
    use dotenv::dotenv;
    use mongodb::{bson::oid::ObjectId, options::UpdateOptions};

    async fn setup() -> Database {
        dotenv().ok();
        std::env::set_var("RUST_BACKTRACE", "0");
        let mongodb = MongoDB::init_test().await.unwrap();

        // 시드 데이터 생성
        let user = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let seeds = vec![
            MemoModel {
                id: ObjectId::from_str("507f1f77bcf86cd799439011").unwrap(),
                user,
                title: "첫 번째 노트".to_string(),
                content: "첫 번째 노트의 내용입니다.".to_string(),
                color: "#f97316".to_string(),
                createdAt: Utc::now(),
                updatedAt: Utc::now(),
            },
            MemoModel {
                id: ObjectId::from_str("507f191e810c19729de860ea").unwrap(),
                user,
                title: "두 번째 노트".to_string(),
                content: "두 번째 노트의 내용입니다.".to_string(),
                color: "#06b6d4".to_string(),
                createdAt: Utc::now(),
                updatedAt: Utc::now(),
            },
            MemoModel {
                id: ObjectId::from_str("507f191e810c19729de860ec").unwrap(),
                user,
                title: "세 번째 노트".to_string(),
                content: "세 번째 노트의 내용입니다.".to_string(),
                color: "#84cc16".to_string(),
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
                .collection::<MemoModel>("memos")
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
    async fn test_create_memo() {
        let db = setup().await;
        let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let body = CreateMemoReq {
            user: user_id,
            title: "Test Memo".to_string(),
            color: "#71717a".to_string(),
        };

        let res = MemoService::create_memo(&db, &body, &user_id).await;
        claim::assert_ok!(&res);
        let res = res.unwrap();
        claim::assert_matches!(res.status, "success");
        assert_eq!(res.data.memo.title, body.title);
    }

    #[tokio::test]
    async fn test_fetch_memos() {
        let db = setup().await;
        let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let limit = 10;
        let page = 1;

        let res = MemoService::fetch_memos(&db, limit, page, &user_id).await;
        claim::assert_ok!(&res);
        let res = res.unwrap();
        claim::assert_matches!(res.status, "success");
    }

    #[tokio::test]
    async fn test_get_memo() {
        let db = setup().await;
        let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let memo_id = "507f1f77bcf86cd799439011";

        let res = MemoService::get_memo(&db, memo_id, &user_id).await;
        claim::assert_ok!(&res);
        let res = res.unwrap();
        claim::assert_matches!(res.status, "success");
        assert_eq!(res.data.memo.id, memo_id);
    }

    #[tokio::test]
    async fn test_update_memo() {
        let db = setup().await;
        let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let memo_id = "507f1f77bcf86cd799439011";
        let body = UpdateMemoReq {
            title: Some("Updated Title".to_string()),
            content: None, // No change to content
            color: Some("#10b981".to_string()),
        };

        let res = MemoService::update_memo(&db, memo_id, &body, &user_id).await;
        claim::assert_ok!(&res);
        let res = res.unwrap();
        claim::assert_matches!(res.status, "success");
        assert_eq!(res.data.memo.title, body.title.unwrap());
        if let Some(content) = body.content {
            assert_eq!(res.data.memo.content, content);
        } else {
            dbg!(res.data.memo.content);
        } //기존값 유지
    }

    #[tokio::test]
    async fn test_delete_memo() {
        let db = setup().await;
        let memo_id = "507f191e810c19729de860ec";

        let res = MemoService::delete_memo(&db, memo_id).await;
        claim::assert_ok!(&res);
    }
}
