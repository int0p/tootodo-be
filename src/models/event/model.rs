
use chrono::prelude::*;
use mongodb::bson::{self, oid::ObjectId,};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::chat::model::{ChatModel, ChatType, MsgModel};
use std::time::Duration;

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
