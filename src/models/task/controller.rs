use super::{
    model::TaskModel,
    response::{TaskData, TaskListResponse, TaskResponse, SingleTaskResponse},
    schema::{CreateTaskSchema, UpdateTaskSchema},
};
use crate::{
    db::error::Error as DBError,
    models::{base::{self, MongoBMC}, error::{Error::*, Result}},
};
use chrono::prelude::*;
use mongodb::bson;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::{bson::Document, Database};
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
            complete: task.complete.to_owned(),
            start_date: task.start_date.to_owned(),
            due_at: task.due_at.to_owned(),
            location: task.location.to_owned(),
            chat_type: task.chat_type.to_owned(),
            chat_msgs: task.chat_msgs.to_owned(),
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
        
        // let msgs = ChatModel {
        //     src_type: ChatType::Task,
        //     msgs: None,
        // };
        // let serialized_chat = bson::to_bson(&msgs).map_err(|e| DB(DBError::MongoSerializeBsonError(e)))?;
        let datetime = Utc::now();

        let mut doc_with_dates = doc! {
            "user": user,
            "complete":false,
            "chat_type": "Task",
            // "chat": serialized_chat,
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
        db: &Database,
        body: &CreateTaskSchema,
        user: &Uuid,
    ) -> Result<SingleTaskResponse> {
        let task_result = base::create::<Self, CreateTaskSchema>(db, body, user)
            .await
            .expect("task 생성에 실패했습니다.");

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

    pub async fn delete_task(db: &Database, id: &str) -> Result<()> {
        base::delete::<Self>(db, id).await
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::{db::MongoDB, models::chat::{model::{ChatType, MsgModel, MsgType}, schema::UpdateChatSchema}};
    use dotenv::dotenv;
    use mongodb::options::UpdateOptions;

    async fn setup() -> Database {
        dotenv().ok();
        std::env::set_var("RUST_BACKTRACE", "0");
        let mongodb = MongoDB::init_test().await.unwrap();

        // 시드 데이터 생성
        let user = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let seeds = vec![
            TaskModel {
                id: ObjectId::from_str("507f1f77bcf86cd799439011").unwrap(),
                user,
                title: "잼미니 대회 관련 미팅 1회차".to_string(),
                complete: true,
                chat_type: ChatType::Task,
                chat_msgs: None,
                start_date: Some(Utc::now().date_naive()),
                due_at: Some(Utc.with_ymd_and_hms(2024 ,5, 28, 0, 0, 0).unwrap()),
                location: None,
                createdAt: Utc::now(),
                updatedAt: Utc::now(),
            },
            TaskModel {
                id: ObjectId::from_str("507f1f77bcf86cd799439013").unwrap(),
                user,
                title: "잼미니 대회 관련 미팅 2회차".to_string(),
                complete: true,
                chat_type: ChatType::Task,
                chat_msgs: Some(vec![
                        MsgModel {
                            msg_type: MsgType::Ask, 
                            content: "기술스택 토론 예정".to_string(),
                            created_at: Utc::now(),
                            booked: false,
                            chat: None,
                        },
                    ]),
                start_date: Some(Utc::now().date_naive()),
                due_at: Some(Utc.with_ymd_and_hms(2024 ,5, 30, 0, 0, 0).unwrap()),
                location: Some("학교".to_string()),
                createdAt: Utc::now(),
                updatedAt: Utc::now(),
            }
        ];

        // 시드 데이터를 MongoDB에 삽입
        for seed in seeds {
            let filter = doc! { "_id": seed.id };
            let update = doc! { "$setOnInsert": bson::to_bson(&seed).unwrap() };
            let options = UpdateOptions::builder().upsert(true).build();

            let result = mongodb
                .db
                .collection::<TaskModel>("tasks")
                .update_one(filter, update, options)
                .await
                .expect("cannot insert seed data");

            // if result.upserted_id.is_some() {
            //     println!(
            //         "✅ 새로운 노트 시드 데이터가 추가되었습니다. ID: {}",
            //         seed.id
            //     );
            // } else {
            //     println!("이미 존재하는 노트 시드 데이터입니다. ID: {}", seed.id);
            // }
        }

        mongodb.db
    }

    #[tokio::test]
    async fn test_create_task() {
        let db = setup().await;
        let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let body1 = CreateTaskSchema {
            title: "Test Task".to_string(),
            start_date: None,
            due_at: None,
            location: None,
        };

        let body2 = CreateTaskSchema {
            title: "Test Task2".to_string(),
            start_date: None,
            due_at: Some(Utc::now()),
            location: None,
        };

        let res = TaskBMC::create_task(&db, &body1, &user_id).await;
        claim::assert_ok!(&res);
        let res = res.unwrap();
        claim::assert_matches!(res.status, "success");
        assert_eq!(res.data.task.title, body1.title);

        let res = TaskBMC::create_task(&db, &body2, &user_id).await;
        claim::assert_ok!(&res);
        let res = res.unwrap();
        claim::assert_matches!(res.status, "success");
        assert_eq!(res.data.task.title, body2.title);
    }

    #[tokio::test]
    async fn test_fetch_tasks() {
        let db = setup().await;
        let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let limit = 10;
        let page = 1;

        let res = TaskBMC::fetch_tasks(&db, limit, page, &user_id).await;
        claim::assert_ok!(&res);
        let res = res.unwrap();
        claim::assert_matches!(res.status, "success");
    }

    #[tokio::test]
    async fn test_get_task() {
        let db = setup().await;
        let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let task_id = "507f1f77bcf86cd799439013";

        let res = TaskBMC::get_task(&db, task_id, &user_id).await;
        claim::assert_ok!(&res);
        let res = res.unwrap();
        claim::assert_matches!(res.status, "success");
        assert_eq!(res.data.task.id, task_id);
    }

    #[tokio::test]
    async fn test_update_task() {
        let db = setup().await;
        let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let task_id = "507f1f77bcf86cd799439013";
        let body = UpdateTaskSchema {
            title: Some("Updated Title".to_string()),
            complete: Some(true),
            start_date:None,     
            due_at: None,
            location: None,
            chat_type: Some(ChatType::Task),      
        };

        let res =  TaskBMC::update_task(&db, task_id, &body, &user_id).await;
        claim::assert_ok!(&res);
        let res = res.unwrap();
        claim::assert_matches!(res.status, "success");
        assert_eq!(res.data.task.title, body.title.unwrap());
        // if let Some(content) = body.content{
        //     assert_eq!(res.data.task.content, content);            
        // } 
        // else {dbg!(res.data.task.content);} //기존값 유지
    }

    #[tokio::test]
    async fn test_delete_task() {
        let db = setup().await;
        let task_id = "507f1f77bcf86cd799439011";

        let res = TaskBMC::delete_task(&db, task_id).await;
        claim::assert_ok!(&res);
    }
}
