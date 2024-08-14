use crate::domain::sub::chat::MsgModel;
use crate::infra::types::{ChatType, QueryFilterOptions};

use chrono::prelude::*;
use mongodb::bson::doc;
use mongodb::bson::{self, oid::ObjectId};
use mongodb::{bson::Document, Database};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::interface::dto::task::{
    req::{CreateTaskReq, TaskFetchOptions, UpdateTaskReq},
    res::{TaskData, TaskListRes, TaskRes, SingleTaskRes},
};

use crate::{
    domain::error::{Error::*, Result},
    domain::repo::base::{self, MongoRepo},
    infra::db::error::Error as DBError,
};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub user: Uuid,
    pub title: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,

    pub due_at: Option<DateTime<Local>>,

    pub progress_rate: u8,
    pub milestone: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<ChatType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_msgs: Option<Vec<MsgModel>>,

    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct TaskService;

impl MongoRepo for TaskService {
    const COLL_NAME: &'static str = "tasks";
    type Model = TaskModel;
    type ModelResponse = TaskRes;
    fn convert_doc_to_response(task: &TaskModel) -> Result<TaskRes> {
        Ok(TaskRes::from_model(task))
    }

    fn create_doc<CreateTaskReq: Serialize>(
        user: &Uuid,
        body: &CreateTaskReq,
    ) -> Result<Document> {
        let ser_data = bson::to_bson(body).map_err(|e| DB(DBError::MongoSerializeBsonError(e)))?;
        let doc = ser_data.as_document().unwrap();

        let datetime = Utc::now();

        let mut doc_with_date = doc! {
            "user": user,
            "milestone":false,
            "chat_type": "Task",
            "progress_rate": 0,
            "createdAt": datetime,
            "updatedAt": datetime,
        };
        doc_with_date.extend(doc.clone());
        Ok(doc_with_date)
    }
}

impl TaskService {
    pub async fn fetch_tasks(
        db: &Database,
        limit: i64,
        page: i64,
        start_date: &str,
        end_date: &str,
        user: &Uuid,
    ) -> Result<TaskListRes> {
        let mut find_filter = doc! {
            "user": user,
        };

        if !start_date.is_empty() && !end_date.is_empty() {
            find_filter.insert(
                "$and",
                vec![
                    doc! { "start_date": { "$lt": end_date } },
                    doc! { "end_date": { "$gt": start_date } },
                ],
            );
        }

        let filter_opts = QueryFilterOptions {
            find_filter: Some(find_filter),
            proj_opts: Some(TaskFetchOptions::build_projection()),
            limit,
            page,
        };

        tracing::info!("filter_opts: {:?}", filter_opts.find_filter);
        let tasks_results = base::fetch::<Self>(db, filter_opts, user)
            .await
            .expect("task 응답을 받아오지 못했습니다.");

        Ok(TaskListRes {
            status: "success",
            results: tasks_results.len(),
            tasks: tasks_results,
        })
    }

    pub async fn create_task(
        db: &Database,
        body: &CreateTaskReq,
        user: &Uuid,
    ) -> Result<SingleTaskRes> {
        tracing::info!("body: {:?}", body);
        let task_result = base::create::<Self, CreateTaskReq>(
            db,
            body,
            user,
            Some(vec!["start_date", "end_date"]),
        )
        .await
        .expect("task 생성에 실패했습니다.");

        Ok(SingleTaskRes {
            status: "success",
            data: TaskData {
                task: task_result,
            },
        })
    }

    pub async fn get_task(db: &Database, id: &str, user: &Uuid) -> Result<SingleTaskRes> {
        let task_result = base::get::<Self>(db, id, user)
            .await
            .expect("task를 가져오는데 실패했습니다.");

        Ok(SingleTaskRes {
            status: "success",
            data: TaskData {
                task: task_result,
            },
        })
    }

    pub async fn update_task(
        db: &Database,
        id: &str,
        body: &UpdateTaskReq,
        user: &Uuid,
    ) -> Result<SingleTaskRes> {
        let task_result = base::update::<Self, UpdateTaskReq>(db, id, body, user)
            .await
            .expect("task 업데이트에 실패했습니다.");

        Ok(SingleTaskRes {
            status: "success",
            data: TaskData {
                task: task_result,
            },
        })
    }

    pub async fn delete_task(db: &Database, id: &str) -> Result<()> {
        base::delete::<Self>(db, id).await
    }
}

