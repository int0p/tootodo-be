use chrono::{DateTime, NaiveDate, Utc};
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::sub::daily_item::{DailyEvent, DailyHabit, DailyTask, TimerResultModel};

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
