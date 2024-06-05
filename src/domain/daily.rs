use chrono::{DateTime, NaiveDate, Utc};
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::sub::daily_item::{DailyEventModel, DailyHabitModel, DailyTaskModel, TimerResultModel};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DailyModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub user: Uuid,
    pub date: NaiveDate,
    pub diary: String,
    pub rating: i8,
    pub tasks: Vec<DailyTaskModel>,
    pub events: Vec<DailyEventModel>,
    pub habits: Vec<DailyHabitModel>,
    pub timer_results: Vec<TimerResultModel>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>,
}
