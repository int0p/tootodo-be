use chrono::{DateTime, NaiveDate, Utc};
use futures::TryStreamExt;
use std::str::FromStr;

use super::{
    category::CategoryModel,
    sub::{
        chat::MsgModel,
        task_block::BlockModel,
        task_propV::{PropValueModel, PropValueService},
    },
    types::{ChatType, PropValueType, PropertyType},
};

use crate::{
    domain::{
        error::{Error::*, Result},
        repo::base::{self, MongoRepo},
    },
    infra::db::error::Error as DBError,
    interface::dto::{
        sub::task_propV::{req::UpdatePropValueReq, res::PropValueListRes},
        task::{
            req::{CreateTaskReq, UpdateTaskReq},
            res::{SingleTaskRes, TaskData, TaskListRes, TaskRes},
        },
    },
};

use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    options::{AggregateOptions, FindOneAndUpdateOptions, ReturnDocument},
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
            prop_values: original_task.prop_values.clone(),
            blocks: vec![BlockModel::new(original_task.id)],
            subtasks: vec![],
            parent_id: Some(original_task.id),
            chat_type: original_task.chat_type.clone(),
            chat_msgs: original_task.chat_msgs.clone(),
            createdAt: Utc::now(),
            updatedAt: Utc::now(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TaskService;

impl MongoRepo for TaskService {
    const COLL_NAME: &'static str = "tasks";
    const DOC_COLL_NAME: &'static str = "tasks";
    type Model = TaskModel;
    type ModelResponse = TaskRes;

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
    //mongodb에서 task를 가져옴.
    pub async fn fetch_tasks(
        db: &Database,
        limit: i64,
        page: i64,
        user: &Uuid,
    ) -> Result<TaskListRes> {
        let tasks_result = base::fetch::<Self>(db, limit, page, user)
            .await
            .expect("task 응답을 받아오지 못했습니다.");

        Ok(TaskListRes {
            status: "success",
            results: tasks_result.len(),
            tasks: tasks_result,
        })
    }

    pub async fn create_task(
        db: &Database,
        body: &CreateTaskReq,
        user: &Uuid,
    ) -> Result<SingleTaskRes> {
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

    // subtask
    pub async fn add_subtask(self, db: &Database, id: &str, user: &Uuid) -> Result<SingleTaskRes> {
        // Parse the task id
        let task_oid = ObjectId::from_str(id).map_err(DBError::MongoGetOidError)?;

        // Retrieve the original task
        let coll = db.collection::<TaskModel>("tasks");
        let mut original_task = match coll
            .find_one(doc! { "_id": &task_oid, "user": user }, None)
            .await
        {
            Ok(Some(doc)) => doc,
            Ok(None) => return Err(NotFoundError(task_oid.to_string())),
            Err(e) => return Err(DB(DBError::MongoQueryError(e))),
        };

        // Add the new subtask to the original task's subtasks
        original_task
            .subtasks
            .push(TaskModel::new_subtask(&original_task));

        // Serialize the subtasks
        let subtasks_bson = bson::to_bson(&original_task.subtasks)
            .map_err(|e| DB(DBError::MongoSerializeBsonError(e)))?;

        // Update the original task in the database
        coll.update_one(
            doc! { "_id": &task_oid, "user": user },
            doc! { "$set": { "subtasks": subtasks_bson }},
            None,
        )
        .await?;

        // convert doc to Res
        let task_result = Self::convert_doc_to_response(&original_task)?;

        // Return the updated task
        Ok(SingleTaskRes {
            status: "success",
            data: TaskData { task: task_result },
        })
    }

    pub async fn get_task_with_subtasks(
        db: &Database,
        id: &str,
        user: &Uuid,
    ) -> Result<SingleTaskRes> {
        // Parse the task id
        let task_oid = ObjectId::from_str(id).map_err(DBError::MongoGetOidError)?;

        // Define the aggregation pipeline
        let pipeline = vec![
            doc! { "$match": { "_id": &task_oid, "user": user }},
            doc! {
                "$graphLookup": {
                    "from": "tasks",
                    "startWith": "$_id",
                    "connectFromField": "_id",
                    "connectToField": "parent_id",
                    "as": "subtasks",
                    "maxDepth": 5,
                    "depthField": "depth"
                }
            },
        ];

        let options = AggregateOptions::builder().allow_disk_use(true).build();
        let tasks_collection = db.collection::<TaskModel>("tasks");
        let mut cursor = tasks_collection.aggregate(pipeline, options).await?;
        let mut tasks = Vec::new();

        while let Some(result) = cursor.try_next().await? {
            tasks.push(bson::from_document::<TaskModel>(result));
        }

        // Assuming there's only one task matched, as we queried by _id
        let updated_task = match tasks.into_iter().next() {
            Some(Ok(task)) => task,
            Some(Err(e)) => return Err(DB(DBError::MongoDeserializeBsonError(e))),
            None => return Err(NotFoundError(task_oid.to_string())),
        };

        // convert doc to Res
        let task_result = Self::convert_doc_to_response(&updated_task)?;

        // Return the updated task
        Ok(SingleTaskRes {
            status: "success",
            data: TaskData { task: task_result },
        })
    }
}
