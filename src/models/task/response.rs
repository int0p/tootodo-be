use super::model::*;
use crate::models::{
    category::model::CategoryModel,
    chat::model::{ChatType, MsgModel},
};
use chrono::{DateTime, NaiveDate, Utc};
use mongodb::bson::oid::ObjectId;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct TaskResponse {
    pub id: String,
    pub user: Uuid,
    pub title: String,

    pub chat_type: ChatType,
    pub chat_msgs: Option<Vec<MsgModel>>,

    pub start_date: Option<NaiveDate>,
    pub due_at: Option<DateTime<Utc>>,

    pub category: CategoryModel,
    pub proerties: Option<Vec<PropertyValue>>,

    pub subtasks: Option<Vec<TaskModel>>,
    pub parent_id: Option<ObjectId>,

    pub blocks: Option<Vec<BlockModel>>,

    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct TaskData {
    pub task: TaskResponse,
}

#[derive(Serialize, Debug)]
pub struct SingleTaskResponse {
    pub status: &'static str,
    pub data: TaskData,
}

#[derive(Serialize, Debug)]
pub struct TaskListResponse {
    pub status: &'static str,
    pub results: usize,
    pub tasks: Vec<TaskResponse>,
}
