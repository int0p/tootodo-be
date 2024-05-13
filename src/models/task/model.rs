use chrono::prelude::*;
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::chat::model::ChatModel;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub user: Uuid,
    pub title: String,
    pub display: bool,
    pub category: Option<CategoryModel>,
    pub proerties: Vec<Property>,
    pub subtask: Vec<TaskModel>,
    pub blocks: Vec<BlockModel>,
    pub chat: ChatModel,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryModel{
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub user: Uuid,
    pub name:String,
    pub color:String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PropertyModel{
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub user: Uuid,
    pub category_id: ObjectId,
    pub name:String,
    pub value:Option<Vec<String>>,
    pub prop_type:PropertyType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Property{
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub prop_id:ObjectId,
    pub prop_name:String,
    pub value:Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PropertyType{
    Status,
    Duration,
    MultiSelect,
    SingleSelect,    
    Text,
    Number,
    DateTime,
    File,
    Image,
    Link,
    Email,
    Phone,
    Location,    
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockModel{
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub src_task_id: ObjectId,
    pub block_type:BlockType,
    pub body:String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BlockType {
    Editor,
    Code,
    Drawing,
    Table,
}