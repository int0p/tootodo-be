use std::any::TypeId;

use crate::domain::error::Result;
use crate::domain::repo::base_array::{self, MongoArrayRepo};
use crate::domain::schedule::ScheduleModel;
use crate::domain::types::ScheduleType;
use crate::interface::dto::schedule::req::*;
use chrono::{DateTime, NaiveDate, NaiveTime, Utc, Weekday};
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ScheduledTask {
    pub task_id: ObjectId,
    pub title: String,
    pub category_id: ObjectId,
    pub category_color: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ScheduledEvent {
    pub event_id: ObjectId,
    pub title: String,
    pub start_date: Option<NaiveDate>,
    pub due_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ScheduledHabit {
    pub habit_id: ObjectId,
    pub name: String,
    pub icon: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ScheduledAt {
    pub item_id: ObjectId,
    pub item_type: ScheduleType,
    pub weekday: Weekday, //Mon=0
    pub startAt: Option<NaiveTime>,
    pub endAt: Option<NaiveTime>,
}

pub trait ElemInfo {
    const ARR_NAME: &'static str;
    type UpdateReq: Serialize;
    type CreateReq: Serialize;
}

impl ElemInfo for ScheduledTask {
    const ARR_NAME: &'static str = "tasks";
    type UpdateReq = UpdateScheduledTaskReq;
    type CreateReq = CreateScheduledTaskReq;
}

impl ElemInfo for ScheduledEvent {
    const ARR_NAME: &'static str = "events";
    type UpdateReq = UpdateScheduledEventReq;
    type CreateReq = CreateScheduledEventReq;
}

impl ElemInfo for ScheduledHabit {
    const ARR_NAME: &'static str = "habits";
    type UpdateReq = UpdateScheduledHabitReq;
    type CreateReq = CreateScheduledHabitReq;
}

impl ElemInfo for ScheduledAt {
    const ARR_NAME: &'static str = "scheduled_times";
    type UpdateReq = UpdateScheduledAtReq;
    type CreateReq = CreateScheduledAtReq;
}

pub struct ScheduleItemService<Elem> {
    _phantom: std::marker::PhantomData<Elem>,
}
// daily collection의 tasks, habits, events, timer_results 배열필드에 각각의 element model을 CRUD하는 서비스
impl<Elem> MongoArrayRepo for ScheduleItemService<Elem>
where
    Elem: DeserializeOwned + Serialize + Unpin + Send + Sync + ElemInfo,
{
    type CollModel = ScheduleModel;
    type ElemModel = Elem;

    type UpdateElemReq = Elem::UpdateReq;
    type CreateElemReq = Elem::CreateReq;

    const COLL_NAME: &'static str = "schedule";

    const ARR_NAME: &'static str = Elem::ARR_NAME;
}

impl<Elem> ScheduleItemService<Elem>
where
    Elem: DeserializeOwned + Serialize + Unpin + Send + Sync + ElemInfo,
{
    pub async fn get_elem(db: &Database, src_id: &str, elem_id: &str) -> Result<Elem> {
        Ok(base_array::get_elem::<Self>(db, src_id, elem_id).await?)
    }

    pub async fn add_elem(
        db: &Database,
        src_id: &str,
        new_elem: &Elem::CreateReq,
    ) -> Result<Vec<Elem>> {
        // 배열의 원소에 index에 해당하는 필드가 없을 경우, 해당 index를 무시하므로 ScheduledAt 외의 원소에 아무 영항 안 줌.
        Ok(base_array::add_elem::<Self>(db, src_id, new_elem, Some("item_type")).await?)

        // Elem이 static lifetime이어야 가능.
        // if TypeId::of::<Elem>() == TypeId::of::<ScheduledAt>() {
        //     Ok(base_array::add_elem::<Self>(db, src_id, new_elem, Some("item_type")).await?)
        // } else {
        //     Ok(base_array::add_elem::<Self>(db, src_id, new_elem, None).await?)
        // }
    }

    pub async fn fetch_elems(db: &Database, src_id: &str) -> Result<Vec<Elem>> {
        Ok(base_array::fetch_elems::<Self>(db, src_id).await?)
    }

    pub async fn remove_elem(db: &Database, src_id: &str, elem_id: &str) -> Result<Vec<Elem>> {
        Ok(base_array::remove_elem::<Self>(db, src_id, elem_id).await?)
    }

    pub async fn update_elem(
        db: &Database,
        src_id: &str,
        elem_id: &str,
        new_elem: &Elem::UpdateReq,
    ) -> Result<Vec<Elem>> {
        Ok(base_array::update_elem::<Self>(db, src_id, elem_id, new_elem).await?)
    }
}
