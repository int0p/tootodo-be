pub mod req {
    use crate::domain::{
        sub::{task_block::BlockModel, task_propV::PropValueModel},
        task::TaskModel,
        types::{ChatType, PropertyType},
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
        pub category_id: ObjectId,
        pub category_color: String,
        pub category_name: String,
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
        pub category_id: Option<ObjectId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub category_color: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub category_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_date: Option<NaiveDate>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub due_at: Option<DateTime<Utc>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub parent_id: Option<ObjectId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub subtasks: Option<Vec<TaskModel>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub chat_type: Option<ChatType>,
    }

    impl Default for UpdateTaskReq {
        fn default() -> Self {
            Self {
                title: None,
                category_id: None,
                category_color: None,
                category_name: None,
                start_date: None,
                due_at: None,
                parent_id: None,
                subtasks: None,
                chat_type: None,
            }
        }
    }
}

pub mod res {
    use crate::domain::{
        sub::{chat::MsgModel, task_block::BlockModel, task_propV::PropValueModel},
        task::TaskModel,
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
        pub prop_values: Vec<PropValueModel>,

        pub blocks: Vec<BlockModel>,

        pub subtasks: Vec<TaskModel>,
        pub parent_id: Option<ObjectId>,

        pub chat_type: ChatType,
        pub chat_msgs: Option<Vec<MsgModel>>,

        pub createdAt: DateTime<Utc>,
        pub updatedAt: DateTime<Utc>,
    }

    impl TaskRes {
        pub fn from_model(task: &TaskModel) -> Self {
            Self {
                id: task.id.to_hex(),
                user: task.user,
                title: task.title.to_owned(),
                start_date: task.start_date.to_owned(),
                due_at: task.due_at.to_owned(),
                category_id: task.category_id.to_owned(),
                category_color: task.category_color.to_owned(),
                category_name: task.category_name.to_owned(),
                prop_values: task.prop_values.clone(),
                blocks: task.blocks.to_owned(),
                subtasks: task.subtasks.to_owned(),
                parent_id: task.parent_id.to_owned(),
                chat_type: task.chat_type.to_owned(),
                chat_msgs: task.chat_msgs.to_owned(),
                createdAt: task.createdAt,
                updatedAt: task.updatedAt,
            }
        }
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
