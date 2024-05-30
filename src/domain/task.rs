use chrono::{DateTime, NaiveDate, Utc};
use futures::TryStreamExt;
use serde_json::Value;
use std::str::FromStr;

use super::sub::{
    chat::MsgModel,
    property::PropertyService,
    task_block::BlockModel,
    task_propV::{PropValueModel, PropValueService},
};

use crate::{
    domain::{
        error::{Error::*, Result},
        repo::base::{self, MongoRepo},
    },
    infra::{
        db::error::Error as DBError,
        types::{ChatType, FetchFilterOptions, PropertyType},
    },
    interface::dto::{
        sub::task_propV::{req::UpdatePropValueReq, res::PropValueListRes},
        task::{
            req::{CreateTaskReq, UpdateTaskReq},
            res::{SingleTaskRes, TaskData, TaskFetchRes, TaskFetchedRes, TaskListRes, TaskRes},
        },
    },
};

use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    options::AggregateOptions,
    Database,
};

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

    pub category_id: ObjectId,
    pub category_color: String,
    pub category_name: String,

    pub prop_values: Vec<PropValueModel>,

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

impl TaskModel {
    pub fn new_subtask(original_task: &Self) -> Self {
        Self {
            id: ObjectId::new(),
            user: original_task.user,
            title: "New Subtask".to_string(),
            start_date: Some(Utc::now().date_naive()),
            due_at: None,
            category_id: original_task.category_id,
            category_color: original_task.category_color.clone(),
            category_name: original_task.category_name.clone(),
            prop_values: vec![],
            blocks: vec![BlockModel::new_from(original_task.id)],
            subtasks: vec![],
            parent_id: Some(original_task.id),
            chat_type: original_task.chat_type.clone(),
            chat_msgs: None,
            createdAt: Utc::now(),
            updatedAt: Utc::now(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TaskService;

impl MongoRepo for TaskService {
    const COLL_NAME: &'static str = "tasks";
    type Model = TaskModel;
    type ModelResponse = TaskRes;
    type ModelFetchResponse = TaskFetchRes;

    fn convert_doc_to_response(task: &TaskModel) -> Result<TaskRes> {
        Ok(TaskRes::from_model(task))
    }

    fn create_doc<CreateTaskReq: Serialize>(user: &Uuid, body: &CreateTaskReq) -> Result<Document> {
        let serialized_data =
            bson::to_bson(body).map_err(|e| DB(DBError::MongoSerializeBsonError(e)))?;
        let document = serialized_data.as_document().unwrap();

        let datetime = Utc::now();

        let mut doc_with_dates = doc! {
            "user": user,
            "complete":false,
            "chat_type": "Task",
            "createdAt": datetime,
            "updatedAt": datetime,
        };
        doc_with_dates.extend(document.clone());
        Ok(doc_with_dates)
    }
}

impl TaskService {
    pub async fn fetch_tasks(
        db: &Database,
        limit: i64,
        page: i64,
        user: &Uuid,
    ) -> Result<TaskFetchedRes> {
        let filter_opts = FetchFilterOptions {
            find_filter: Some(doc! {"user": user}),
            proj_opts: Some(TaskFetchRes::build_projection()),
            limit,
            page,
        };
        let tasks_result = base::fetch::<Self>(db, filter_opts, user)
            .await
            .expect("task 응답을 받아오지 못했습니다.");

        Ok(TaskFetchedRes {
            status: "success",
            results: tasks_result.len(),
            tasks: tasks_result,
        })
    }

    pub async fn fetch_tasks_by_category(
        db: &Database,
        limit: i64,
        page: i64,
        category_id: &str,
        user: &Uuid,
    ) -> Result<TaskFetchedRes> {
        let filter_opts = FetchFilterOptions {
            find_filter: Some(doc! {"user": user,"category_id":category_id}),
            proj_opts: Some(TaskFetchRes::build_projection()),
            limit,
            page,
        };

        let tasks_result = base::fetch::<Self>(db, filter_opts, user)
            .await
            .expect("task 응답을 받아오지 못했습니다.");

        Ok(TaskFetchedRes {
            status: "success",
            results: tasks_result.len(),
            tasks: tasks_result,
        })
    }

    pub async fn create_task(
        db: &Database,
        body: &mut CreateTaskReq,
        user: &Uuid,
    ) -> Result<SingleTaskRes> {
        if let Ok(prop_values) = Self::get_prop_values(db, &body.category_id).await {
            body.prop_values = Some(prop_values);
        }
        let task_result = base::create::<Self, CreateTaskReq>(db, body, user)
            .await
            .expect("task 생성에 실패했습니다.");

        Ok(SingleTaskRes {
            status: "success",
            data: TaskData { task: task_result },
        })
    }

    pub async fn get_task(db: &Database, id: &str, user: &Uuid) -> Result<SingleTaskRes> {
        let task_result = base::get::<Self>(db, id, user)
            .await
            .expect("task를 가져오는데 실패했습니다.");

        Ok(SingleTaskRes {
            status: "success",
            data: TaskData { task: task_result },
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
            data: TaskData { task: task_result },
        })
    }

    pub async fn delete_task(db: &Database, id: &str) -> Result<()> {
        base::delete::<Self>(db, id).await
    }

    // category 갱신에 따른 task 갱신
    pub async fn update_tasks_for_category_change(
        db: &Database,
        category_id: &str,
        new_category_name: &str,
        new_category_color: &str,
        user: &Uuid,
    ) -> Result<TaskListRes> {
        let tasks_collection = db.collection::<TaskModel>("tasks");

        let mut cursor = tasks_collection
            .find(doc! { "category_id": category_id, "user": user }, None)
            .await?;

        let mut tasks_results = Vec::new();

        while let Some(task) = cursor.try_next().await? {
            let update_task_req = UpdateTaskReq {
                category_name: Some(new_category_name.to_string()),
                category_color: Some(new_category_color.to_string()),
                ..Default::default()
            };

            let task_result =
                base::update::<Self, UpdateTaskReq>(db, &task.id.to_hex(), &update_task_req, user)
                    .await
                    .expect("task 업데이트에 실패했습니다.");

            tasks_results.push(task_result);
        }

        Ok(TaskListRes {
            status: "success",
            results: tasks_results.len(),
            tasks: tasks_results,
        })
    }

    pub async fn update_tasks_for_property_change(
        db: &Database,
        category_id: &str,
        prop_id: &str,
        new_prop_name: &str,
        new_prop_type: &PropertyType,
        user_id: &Uuid,
    ) -> Result<PropValueListRes> {
        let tasks_collection = db.collection::<TaskModel>("tasks");
        let category_oid =
            ObjectId::from_str(&category_id).map_err(|e| DBError::MongoGetOidError(e))?;
        let prop_oid = ObjectId::from_str(&prop_id).map_err(|e| DBError::MongoGetOidError(e))?;
        let mut cursor = tasks_collection
            .find(doc! { "category_id": category_oid, "user": user_id }, None)
            .await?;

        let mut prop_results = Vec::new();

        while let Some(task) = cursor.try_next().await? {
            if let Some(prop_value) = task.prop_values.iter().find(|p| p.prop_id == prop_oid) {
                let values = match &prop_value.values {
                    Some(val) => Some(vec![val.clone()]),
                    None => None,
                };

                let update_prop_req = UpdatePropValueReq {
                    name: Some(new_prop_name.to_string()),
                    values,
                    prop_type: Some(new_prop_type.to_owned()),
                };

                let prop_result =
                    PropValueService::update_propV(db, &category_id, &prop_id, &update_prop_req)
                        .await?;

                prop_results.push(prop_result.data.propV);
            }
        }

        Ok(PropValueListRes {
            status: "success",
            results: prop_results.len(),
            propVs: prop_results,
        })
    }

    // utils
    pub async fn get_prop_values(db: &Database, category_id: &str) -> Result<Vec<PropValueModel>> {
        let properties = PropertyService::fetch_properties(db, category_id)
            .await
            .expect("properties를 가져오는데 실패했습니다.")
            .props;

        let prop_values = properties
            .iter()
            .map(|prop| {
                let prop_oid = ObjectId::from_str(&prop.id)
                    .map_err(DBError::MongoGetOidError)
                    .unwrap();
                PropValueModel {
                    prop_id: prop_oid,
                    prop_name: prop.name.clone(),
                    prop_type: prop.prop_type.clone(),
                    values: None,
                }
            })
            .collect::<Vec<PropValueModel>>();
        Ok(prop_values)
    }
}
