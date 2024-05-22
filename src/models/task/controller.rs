use std::str::FromStr;

use super::{
    model::{PropertyValue, PropertyValueData, TaskModel},
    response::{SingleTaskResponse, TaskData, TaskListResponse, TaskResponse},
    schema::{CreateTaskSchema, UpdateTaskSchema},
};
use crate::{
    db::error::Error as DBError,
    models::{
        base::{self, MongoBMC},
        category::model::{CategoryModel, PropertyType},
        error::{Error::*, Result},
    },
};
use chrono::prelude::*;
use mongodb::{
    bson::doc,
    options::{FindOneAndUpdateOptions, UpdateOptions},
};
use mongodb::{bson::Document, Database};
use mongodb::{
    bson::{self, oid::ObjectId},
    options::ReturnDocument,
};
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct TaskBMC;

impl MongoBMC for TaskBMC {
    const COLL_NAME: &'static str = "tasks";
    const DOC_COLL_NAME: &'static str = "tasks";
    type Model = TaskModel;
    type ModelResponse = TaskResponse;

    fn convert_doc_to_response(task: &TaskModel) -> Result<TaskResponse> {
        let task_response = TaskResponse {
            user: task.user,
            id: task.id.to_hex(),
            title: task.title.to_owned(),
            start_date: task.start_date.to_owned(),
            due_at: task.due_at.to_owned(),
            chat_type: task.chat_type.to_owned(),
            chat_msgs: task.chat_msgs.to_owned(),
            category: task.category.to_owned(),
            proerties: task.proerties.to_owned(),
            subtasks: task.subtasks.to_owned(),
            parent_id: task.parent_id.to_owned(),
            blocks: task.blocks.to_owned(),
            createdAt: task.createdAt,
            updatedAt: task.updatedAt,
        };
        Ok(task_response)
    }

    fn create_doc<CreateTaskSchema: Serialize>(
        user: &Uuid,
        body: &CreateTaskSchema,
    ) -> Result<Document> {
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

impl TaskBMC {
    //mongodb에서 task를 가져옴.
    pub async fn fetch_tasks(
        db: &Database,
        limit: i64,
        page: i64,
        user: &Uuid,
    ) -> Result<TaskListResponse> {
        let tasks_result = base::fetch::<Self>(db, limit, page, user)
            .await
            .expect("task 응답을 받아오지 못했습니다.");

        Ok(TaskListResponse {
            status: "success",
            results: tasks_result.len(),
            tasks: tasks_result,
        })
    }

    pub async fn create_task(
        self,
        db: &Database,
        body: &CreateTaskSchema,
        user: &Uuid,
    ) -> Result<SingleTaskResponse> {
        let task_result = base::create::<Self, CreateTaskSchema>(db, body, user)
            .await
            .expect("task 생성에 실패했습니다.");

        // Property 정보 추가.
        let category_collection = db.collection::<CategoryModel>("categories");
        let category = category_collection
            .find_one(doc! { "_id": task_result.category.id }, None)
            .await
            .expect("Failed to fetch category")
            .expect("Category not found");

        let properties: Vec<PropertyValue> = category
            .properties
            .iter()
            .map(|prop| PropertyValue {
                prop_id: prop.id.clone(),
                prop_name: prop.name.clone(),
                value: None,
                prop_type: prop.prop_type.clone(),
            })
            .collect();

        self.update_task(
            db,
            &task_result.id,
            &UpdateTaskSchema {
                properties: Some(properties),
                title: None,
                category: None,
                start_date: None,
                due_at: None,
                parent_id: None,
                subtasks: None,
                blocks: None,
                chat_type: None,
            },
            user,
        );

        Ok(SingleTaskResponse {
            status: "success",
            data: TaskData { task: task_result },
        })
    }

    pub async fn get_task(db: &Database, id: &str, user: &Uuid) -> Result<SingleTaskResponse> {
        let task_result = base::get::<Self>(db, id, user)
            .await
            .expect("task를 가져오는데 실패했습니다.");

        Ok(SingleTaskResponse {
            status: "success",
            data: TaskData { task: task_result },
        })
    }

    pub async fn update_task(
        self,
        db: &Database,
        id: &str,
        body: &UpdateTaskSchema,
        user: &Uuid,
    ) -> Result<SingleTaskResponse> {
        let task_result = base::update::<Self, UpdateTaskSchema>(db, id, body, user)
            .await
            .expect("task 업데이트에 실패했습니다.");

        Ok(SingleTaskResponse {
            status: "success",
            data: TaskData { task: task_result },
        })
    }

    pub async fn update_task_prop_val(
        db: &Database,
        id: &str,
        prop_id: &str,
        value: &PropertyValueData,
        user: &Uuid,
    ) -> Result<SingleTaskResponse> {
        let task_oid = ObjectId::from_str(id).map_err(|e| DBError::MongoGetOidError(e))?;
        let prop_oid = ObjectId::from_str(prop_id).map_err(|e| DBError::MongoGetOidError(e))?;

        let coll = db.collection::<TaskModel>("tasks");

        // get task, property info from collection
        let task = match coll
            .find_one(doc! { "_id": task_oid, "user": user }, None)
            .await
        {
            Ok(Some(doc)) => doc,
            Ok(None) => return Err(NotFoundError(task_oid.to_string())),
            Err(e) => return Err(DB(DBError::MongoQueryError(e))),
        };

        let property = match task.properties.iter().find(|p| p.prop_id == prop_oid) {
            Some(prop) => prop,
            None => return Err(NotFoundError(prop_oid.to_string())),
        };

        let prop_type = &property.prop_type;

        // set new value
        let new_value = match prop_type {
            PropertyType::MultiSelect | PropertyType::SingleSelect => {
                if let PropertyValueData::Multiple(_) = value {
                    value
                } else {
                    return Err(TypedError(
                        "MultiSelect or SingleSelect types must have Multiple(Vec<String>) value"
                            .to_string(),
                    ));
                }
            }
            _ => {
                if let PropertyValueData::Single(_) = value {
                    value
                } else {
                    return Err(TypedError(
                        "Non-MultiSelect or SingleSelect types must have Single(String) value"
                            .to_string(),
                    ));
                }
            }
        };

        // update property with new value
        let update_doc = doc! {
            "$set": { "properties.$[elem].value": bson::to_bson(&new_value).map_err(|e| DBError::MongoSerializeBsonError(e))? }
        };

        let array_filters = bson::doc! {
            "arrayFilters": [ { "elem.prop_id": prop_oid } ]
        };

        let options = FindOneAndUpdateOptions::builder()
            .array_filters(Some(vec![array_filters]))
            .return_document(ReturnDocument::After)
            .build();

        let doc = match coll
            .find_one_and_update(doc! {"_id": task_oid}, doc! { "$set": update_doc }, options)
            .await
        {
            Ok(Some(doc)) => doc,
            Ok(None) => return Err(NotFoundError(task_oid.to_string())),
            Err(e) => return Err(DB(DBError::MongoQueryError(e))),
        };

        // convert doc to response
        let task_result = Self::convert_doc_to_response(&doc)?;

        Ok(SingleTaskResponse {
            status: "success",
            data: TaskData { task: task_result },
        })
    }

    pub async fn delete_task(db: &Database, id: &str) -> Result<()> {
        base::delete::<Self>(db, id).await
    }
}
