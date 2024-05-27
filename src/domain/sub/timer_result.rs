use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use serde::{Deserialize, Serialize};

use super::{
    daily::DailyModel,
    repo::base_array::{self, MongoArrayRepo},
};
use crate::{domain::error::Result, interface::dto::daily::req::UpdateTimerResultReq};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TimerResultModel {
    pub category_id: ObjectId,
    pub category_color: String,
    pub startAt: DateTime<Utc>,
    pub endAt: DateTime<Utc>,
    pub focus_time: String,
}

pub struct TimerResultService;

impl MongoArrayRepo for TimerResultService {
    type CollModel = DailyModel;
    type ElemModel = TimerResultModel;
    type UpdateElemReq = UpdateTimerResultReq;
    const COLL_NAME: &'static str = "daily";
    const ARR_NAME: &'static str = "timer_results";
}

impl TimerResultService {
    pub async fn get_timer_result(
        db: &Database,
        daily_id: &str,
        prop_id: &str,
    ) -> Result<TimerResultModel> {
        let doc = base_array::get_elem::<TimerResultService>(db, daily_id, prop_id).await?;
        Ok(doc)
    }

    pub async fn add_timer_result(
        db: &Database,
        daily_id: &str,
        new_prop: &TimerResultModel,
    ) -> Result<Vec<TimerResultModel>> {
        let doc: DailyModel =
            base_array::add_elem::<TimerResultService>(db, daily_id, new_prop).await?;
        Ok(doc.timer_results)
    }

    pub async fn fetch_timer_results(
        db: &Database,
        daily_id: &str,
    ) -> Result<Vec<TimerResultModel>> {
        let doc: DailyModel = base_array::fetch_elems::<TimerResultService>(db, daily_id).await?;
        Ok(doc.timer_results)
    }

    pub async fn update_timer_result(
        db: &Database,
        daily_id: &str,
        prop_id: &str,
        new_prop: &UpdateTimerResultReq,
    ) -> Result<Vec<TimerResultModel>> {
        let doc: DailyModel =
            base_array::update_elem::<TimerResultService>(db, daily_id, prop_id, new_prop).await?;
        Ok(doc.timer_results)
    }

    pub async fn remove_timer_result(
        db: &Database,
        daily_id: &str,
        prop_id: &str,
    ) -> Result<Vec<TimerResultModel>> {
        let doc: DailyModel =
            base_array::remove_elem::<TimerResultService>(db, daily_id, prop_id).await?;
        Ok(doc.timer_results)
    }
}
