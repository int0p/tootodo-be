pub mod req {
    use chrono::{DateTime, NaiveDate, Utc};
    use serde::{Deserialize, Serialize};

    use crate::infra::types::ChatType;

    // Event
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateEventReq {
        pub title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_date: Option<NaiveDate>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub due_at: Option<DateTime<Utc>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub location: Option<String>,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateEventReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub complete: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_date: Option<NaiveDate>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub due_at: Option<DateTime<Utc>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub location: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub chat_type: Option<ChatType>,
    }
}

pub mod res {
    use crate::domain::{event::EventModel, sub::chat::MsgModel};
    use crate::infra::types::ChatType;
    use chrono::{DateTime, NaiveDate, Utc};
    use mongodb::bson::Document;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[allow(non_snake_case)]
    #[derive(Serialize, Debug)]
    pub struct EventRes {
        pub id: String,
        pub user: Uuid,
        pub title: String,
        pub complete: bool,
        pub chat_type: ChatType,
        pub chat_msgs: Option<Vec<MsgModel>>,
        pub start_date: Option<NaiveDate>,
        pub due_at: Option<DateTime<Utc>>,
        pub location: Option<String>,
        pub createdAt: DateTime<Utc>,
        pub updatedAt: DateTime<Utc>,
    }

    impl EventRes {
        pub fn from_model(event: &EventModel) -> Self {
            Self {
                id: event.id.to_hex(),
                user: event.user,
                title: event.title.to_owned(),
                complete: event.complete.to_owned(),
                start_date: event.start_date.to_owned(),
                due_at: event.due_at.to_owned(),
                location: event.location.to_owned(),
                chat_type: event.chat_type.to_owned(),
                chat_msgs: event.chat_msgs.to_owned(),
                createdAt: event.createdAt,
                updatedAt: event.updatedAt,
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

    #[allow(non_snake_case)]
    #[derive(Deserialize, Serialize, Debug)]
    pub struct EventFetchRes {
        pub id: String,
        pub user: Uuid,
        pub title: String,
        pub complete: bool,
        pub start_date: Option<NaiveDate>,
        pub due_at: Option<DateTime<Utc>>,
        pub location: Option<String>,
        pub createdAt: DateTime<Utc>,
        pub updatedAt: DateTime<Utc>,
    }

    impl EventFetchRes {
        pub fn build_projection() -> Document {
            let mut projection = Document::new();

            let fields = vec![
                "_id",
                "user",
                "title",
                "complete",
                "start_date",
                "due_at",
                "location",
                "createdAt",
                "updatedAt",
            ];

            for field in fields {
                projection.insert(field, 1);
            }

            projection
        }
    }
    #[derive(Serialize, Debug)]
    pub struct EventFetchedRes {
        pub status: &'static str,
        pub results: usize,
        pub events: Vec<EventFetchRes>,
    }
}
