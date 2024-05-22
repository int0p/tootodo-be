use crate::models::{
    category::model::{CategoryModel, PropertyType},
    chat::model::{ChatType, MsgModel},
};
use chrono::{DateTime, NaiveDate, Utc};
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub user: Uuid,
    pub title: String,
    pub start_date: Option<NaiveDate>,
    pub due_at: Option<DateTime<Utc>>,

    pub category: CategoryModel,
    pub properties: Vec<PropertyValue>,

    pub blocks: Vec<BlockModel>,

    pub subtasks: Vec<TaskModel>,
    pub parent_id: Option<ObjectId>,

    pub chat_type: ChatType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_msgs: Option<Vec<MsgModel>>,

    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PropertyValue {
    pub prop_id: ObjectId,
    pub prop_name: String,
    pub value: Option<PropertyValueData>,
    pub prop_type: PropertyType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum PropertyValueData {
    Multiple(Vec<String>),
    Single(String),
}

impl PropertyValue {
    pub fn new(
        prop_id: ObjectId,
        prop_name: String,
        prop_type: PropertyType,
        value: PropertyValueData,
    ) -> Result<Self, String> {
        let value =
            match (&prop_type, &value) {
                (PropertyType::MultiSelect, PropertyValueData::Multiple(_))
                | (PropertyType::SingleSelect, PropertyValueData::Multiple(_)) => Some(value),
                (PropertyType::MultiSelect, PropertyValueData::Single(_))
                | (PropertyType::SingleSelect, PropertyValueData::Single(_)) => {
                    return Err(
                        "MultiSelect or SingleSelect types must have Multiple(Vec<String>) value"
                            .to_string(),
                    )
                }
                (_, PropertyValueData::Single(_)) => Some(value),
                (_, PropertyValueData::Multiple(_)) => return Err(
                    "Only MultiSelect or SingleSelect types can have Multiple(Vec<String>) value"
                        .to_string(),
                ),
            };

        Ok(PropertyValue {
            prop_id,
            prop_name,
            prop_type,
            value,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub src_task_id: ObjectId,
    pub block_type: BlockType,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BlockType {
    Editor,
    Code,
    Drawing,
    Table,
}
