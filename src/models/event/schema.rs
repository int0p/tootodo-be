use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::models::chat::model::ChatType;
#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

// Event
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateEventSchema {
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
pub struct UpdateEventSchema {
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
