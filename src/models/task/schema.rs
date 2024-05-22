use super::model::*;
use crate::models::{
    category::model::{PropertyModel, PropertyType},
    chat::model::ChatModel,
};
use chrono::{DateTime, NaiveDate, Utc};
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// task
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTaskSchema {
    pub user: Uuid,
    pub title: String,
    pub display: bool,
    pub category: ObjectId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtask: Option<Vec<TaskModel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_blocks: Option<Vec<BlockModel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat: Option<ChatModel>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTaskSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<PropertyValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtasks: Option<Vec<TaskModel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<BlockModel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<ChatType>,
}

// category
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCategorySchema {
    pub user: Uuid,
    pub name: String,
    pub color: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCategorySchema {
    pub user: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilterCategorySchema {
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub user: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

// property
#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePropertySchema {
    pub user: Uuid,
    pub name: String,
    pub category_id: ObjectId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
    pub prop_type: PropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePropertySchema {
    pub user: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prop_type: Option<PropertyType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilterPropertySchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

// block
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateBlockSchema {
    pub src_task_id: ObjectId,
    pub block_type: BlockType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateBlockSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_task_id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_type: Option<BlockType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilterBlockSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_task_id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_type: Option<BlockType>,
}
