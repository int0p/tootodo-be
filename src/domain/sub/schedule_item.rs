use crate::domain::error::Result;
use crate::domain::repo::base_array::{self, MongoArrayRepo};
use crate::domain::repo::ElemInfo;
use crate::domain::schedule::ScheduleModel;
use crate::infra::types::ScheduleType;
use crate::interface::dto::sub::schedule_item::{req::*, res::*};
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

impl ElemInfo for ScheduledTask {
    const ARR_NAME: &'static str = "tasks";
    type UpdateReq = UpdateScheduledTaskReq;
    type CreateReq = CreateScheduledTaskReq;
    type Res = ScheduledTaskRes;

    fn convert_to_res(doc: &Self) -> Self::Res {
        ScheduledTaskRes::from_model(doc)
    }
}

impl ElemInfo for ScheduledEvent {
    const ARR_NAME: &'static str = "events";
    type UpdateReq = UpdateScheduledEventReq;
    type CreateReq = CreateScheduledEventReq;
    type Res = ScheduledEventRes;

    fn convert_to_res(doc: &Self) -> Self::Res {
        ScheduledEventRes::from_model(doc)
    }
}

impl ElemInfo for ScheduledHabit {
    const ARR_NAME: &'static str = "habits";
    type UpdateReq = UpdateScheduledHabitReq;
    type CreateReq = CreateScheduledHabitReq;
    type Res = ScheduledHabitRes;

    fn convert_to_res(doc: &Self) -> Self::Res {
        ScheduledHabitRes::from_model(doc)
    }
}

impl ElemInfo for ScheduledAt {
    const ARR_NAME: &'static str = "scheduled_times";
    type UpdateReq = UpdateScheduledAtReq;
    type CreateReq = CreateScheduledAtReq;
    type Res = ScheduledAtRes;

    fn convert_to_res(doc: &Self) -> Self::Res {
        ScheduledAtRes::from_model(doc)
    }
}

pub struct ScheduleItemService<Elem> {
    _phantom: std::marker::PhantomData<Elem>,
}

// schedule collection의 tasks, habits, events, scheduled_times 배열 필드에 각각의 element model을 CRUD하는 서비스
impl<Elem> MongoArrayRepo for ScheduleItemService<Elem>
where
    Elem: DeserializeOwned + Serialize + Unpin + Send + Sync + ElemInfo,
{
    type CollModel = ScheduleModel;
    type ElemModel = Elem;

    type UpdateElemReq = Elem::UpdateReq;
    type CreateElemReq = Elem::CreateReq;
    type ElemRes = Elem::Res;

    const COLL_NAME: &'static str = "schedules";
    const ARR_NAME: &'static str = Elem::ARR_NAME;

    fn convert_doc_to_response(doc: &Self::ElemModel) -> Result<Self::ElemRes> {
        Ok(Elem::convert_to_res(doc))
    }
}

impl<Elem> ScheduleItemService<Elem>
where
    Elem: DeserializeOwned + Serialize + Unpin + Send + Sync + ElemInfo,
{
    pub async fn get_elem(
        db: &Database,
        src_id: &str,
        elem_id: &str,
    ) -> Result<SingleScheduleItemRes<Elem::Res>> {
        let result = base_array::get_elem::<Self>(db, src_id, elem_id).await?;
        Ok(SingleScheduleItemRes {
            status: "success",
            data: ScheduleItemData { item: result },
        })
    }

    pub async fn add_elem(
        db: &Database,
        src_id: &str,
        new_elem: &Elem::CreateReq,
    ) -> Result<SingleScheduleItemRes<Elem::Res>> {
        let result = base_array::add_elem::<Self>(db, src_id, new_elem, Some("item_type")).await?;
        Ok(SingleScheduleItemRes {
            status: "success",
            data: ScheduleItemData { item: result },
        })
    }

    pub async fn fetch_elems(
        db: &Database,
        src_id: &str,
        limit: i64,
        page: i64,
    ) -> Result<ScheduleItemListRes<Elem::Res>> {
        let results = base_array::fetch_elems::<Self>(db, src_id, Some(limit), Some(page)).await?;
        Ok(ScheduleItemListRes {
            status: "success",
            results: results.len(),
            items: results,
        })
    }

    pub async fn update_elem(
        db: &Database,
        src_id: &str,
        elem_id: &str,
        new_elem: &Elem::UpdateReq,
    ) -> Result<SingleScheduleItemRes<Elem::Res>> {
        let result = base_array::update_elem::<Self>(db, src_id, elem_id, new_elem).await?;
        Ok(SingleScheduleItemRes {
            status: "success",
            data: ScheduleItemData { item: result },
        })
    }

    pub async fn remove_elem(db: &Database, src_id: &str, elem_id: &str) -> Result<()> {
        base_array::remove_elem::<Self>(db, src_id, elem_id).await?;
        Ok(())
    }
}
