use crate::models::chat::model::{ChatType, MsgModel};
use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct EventResponse {
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

#[derive(Serialize, Debug)]
pub struct EventData {
    pub event: EventResponse,
}

#[derive(Serialize, Debug)]
pub struct SingleEventResponse {
    pub status: &'static str,
    pub data: EventData,
}

#[derive(Serialize, Debug)]
pub struct EventListResponse {
    pub status: &'static str,
    pub results: usize,
    pub events: Vec<EventResponse>,
}
