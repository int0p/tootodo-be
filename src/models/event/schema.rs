
use std::time::Duration;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::model::*;
use crate::models::chat::model::ChatModel;

// Event
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateEventSchema {
    pub user: Uuid,
    pub title: String,
    pub complete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Duration>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateEventSchema {
    pub user: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Duration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat: Option<ChatModel>,
}

