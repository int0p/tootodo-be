use chrono::{DateTime, Utc};
use mongodb::bson::doc;
use mongodb::bson::{self, oid::ObjectId};
use mongodb::{bson::Document, Database};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infra::types::QueryFilterOptions;
use crate::interface::dto::habit::{
    req::{CreateHabitReq, UpdateHabitReq},
    res::{HabitData, HabitListRes, HabitRes, SingleHabitRes},
};
use crate::{
    domain::error::{Error::*, Result},
    domain::repo::base::{self, MongoRepo},
    infra::db::error::Error as DBError,
    infra::types::StatusType,
};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HabitModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub user: Uuid,
    pub name: String,
    pub icon: String,
    pub status: StatusType,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct HabitService;

impl MongoRepo for HabitService {
    const COLL_NAME: &'static str = "habits";
    type Model = HabitModel;
    type ModelResponse = HabitRes;
    fn convert_doc_to_response(habit: &HabitModel) -> Result<HabitRes> {
        Ok(HabitRes::from_model(habit))
    }

    fn create_doc<CreateHabitReq: Serialize>(
        user: &Uuid,
        body: &CreateHabitReq,
    ) -> Result<Document> {
        let serialized_data =
            bson::to_bson(body).map_err(|e| DB(DBError::MongoSerializeBsonError(e)))?;
        let document = serialized_data.as_document().unwrap();
        let datetime = Utc::now();
        let mut doc_with_dates = doc! {
            "user": user,
            "status": "InProgress",
            "createdAt": datetime,
            "updatedAt": datetime,
        };
        doc_with_dates.extend(document.clone());
        Ok(doc_with_dates)
    }
}

impl HabitService {
    //mongodb에서 habit를 가져옴.
    pub async fn fetch_habits(
        db: &Database,
        limit: i64,
        page: i64,
        user: &Uuid,
    ) -> Result<HabitListRes> {
        let filter_opts = QueryFilterOptions {
            find_filter: None,
            proj_opts: None,
            limit,
            page,
        };
        let habits_result = base::fetch::<Self>(db, filter_opts, user)
            .await
            .expect("habit 응답을 받아오지 못했습니다.");

        Ok(HabitListRes {
            status: "success",
            results: habits_result.len(),
            habits: habits_result,
        })
    }

    pub async fn create_habit(
        db: &Database,
        body: &CreateHabitReq,
        user: &Uuid,
    ) -> Result<SingleHabitRes> {
        let habit_result = base::create::<Self, CreateHabitReq>(db, body, user, None)
            .await
            .expect("habit 생성에 실패했습니다.");

        Ok(SingleHabitRes {
            status: "success",
            data: HabitData {
                habit: habit_result,
            },
        })
    }

    pub async fn get_habit(db: &Database, id: &str, user: &Uuid) -> Result<SingleHabitRes> {
        let habit_result = base::get::<Self>(db, id, user)
            .await
            .expect("habit를 가져오는데 실패했습니다.");

        Ok(SingleHabitRes {
            status: "success",
            data: HabitData {
                habit: habit_result,
            },
        })
    }

    pub async fn update_habit(
        db: &Database,
        id: &str,
        body: &UpdateHabitReq,
        user: &Uuid,
    ) -> Result<SingleHabitRes> {
        let habit_result = base::update::<Self, UpdateHabitReq>(db, id, body, user)
            .await
            .expect("habit 업데이트에 실패했습니다.");

        Ok(SingleHabitRes {
            status: "success",
            data: HabitData {
                habit: habit_result,
            },
        })
    }

    pub async fn delete_habit(db: &Database, id: &str) -> Result<()> {
        base::delete::<Self>(db, id).await
    }
}
