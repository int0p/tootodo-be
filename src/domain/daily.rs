use chrono::{DateTime, NaiveDate, Utc};
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::timer_result::TimerResultModel;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DailyModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub user: Uuid,
    pub date: NaiveDate,
    pub diary: String,
    pub feedback: i8,
    pub tasks: Vec<DailyTask>,
    pub events: Vec<DailyEvent>,
    pub habits: Vec<DailyHabit>,
    pub timer_results: Vec<TimerResultModel>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DailyTask {
    pub task_id: ObjectId,
    pub title: String,
    pub done: bool,
    pub doneAt: Option<DateTime<Utc>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DailyEvent {
    pub event_id: ObjectId,
    pub title: String,
    pub done: bool,
    pub doneAt: Option<DateTime<Utc>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DailyHabit {
    pub habit_id: ObjectId,
    pub name: String,
    pub done: bool,
    pub doneAt: Option<DateTime<Utc>>,
}
