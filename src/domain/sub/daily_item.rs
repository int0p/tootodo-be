use crate::interface::dto::sub::daily_item::req::*;
use crate::interface::dto::sub::daily_item::res::*;
use crate::{
    domain::daily::DailyModel,
    domain::error::{Result},
    domain::repo::base_array::{self, MongoArrayRepo},
    domain::repo::ElemInfo,
};
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{self, doc};
use mongodb::Database;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DailyTaskModel {
    pub task_id: ObjectId,
    pub title: String,
    pub done: bool,
    pub doneAt: Option<DateTime<Utc>>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DailyEventModel {
    pub event_id: ObjectId,
    pub title: String,
    pub done: bool,
    pub doneAt: Option<DateTime<Utc>>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DailyHabitModel {
    pub habit_id: ObjectId,
    pub icon: String,
    pub name: String,
    pub done: bool,
    pub doneAt: Option<DateTime<Utc>>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TimerResultModel {
    pub category_id: ObjectId,
    pub category_color: String,
    pub startAt: DateTime<Utc>,
    pub endAt: DateTime<Utc>,
    pub focus_time: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
}

pub struct DailyItemService<Elem> {
    _phantom: std::marker::PhantomData<Elem>,
}

impl ElemInfo for TimerResultModel {
    const ARR_NAME: &'static str = "timer_results";
    type UpdateReq = UpdateTimerResultReq;
    type CreateReq = CreateTimerResultReq;
    type Res = TimerResultRes;

    fn convert_to_res(doc: &Self) -> Self::Res {
        TimerResultRes::from_model(doc)
    }
}

impl ElemInfo for DailyTaskModel {
    const ARR_NAME: &'static str = "tasks";
    type UpdateReq = UpdateDailyTaskReq;
    type CreateReq = CreateDailyTaskReq;
    type Res = DailyTaskRes;

    fn convert_to_res(doc: &Self) -> Self::Res {
        DailyTaskRes::from_model(doc)
    }
}

impl ElemInfo for DailyEventModel {
    const ARR_NAME: &'static str = "events";
    type UpdateReq = UpdateDailyEventReq;
    type CreateReq = CreateDailyEventReq;
    type Res = DailyEventRes;

    fn convert_to_res(doc: &Self) -> Self::Res {
        DailyEventRes::from_model(doc)
    }
}

impl ElemInfo for DailyHabitModel {
    const ARR_NAME: &'static str = "habits";
    type UpdateReq = UpdateDailyHabitReq;
    type CreateReq = CreateDailyHabitReq;
    type Res = DailyHabitRes;

    fn convert_to_res(doc: &Self) -> Self::Res {
        DailyHabitRes::from_model(doc)
    }
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
        Ok(Elem::convert_to_res(doc))
    }
}

impl<Elem> DailyItemService<Elem>
where
    Elem: DeserializeOwned + Serialize + Unpin + Send + Sync + ElemInfo,
{
    pub async fn get_elem(
        db: &Database,
        src_id: &str,
        elem_id: &str,
    ) -> Result<SingleDailyItemRes<Elem::Res>> {
        let result = base_array::get_elem::<Self>(db, src_id, elem_id).await?;
        Ok(SingleDailyItemRes {
            status: "success",
            data: DailyItemData { item: result },
        })
    }

    pub async fn add_elem(
        db: &Database,
        src_id: &str,
        new_elem: &Elem::CreateReq,
    ) -> Result<SingleDailyItemRes<Elem::Res>> {
        let result = base_array::add_elem::<Self>(db, src_id, new_elem, Some("done")).await?;
        Ok(SingleDailyItemRes {
            status: "success",
            data: DailyItemData { item: result },
        })
    }

    pub async fn fetch_elems(
        db: &Database,
        src_id: &str,
        limit: i64,
        page: i64,
    ) -> Result<DailyItemListRes<Elem::Res>> {
        let results = base_array::fetch_elems::<Self>(db, src_id, Some(limit), Some(page)).await?;
        Ok(DailyItemListRes {
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
    ) -> Result<SingleDailyItemRes<Elem::Res>> {
        let result = base_array::update_elem::<Self>(db, src_id, elem_id, new_elem).await?;
        Ok(SingleDailyItemRes {
            status: "success",
            data: DailyItemData { item: result },
        })
    }

    pub async fn remove_elem(db: &Database, src_id: &str, elem_id: &str) -> Result<()> {
        base_array::remove_elem::<Self>(db, src_id, elem_id).await?;
        Ok(())
    }
}
