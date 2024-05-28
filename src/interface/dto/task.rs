pub mod req {
    use crate::domain::{
        sub::task_block::BlockModel,
        task::{PropertyValue, TaskModel},
        types::{BlockType, ChatType, PropertyType},
    };
    use chrono::{DateTime, NaiveDate, Utc};
    use mongodb::bson::{self, oid::ObjectId};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    // task
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateTaskReq {
        pub user: Uuid,
        pub title: String,
        pub category: ObjectId,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub subtasks: Option<Vec<TaskModel>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parent_id: Option<ObjectId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub blocks: Option<Vec<BlockModel>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub chat_type: Option<ChatType>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateTaskReq {
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
    pub struct CreateCategoryReq {
        pub user: Uuid,
        pub name: String,
        pub color: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateCategoryReq {
        pub user: Uuid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub color: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FilterCategoryReq {
        #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
        pub user: Uuid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    // property
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreatePropertyReq {
        pub user: Uuid,
        pub name: String,
        pub category_id: ObjectId,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<Vec<String>>,
        pub prop_type: PropertyType,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdatePropertyReq {
        pub user: Uuid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prop_type: Option<PropertyType>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FilterPropertyReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    // block
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateBlockReq {
        pub src_task_id: ObjectId,
        pub block_type: BlockType,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateBlockReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub src_task_id: Option<ObjectId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub block_type: Option<BlockType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub body: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FilterBlockReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub src_task_id: Option<ObjectId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub block_type: Option<BlockType>,
    }
}

pub mod res {
    use crate::domain::{
        sub::chat::MsgModel,
        sub::task_block::BlockModel,
        task::{PropertyValue, TaskModel},
        types::ChatType,
    };
    use chrono::{DateTime, NaiveDate, Utc};
    use mongodb::bson::oid::ObjectId;
    use serde::Serialize;
    use uuid::Uuid;

    #[allow(non_snake_case)]
    #[derive(Serialize, Debug)]
    pub struct TaskRes {
        pub id: String,
        pub user: Uuid,
        pub title: String,
        pub start_date: Option<NaiveDate>,
        pub due_at: Option<DateTime<Utc>>,

        pub category_id: ObjectId,
        pub category_color: String,
        pub category_name: String,
        pub proerties: Vec<PropertyValue>,

        pub blocks: Vec<BlockModel>,

        pub subtasks: Vec<TaskModel>,
        pub parent_id: Option<ObjectId>,

        pub chat_type: ChatType,
        pub chat_msgs: Option<Vec<MsgModel>>,

        pub createdAt: DateTime<Utc>,
        pub updatedAt: DateTime<Utc>,
    }

    #[derive(Serialize, Debug)]
    pub struct TaskData {
        pub task: TaskRes,
    }

    #[derive(Serialize, Debug)]
    pub struct SingleTaskRes {
        pub status: &'static str,
        pub data: TaskData,
    }

    #[derive(Serialize, Debug)]
    pub struct TaskListRes {
        pub status: &'static str,
        pub results: usize,
        pub tasks: Vec<TaskRes>,
    }
}
