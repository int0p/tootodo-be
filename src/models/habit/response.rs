use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use super::model::StatusType;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct HabitResponse {
    pub id: String,
    pub user: Uuid,
    pub name: String,
    pub icon: String,
    pub status: StatusType,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct HabitData {
    pub habit: HabitResponse,
}

#[derive(Serialize, Debug)]
pub struct SingleHabitResponse {
    pub status: &'static str,
    pub data: HabitData,
}

#[derive(Serialize, Debug)]
pub struct HabitListResponse {
    pub status: &'static str,
    pub results: usize,
    pub habits: Vec<HabitResponse>,
}
