use super::{
    model::HabitModel,
    response::{HabitData, HabitListResponse, HabitResponse, SingleHabitResponse},
    schema::{CreateHabitSchema, UpdateHabitSchema},
};
use crate::{
    db::error::Error as DBError,
    models::base::{self, MongoBMC},
    models::error::{Error::*, Result},
};
use chrono::prelude::*;
use mongodb::bson;
use mongodb::bson::doc;
use mongodb::{bson::Document, Database};
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct HabitBMC;

impl MongoBMC for HabitBMC {
    const COLL_NAME: &'static str = "habits";
    const DOC_COLL_NAME: &'static str = "habits";
    type Model = HabitModel;
    type ModelResponse = HabitResponse;

    fn convert_doc_to_response(habit: &HabitModel) -> Result<HabitResponse> {
        let habit_response = HabitResponse {
            user: habit.user,
            id: habit.id.to_hex(),
            name: habit.name.to_owned(),
            icon: habit.icon.to_owned(),
            status: habit.status.to_owned(),
            createdAt: habit.createdAt,
            updatedAt: habit.updatedAt,
        };
        Ok(habit_response)
    }

    fn create_doc<CreateHabitSchema: Serialize>(
        user: &Uuid,
        body: &CreateHabitSchema,
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

impl HabitBMC {
    //mongodb에서 habit를 가져옴.
    pub async fn fetch_habits(
        db: &Database,
        limit: i64,
        page: i64,
        user: &Uuid,
    ) -> Result<HabitListResponse> {
        let habits_result = base::fetch::<Self>(db, limit, page, user)
            .await
            .expect("habit 응답을 받아오지 못했습니다.");

        Ok(HabitListResponse {
            status: "success",
            results: habits_result.len(),
            habits: habits_result,
        })
    }

    pub async fn create_habit(
        db: &Database,
        body: &CreateHabitSchema,
        user: &Uuid,
    ) -> Result<SingleHabitResponse> {
        let habit_result = base::create::<Self, CreateHabitSchema>(db, body, user)
            .await
            .expect("habit 생성에 실패했습니다.");

        Ok(SingleHabitResponse {
            status: "success",
            data: HabitData {
                habit: habit_result,
            },
        })
    }

    pub async fn get_habit(db: &Database, id: &str, user: &Uuid) -> Result<SingleHabitResponse> {
        let habit_result = base::get::<Self>(db, id, user)
            .await
            .expect("habit를 가져오는데 실패했습니다.");

        Ok(SingleHabitResponse {
            status: "success",
            data: HabitData {
                habit: habit_result,
            },
        })
    }

    pub async fn update_habit(
        db: &Database,
        id: &str,
        body: &UpdateHabitSchema,
        user: &Uuid,
    ) -> Result<SingleHabitResponse> {
        let habit_result = base::update::<Self, UpdateHabitSchema>(db, id, body, user)
            .await
            .expect("habit 업데이트에 실패했습니다.");

        Ok(SingleHabitResponse {
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
