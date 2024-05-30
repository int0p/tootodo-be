use crate::domain::sub::chat::MsgModel;
use crate::infra::types::{ChatType, FetchFilterOptions};
use crate::interface::dto::event::res::{EventFetchRes, EventFetchedRes};

use chrono::prelude::*;
use mongodb::bson::doc;
use mongodb::bson::{self, oid::ObjectId};
use mongodb::{bson::Document, Database};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::interface::dto::event::{
    req::{CreateEventReq, UpdateEventReq},
    res::{EventData, EventRes, SingleEventRes},
};

use crate::{
    domain::error::{Error::*, Result},
    domain::repo::base::{self, MongoRepo},
    infra::db::error::Error as DBError,
};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub user: Uuid,
    pub title: String,
    pub complete: bool,
    pub start_date: Option<NaiveDate>,
    pub due_at: Option<DateTime<Utc>>,
    pub location: Option<String>,
    pub chat_type: ChatType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_msgs: Option<Vec<MsgModel>>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct EventService;

impl MongoRepo for EventService {
    const COLL_NAME: &'static str = "events";
    type Model = EventModel;
    type ModelResponse = EventRes;
    type ModelFetchResponse = EventFetchRes;
    fn convert_doc_to_response(event: &EventModel) -> Result<EventRes> {
        Ok(EventRes::from_model(event))
    }

    fn create_doc<CreateEventReq: Serialize>(
        user: &Uuid,
        body: &CreateEventReq,
    ) -> Result<Document> {
        let ser_data = bson::to_bson(body).map_err(|e| DB(DBError::MongoSerializeBsonError(e)))?;
        let doc = ser_data.as_document().unwrap();

        let datetime = Utc::now();

        let mut doc_with_date = doc! {
            "user": user,
            "complete":false,
            "chat_type": "Event",
            "createdAt": datetime,
            "updatedAt": datetime,
        };
        doc_with_date.extend(doc.clone());
        Ok(doc_with_date)
    }
}

impl EventService {
    //mongodb에서 event를 가져옴.
    pub async fn fetch_events(
        db: &Database,
        limit: i64,
        page: i64,
        user: &Uuid,
    ) -> Result<EventFetchedRes> {
        let filter_opts = FetchFilterOptions {
            find_filter: None,
            proj_opts: Some(EventFetchRes::build_projection()),
            limit,
            page,
        };
        let events_results = base::fetch::<Self>(db, filter_opts, user)
            .await
            .expect("event 응답을 받아오지 못했습니다.");

        Ok(EventFetchedRes {
            status: "success",
            results: events_results.len(),
            events: events_results,
        })
    }

    pub async fn create_event(
        db: &Database,
        body: &CreateEventReq,
        user: &Uuid,
    ) -> Result<SingleEventRes> {
        let event_result = base::create::<Self, CreateEventReq>(db, body, user)
            .await
            .expect("event 생성에 실패했습니다.");

        Ok(SingleEventRes {
            status: "success",
            data: EventData {
                event: event_result,
            },
        })
    }

    pub async fn get_event(db: &Database, id: &str, user: &Uuid) -> Result<SingleEventRes> {
        let event_result = base::get::<Self>(db, id, user)
            .await
            .expect("event를 가져오는데 실패했습니다.");

        Ok(SingleEventRes {
            status: "success",
            data: EventData {
                event: event_result,
            },
        })
    }

    pub async fn update_event(
        db: &Database,
        id: &str,
        body: &UpdateEventReq,
        user: &Uuid,
    ) -> Result<SingleEventRes> {
        let event_result = base::update::<Self, UpdateEventReq>(db, id, body, user)
            .await
            .expect("event 업데이트에 실패했습니다.");

        Ok(SingleEventRes {
            status: "success",
            data: EventData {
                event: event_result,
            },
        })
    }

    pub async fn delete_event(db: &Database, id: &str) -> Result<()> {
        base::delete::<Self>(db, id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        infra::db::MongoDB,
        infra::types::{ChatType, MsgType},
    };
    use dotenv::dotenv;
    use mongodb::{bson::oid::ObjectId, options::UpdateOptions};
    use std::str::FromStr;

    async fn setup() -> Database {
        dotenv().ok();
        std::env::set_var("RUST_BACKTRACE", "0");
        let mongodb = MongoDB::init_test().await.unwrap();

        // 시드 데이터 생성
        let user = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let seeds = vec![
            EventModel {
                id: ObjectId::from_str("507f1f77bcf86cd799439011").unwrap(),
                user,
                title: "잼미니 대회 관련 미팅 1회차".to_string(),
                complete: true,
                chat_type: ChatType::Event,
                chat_msgs: None,
                start_date: Some(Utc::now().date_naive()),
                due_at: Some(Utc.with_ymd_and_hms(2024, 5, 28, 0, 0, 0).unwrap()),
                location: None,
                createdAt: Utc::now(),
                updatedAt: Utc::now(),
            },
            EventModel {
                id: ObjectId::from_str("507f1f77bcf86cd799439013").unwrap(),
                user,
                title: "잼미니 대회 관련 미팅 2회차".to_string(),
                complete: true,
                chat_type: ChatType::Event,
                chat_msgs: Some(vec![MsgModel {
                    id: ObjectId::new(),
                    msg_type: MsgType::Ask,
                    content: "기술스택 토론 예정".to_string(),
                    createdAt: Utc::now(),
                    booked: false,
                    chat_type: None,
                    chat_msgs: None,
                }]),
                start_date: Some(Utc::now().date_naive()),
                due_at: Some(Utc.with_ymd_and_hms(2024, 5, 30, 0, 0, 0).unwrap()),
                location: Some("학교".to_string()),
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
                .collection::<EventModel>("events")
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
    async fn test_create_event() {
        let db = setup().await;
        let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let body1 = CreateEventReq {
            title: "Test Event".to_string(),
            start_date: None,
            due_at: None,
            location: None,
        };

        let body2 = CreateEventReq {
            title: "Test Event2".to_string(),
            start_date: None,
            due_at: Some(Utc::now()),
            location: None,
        };

        let res = EventService::create_event(&db, &body1, &user_id).await;
        claim::assert_ok!(&res);
        let res = res.unwrap();
        claim::assert_matches!(res.status, "success");
        assert_eq!(res.data.event.title, body1.title);

        let res = EventService::create_event(&db, &body2, &user_id).await;
        claim::assert_ok!(&res);
        let res = res.unwrap();
        claim::assert_matches!(res.status, "success");
        assert_eq!(res.data.event.title, body2.title);
    }

    #[tokio::test]
    async fn test_fetch_events() {
        let db = setup().await;
        let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let limit = 10;
        let page = 1;

        let res = EventService::fetch_events(&db, limit, page, &user_id).await;
        claim::assert_ok!(&res);
        let res = res.unwrap();
        claim::assert_matches!(res.status, "success");
    }

    #[tokio::test]
    async fn test_get_event() {
        let db = setup().await;
        let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let event_id = "507f1f77bcf86cd799439013";

        let res = EventService::get_event(&db, event_id, &user_id).await;
        claim::assert_ok!(&res);
        let res = res.unwrap();
        claim::assert_matches!(res.status, "success");
        assert_eq!(res.data.event.id, event_id);
    }

    #[tokio::test]
    async fn test_update_event() {
        let db = setup().await;
        let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let event_id = "507f1f77bcf86cd799439013";
        let body = UpdateEventReq {
            title: Some("Updated Title".to_string()),
            complete: Some(true),
            start_date: None,
            due_at: None,
            location: None,
            chat_type: Some(ChatType::Task),
        };

        let res = EventService::update_event(&db, event_id, &body, &user_id).await;
        claim::assert_ok!(&res);
        let res = res.unwrap();
        claim::assert_matches!(res.status, "success");
        assert_eq!(res.data.event.title, body.title.unwrap());
        // if let Some(content) = body.content{
        //     assert_eq!(res.data.event.content, content);
        // }
        // else {dbg!(res.data.event.content);} //기존값 유지
    }

    #[tokio::test]
    async fn test_delete_event() {
        let db = setup().await;
        let event_id = "507f1f77bcf86cd799439011";

        let res = EventService::delete_event(&db, event_id).await;
        claim::assert_ok!(&res);
    }
}
