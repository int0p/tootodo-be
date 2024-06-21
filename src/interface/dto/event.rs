pub mod req {
    use mongodb::bson::Document;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;
    use chrono::{DateTime, NaiveDate, Utc,Local};

    use crate::infra::types::ChatType;

    #[derive(Deserialize, Debug, Default)]
    pub struct EventFilterOptions {
        pub page: Option<usize>,
        pub limit: Option<usize>,
        pub start_date: Option<NaiveDate>,
        pub end_date: Option<NaiveDate>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateEventReq {
        pub title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_date: Option<NaiveDate>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_date: Option<NaiveDate>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub due_at: Option<DateTime<Local>>,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateEventReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub milestone: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_date: Option<NaiveDate>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_date: Option<NaiveDate>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub due_at: Option<DateTime<Local>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub progressRate: Option<f32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub chat_type: Option<ChatType>,
    }

    #[allow(non_snake_case)]
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct EventFetchOptions {
        #[serde(rename = "_id")]
        pub id: String,
        pub user: Uuid,
        pub title: String,
        pub progressRate: f32,
        pub milestone: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_date: Option<NaiveDate>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_date: Option<NaiveDate>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub due_at: Option<DateTime<Local>>,
        pub createdAt: DateTime<Utc>,
        pub updatedAt: DateTime<Utc>,
    }

    impl EventFetchOptions {
        pub fn build_projection() -> Document {
            let mut projection = Document::new();

            let fields = vec![
                "_id",
                "user",
                "title",
                "progressRate",
                "milestone",
                "start_date",
                "end_date",
                "due_at",
                "createdAt",
                "updatedAt",
            ];

            for field in fields {
                projection.insert(field, 1);
            }

            projection
        }
    }
}

pub mod res {
    use crate::domain::{event::EventModel, sub::chat::MsgModel};
    use crate::infra::types::ChatType;
    use chrono::{DateTime, NaiveDate, Utc,Local};
    use serde::Serialize;
    use uuid::Uuid;

    #[allow(non_snake_case)]
    #[derive(Serialize, Debug)]
    pub struct EventRes {
        pub id: String,
        pub user: Uuid,
        pub title: String,
        pub progressRate: f32,
        pub milestone: bool,
        pub chat_type: Option<ChatType>,
        pub chat_msgs: Option<Vec<MsgModel>>,
        pub start_date: Option<NaiveDate>,
        pub end_date: Option<NaiveDate>,
        pub due_at: Option<DateTime<Local>>,
        pub createdAt: DateTime<Utc>,
        pub updatedAt: DateTime<Utc>,
    }

    impl EventRes {
        pub fn from_model(event: &EventModel) -> Self {
            Self {
                id: event.id.to_hex(),
                user: event.user,
                title: event.title.to_owned(),
                start_date: event.start_date.to_owned(),
                due_at: event.due_at.to_owned(),
                chat_type: event.chat_type.to_owned(),
                chat_msgs: event.chat_msgs.to_owned(),
                createdAt: event.createdAt,
                updatedAt: event.updatedAt,
                progressRate: event.progressRate,
                milestone: event.milestone,
                end_date: event.end_date.to_owned(),
            }
        }
    }

    #[derive(Serialize, Debug)]
    pub struct EventData {
        pub event: EventRes,
    }

    #[derive(Serialize, Debug)]
    pub struct SingleEventRes {
        pub status: &'static str,
        pub data: EventData,
    }

    #[derive(Serialize, Debug)]
    pub struct EventListRes {
        pub status: &'static str,
        pub results: usize,
        pub events: Vec<EventRes>,
    }
}
