pub mod req {
    use chrono::{DateTime, Local, NaiveDate, Utc};
    use mongodb::bson::Document;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use crate::infra::types::ChatType;

    #[derive(Deserialize, Debug, Default)]
    pub struct TaskFilterOptions {
        pub page: Option<usize>,
        pub limit: Option<usize>,
        pub start_date: Option<NaiveDate>,
        pub end_date: Option<NaiveDate>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateTaskReq {
        pub title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parent_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_date: Option<NaiveDate>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_date: Option<NaiveDate>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub due_at: Option<DateTime<Local>>,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateTaskReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub milestone: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parent_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_date: Option<NaiveDate>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_date: Option<NaiveDate>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub due_at: Option<DateTime<Local>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub progress_rate: Option<u8>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub chat_type: Option<ChatType>,
    }

    #[allow(non_snake_case)]
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct TaskFetchOptions {
        #[serde(rename = "_id")]
        pub id: String,
        pub user: Uuid,
        pub title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parent_id: Option<String>,
        pub progress_rate: u8,
        pub milestone: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub start_date: Option<NaiveDate>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_date: Option<NaiveDate>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub due_at: Option<DateTime<Local>>,
        pub createdAt: DateTime<Utc>,
        pub updatedAt: DateTime<Utc>,
    }

    impl TaskFetchOptions {
        pub fn build_projection() -> Document {
            let mut projection = Document::new();

            let fields = vec![
                "_id",
                "user",
                "title",
                "parent_id",
                "progress_rate",
                "milestone",
                "start_date",
                "end_date",
                "due_at",
                "createdAt",
                "updatedAt",
            ];

            for field in fields {
                projection.insert(field, 1);
            }

            projection
        }
    }
}

pub mod res {
    use crate::domain::{task::TaskModel, sub::chat::MsgModel};
    use crate::infra::types::ChatType;
    use chrono::{DateTime, Local, NaiveDate, Utc};
    use serde::Serialize;
    use uuid::Uuid;

    #[allow(non_snake_case)]
    #[derive(Serialize, Debug)]
    pub struct TaskRes {
        pub id: String,
        pub user: Uuid,
        pub title: String,
        pub parent_id: Option<String>,
        pub progress_rate: u8,
        pub milestone: bool,
        pub chat_type: Option<ChatType>,
        pub chat_msgs: Option<Vec<MsgModel>>,
        pub start_date: Option<NaiveDate>,
        pub end_date: Option<NaiveDate>,
        pub due_at: Option<DateTime<Local>>,
        pub createdAt: DateTime<Utc>,
        pub updatedAt: DateTime<Utc>,
    }

    impl TaskRes {
        pub fn from_model(task: &TaskModel) -> Self {
            Self {
                id: task.id.to_hex(),
                user: task.user,
                title: task.title.to_owned(),
                parent_id: task.parent_id.map(|id| id.to_hex()),
                start_date: task.start_date.to_owned(),
                due_at: task.due_at.to_owned(),
                chat_type: task.chat_type.to_owned(),
                chat_msgs: task.chat_msgs.to_owned(),
                createdAt: task.createdAt,
                updatedAt: task.updatedAt,
                progress_rate: task.progress_rate,
                milestone: task.milestone,
                end_date: task.end_date.to_owned(),
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
