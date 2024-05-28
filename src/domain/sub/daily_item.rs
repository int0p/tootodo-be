use crate::domain::daily::DailyModel;
use crate::domain::error::Result;
use crate::domain::repo::base_array::{self, MongoArrayRepo};
use crate::interface::dto::daily::req::*;
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DailyTask {
    task_id: ObjectId,
    title: String,
    done: bool,
    doneAt: Option<DateTime<Utc>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DailyEvent {
    event_id: ObjectId,
    title: String,
    done: bool,
    doneAt: Option<DateTime<Utc>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DailyHabit {
    habit_id: ObjectId,
    icon: String,
    name: String,
    done: bool,
    doneAt: Option<DateTime<Utc>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TimerResultModel {
    category_id: ObjectId,
    category_color: String,
    startAt: DateTime<Utc>,
    endAt: DateTime<Utc>,
    focus_time: String,
}

pub struct DailyItemService<Elem> {
    _phantom: std::marker::PhantomData<Elem>,
}

pub trait ElemInfo {
    const ARR_NAME: &'static str;
    type UpdateReq: Serialize;
    type CreateReq: Serialize;
    type Res: DeserializeOwned + Send + Sync + Serialize;
}

impl ElemInfo for TimerResultModel {
    const ARR_NAME: &'static str = "timer_results";
    type UpdateReq = UpdateTimerResultReq;
    type CreateReq = CreateTimerResultReq;
    type Res = TimerResultRes;
}

impl ElemInfo for DailyTask {
    const ARR_NAME: &'static str = "tasks";
    type UpdateReq = UpdateDailyTaskReq;
    type CreateReq = CreateDailyTaskReq;
    type Res = DailyTaskRes;
}

impl ElemInfo for DailyEvent {
    const ARR_NAME: &'static str = "events";
    type UpdateReq = UpdateDailyEventReq;
    type CreateReq = CreateDailyEventReq;
    type Res = DailyEventRes;
}

impl ElemInfo for DailyHabit {
    const ARR_NAME: &'static str = "habits";
    type UpdateReq = UpdateDailyHabitReq;
    type CreateReq = CreateDailyHabitReq;
    type Res = DailyHabitRes;
}

// daily collection의 tasks, habits, events, timer_results 배열필드에 각각의 element model을 CRUD하는 서비스
impl<Elem> MongoArrayRepo for DailyItemService<Elem>
where
    Elem: DeserializeOwned + Serialize + Unpin + Send + Sync + ElemInfo,
{
    type CollModel = DailyModel;
    type ElemModel = Elem;

    type UpdateElemReq = Elem::UpdateReq;
    type CreateElemReq = Elem::CreateReq;
    type ElemRes = Elem::Res;

    const COLL_NAME: &'static str = "daily";

    const ARR_NAME: &'static str = Elem::ARR_NAME;

    fn convert_doc_to_response(doc: &Self::ElemModel) -> Result<Self::ElemRes> {
        let res: Self::ElemRes = bson::ser::from_bson(bson::ser::to_document(doc)?).unwrap();
        Ok(res)
    }
}

impl<Elem> DailyItemService<Elem>
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
        // 배열의 원소에 index에 해당하는 필드가 없을 경우, 해당 index를 무시하므로 timerResult에 아무 영항 안 줌.
        Ok(base_array::add_elem::<Self>(db, src_id, new_elem, Some("done")).await?)
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
