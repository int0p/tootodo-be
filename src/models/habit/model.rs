use chrono::prelude::*;
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HabitModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub user: Uuid,
    pub name: String,
    pub icon: String,
    pub status: StatusType,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum StatusType {
    InProgress,
    Archived,
}
