use super::{
    model::EventModel,
    response::{EventData, EventListResponse, EventResponse, SingleEventResponse},
    schema::{CreateEventSchema, UpdateEventSchema},
};
use crate::{
    db::error::Error as DBError,
    models::{
        base::{self, MongoBMC},
        error::{Error::*, Result},
    },
};
use chrono::prelude::*;
use mongodb::bson;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::{bson::Document, Database};
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct EventBMC;

impl MongoBMC for EventBMC {
    const COLL_NAME: &'static str = "events";
    const DOC_COLL_NAME: &'static str = "events";
    type Model = EventModel;
    type ModelResponse = EventResponse;

    fn convert_doc_to_response(event: &EventModel) -> Result<EventResponse> {
        let event_response = EventResponse {
            user: event.user,
            id: event.id.to_hex(),
            title: event.title.to_owned(),
            complete: event.complete.to_owned(),
            start_date: event.start_date.to_owned(),
            due_at: event.due_at.to_owned(),
            location: event.location.to_owned(),
            chat_type: event.chat_type.to_owned(),
            chat_msgs: event.chat_msgs.to_owned(),
            createdAt: event.createdAt,
            updatedAt: event.updatedAt,
        };
        Ok(event_response)
    }

    fn create_doc<CreateEventSchema: Serialize>(
        user: &Uuid,
        body: &CreateEventSchema,
    ) -> Result<Document> {
        let serialized_data =
            bson::to_bson(body).map_err(|e| DB(DBError::MongoSerializeBsonError(e)))?;
        let document = serialized_data.as_document().unwrap();

        // let msgs = ChatModel {
        //     src_type: ChatType::Event,
        //     msgs: None,
        // };
        // let serialized_chat = bson::to_bson(&msgs).map_err(|e| DB(DBError::MongoSerializeBsonError(e)))?;
        let datetime = Utc::now();

        let mut doc_with_dates = doc! {
            "user": user,
            "complete":false,
            "chat_type": "Event",
            // "chat": serialized_chat,
            "createdAt": datetime,
            "updatedAt": datetime,
        };
        doc_with_dates.extend(document.clone());
        Ok(doc_with_dates)
    }
}

impl EventBMC {
    //mongodb에서 event를 가져옴.
    pub async fn fetch_events(
        db: &Database,
        limit: i64,
        page: i64,
        user: &Uuid,
    ) -> Result<EventListResponse> {
        let events_result = base::fetch::<Self>(db, limit, page, user)
            .await
            .expect("event 응답을 받아오지 못했습니다.");

        Ok(EventListResponse {
            status: "success",
            results: events_result.len(),
            events: events_result,
        })
    }

    pub async fn create_event(
        db: &Database,
        body: &CreateEventSchema,
        user: &Uuid,
    ) -> Result<SingleEventResponse> {
        let event_result = base::create::<Self, CreateEventSchema>(db, body, user)
            .await
            .expect("event 생성에 실패했습니다.");

        Ok(SingleEventResponse {
            status: "success",
            data: EventData {
                event: event_result,
            },
        })
    }

    pub async fn get_event(db: &Database, id: &str, user: &Uuid) -> Result<SingleEventResponse> {
        let event_result = base::get::<Self>(db, id, user)
            .await
            .expect("event를 가져오는데 실패했습니다.");

        Ok(SingleEventResponse {
            status: "success",
            data: EventData {
                event: event_result,
            },
        })
    }

    pub async fn update_event(
        db: &Database,
        id: &str,
        body: &UpdateEventSchema,
        user: &Uuid,
    ) -> Result<SingleEventResponse> {
        let event_result = base::update::<Self, UpdateEventSchema>(db, id, body, user)
            .await
            .expect("event 업데이트에 실패했습니다.");

        Ok(SingleEventResponse {
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
        db::MongoDB,
        models::chat::model::{ChatType, MsgModel, MsgType},
    };
    use dotenv::dotenv;
    use mongodb::options::UpdateOptions;
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
                    msg_type: MsgType::Ask,
                    content: "기술스택 토론 예정".to_string(),
                    created_at: Utc::now(),
                    booked: false,
                    chat: None,
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
        let body1 = CreateEventSchema {
            title: "Test Event".to_string(),
            start_date: None,
            due_at: None,
            location: None,
        };

        let body2 = CreateEventSchema {
            title: "Test Event2".to_string(),
            start_date: None,
            due_at: Some(Utc::now()),
            location: None,
        };

        let res = EventBMC::create_event(&db, &body1, &user_id).await;
        claim::assert_ok!(&res);
        let res = res.unwrap();
        claim::assert_matches!(res.status, "success");
        assert_eq!(res.data.event.title, body1.title);

        let res = EventBMC::create_event(&db, &body2, &user_id).await;
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

        let res = EventBMC::fetch_events(&db, limit, page, &user_id).await;
        claim::assert_ok!(&res);
        let res = res.unwrap();
        claim::assert_matches!(res.status, "success");
    }

    #[tokio::test]
    async fn test_get_event() {
        let db = setup().await;
        let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let event_id = "507f1f77bcf86cd799439013";

        let res = EventBMC::get_event(&db, event_id, &user_id).await;
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
        let body = UpdateEventSchema {
            title: Some("Updated Title".to_string()),
            complete: Some(true),
            start_date: None,
            due_at: None,
            location: None,
            chat_type: Some(ChatType::Task),
        };

        let res = EventBMC::update_event(&db, event_id, &body, &user_id).await;
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

        let res = EventBMC::delete_event(&db, event_id).await;
        claim::assert_ok!(&res);
    }
}
