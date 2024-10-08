use serde::{de::DeserializeOwned, Deserialize, Serialize};

use chrono::{DateTime, Utc};
use mongodb::bson::{self, doc, oid::ObjectId};
use mongodb::Database;

use crate::domain::repo::CollInfo;
use crate::interface::dto::sub::chat::req::{CreateMsgReq, UpdateMsgReq};

use crate::interface::dto::sub::chat::res::*;
use crate::{
    domain::error::{Result},
    domain::repo::base_array::{self, MongoArrayRepo},
};

use crate::domain::task::TaskModel;
//use crate::domain::note::NoteModel;

use crate::infra::types::{ChatType, MsgType};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MsgModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub msg_type: MsgType,
    pub content: String,
    pub booked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<ChatType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_msgs: Option<Vec<MsgModel>>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
}


// impl CollInfo for NoteModel {
//     const COLL_NAME: &'static str = "notes";
//     const ARR_NAME: &'static str = "chat_msgs";
// }



impl CollInfo for TaskModel {
    const COLL_NAME: &'static str = "tasks";
    const ARR_NAME: &'static str = "chat_msgs";
}

pub struct ChatMsgService<Model> {
    _phantom: std::marker::PhantomData<Model>,
}

// event 혹은 task collection의 chat_msgs 배열필드에 msg를 CRUD하는 서비스.
impl<Model> MongoArrayRepo for ChatMsgService<Model>
where
    Model: DeserializeOwned + Serialize + Unpin + Send + Sync + CollInfo,
{
    type CollModel = Model;
    type ElemModel = MsgModel;
    type UpdateElemReq = UpdateMsgReq;
    type CreateElemReq = CreateMsgReq;
    type ElemRes = MsgRes;
    const COLL_NAME: &'static str = Model::COLL_NAME;
    const ARR_NAME: &'static str = Model::ARR_NAME;
    fn convert_doc_to_response(doc: &MsgModel) -> Result<Self::ElemRes> {
        Ok(MsgRes::from_model(doc))
    }
}

impl<Model> ChatMsgService<Model>
where
    Model: DeserializeOwned + Serialize + Unpin + Send + Sync + CollInfo,
{
    pub async fn get_msg(db: &Database, src_id: &str, msg_id: &str) -> Result<SingleMsgRes> {
        let result = base_array::get_elem::<Self>(db, src_id, msg_id).await?;
        Ok(SingleMsgRes {
            status: "success",
            data: MsgData { msg: result },
        })
    }

    pub async fn add_msg(
        db: &Database,
        src_id: &str,
        new_msg: &CreateMsgReq,
    ) -> Result<SingleMsgRes> {
        let result = base_array::add_elem::<Self>(db, src_id, new_msg, Some("booked")).await?;
        Ok(SingleMsgRes {
            status: "success",
            data: MsgData { msg: result },
        })
    }

    pub async fn fetch_msgs(
        db: &Database,
        src_id: &str,
        limit: i64,
        page: i64,
    ) -> Result<MsgListRes> {
        let results = base_array::fetch_elems::<Self>(db, src_id, Some(limit), Some(page)).await?;
        Ok(MsgListRes {
            status: "success",
            results: results.len(),
            msgs: results,
        })
    }

    pub async fn remove_msg(db: &Database, src_id: &str, msg_id: &str) -> Result<()> {
        base_array::remove_elem::<Self>(db, src_id, msg_id).await
    }

    pub async fn update_msg(
        db: &Database,
        src_id: &str,
        msg_id: &str,
        update_msg: &UpdateMsgReq,
    ) -> Result<SingleMsgRes> {
        let result = base_array::update_elem::<Self>(db, src_id, msg_id, update_msg).await?;
        Ok(SingleMsgRes {
            status: "success",
            data: MsgData { msg: result },
        })
    }

    // TODO: 그냥 update_msg함수로 될 것 같은데?
    // pub async fn add_chat_to_msg(
    //     db: &Database,
    //     src_id: &str,
    //     msg_id: &str,
    // ) -> Result<SingleMsgRes> {
    //     let coll = db.collection::<Model>(Self::COLL_NAME);

    //     let oid = ObjectId::from_str(src_id).map_err(DBError::MongoGetOidError)?;

    //     let update_doc = doc! {
    //         "$push": { "chat_msgs.$[msg].chat_type": bson::to_bson(&ChatType::Ask).map_err(DBError::MongoSerializeBsonError)? },
    //         "$set": { "updatedAt": Bson::DateTime(Utc::now().into()) }
    //     };

    //     let array_filters =
    //         bson::doc! { "msg._id": ObjectId::from_str(msg_id).map_err(DBError::MongoGetOidError)?};

    //     let result = match update_mdoc_by_id(
    //         &coll,
    //         &oid,
    //         Some(array_filters),
    //         update_doc,
    //         doc! { "_id": oid },
    //     )
    //     .await
    //     {
    //         Ok(updated_doc) => get_m_elems("chat_msgs", &updated_doc).await?,
    //         Err(e) => Err(e),
    //     };

    //     Ok(SingleMsgRes {
    //         status: "success",
    //         data: MsgData { msg: result },
    //     })
    // }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{
//         domain::types::{ChatType, MsgType},
//         infra::db::MongoDB,
//     };
//     use dotenv::dotenv;
//     use uuid::Uuid;

//     async fn get_test_db() -> Database {
//         dotenv().ok();
//         std::env::set_var("RUST_BACKTRACE", "0");
//         let mongodb = MongoDB::init_test().await.unwrap();

//         mongodb.db
//     }

//     async fn get_fake_src_id() -> String {
//         let db = get_test_db().await;
//         let coll = db.collection::<EventModel>("events");

//         let new_event = EventModel {
//             id: ObjectId::new(),
//             user: Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
//             title: "test chatting".to_string(),
//             complete: false,
//             start_date: None,
//             due_at: None,
//             location: None,
//             chat_type: ChatType::Event,
//             chat_msgs: None,
//             createdAt: Utc::now(),
//             updatedAt: Utc::now(),
//         };

//         let result = coll.insert_one(new_event, None).await.unwrap();
//         result.inserted_id.as_object_id().unwrap().to_hex()
//     }

//     async fn get_fake_msg_id(db: &Database, src_id: &str) -> String {
//         let coll = db.collection::<EventModel>("events");

//         let oid = ObjectId::from_str(src_id).unwrap();

//         let new_msg = MsgModel {
//             id: ObjectId::new(),
//             msg_type: MsgType::Text,
//             content: "fake 새로운 메세지".to_string(),
//             created_at: Utc::now(),
//             booked: false,
//             chat_msgs: None,
//             chat_type: None,
//         };

//         let update_doc = doc! {
//             "$push": { "chat_msgs": bson::to_bson(&new_msg).unwrap() },
//             "$set": { "updatedAt": Bson::DateTime(Utc::now().into()) }
//         };

//         let options = FindOneAndUpdateOptions::builder()
//             .return_document(ReturnDocument::After)
//             .build();

//         let doc = coll
//             .find_one_and_update(doc! {"_id": oid}, update_doc, options)
//             .await
//             .unwrap()
//             .unwrap();

//         doc.msgs().unwrap().last().unwrap().id.to_hex()
//     }

//     #[tokio::test]
//     async fn test_add_msg() -> Result<()> {
//         let db = get_test_db().await;
//         let src_id = get_fake_src_id().await;
//         let new_msg = MsgModel {
//             id: ObjectId::new(),
//             msg_type: MsgType::Text,
//             content: "배고파요".to_string(),
//             created_at: Utc::now(),
//             booked: false,
//             chat_msgs: None,
//             chat_type: None,
//         };

//         let result = ChatMsgService::<EventModel>::add_msg(&db, &src_id, &new_msg).await?;
//         assert!(result.contains(&new_msg));
//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_fetch_msgs() -> Result<()> {
//         let db = get_test_db().await;
//         let src_id = get_fake_src_id().await;
//         let result = ChatMsgService::<EventModel>::fetch_msgs(&db, &src_id).await?;
//         assert!(!result.is_empty());
//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_remove_msg() -> Result<()> {
//         let db = get_test_db().await;
//         let src_id = get_fake_src_id().await;
//         let msg_id = get_fake_msg_id(&db, &src_id).await;
//         let result = ChatMsgService::<EventModel>::remove_msg(&db, &src_id, &msg_id).await?;

//         assert!(!result.iter().any(|msg| msg.id.to_hex() == msg_id));
//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_update_msg() -> Result<()> {
//         let db = get_test_db().await;
//         let src_id = get_fake_src_id().await;
//         let msg_id = get_fake_msg_id(&db, &src_id).await;
//         let update_msg = UpdateMsgReq {
//             msg_type: todo!(),
//             content: todo!(),
//             booked: todo!(),
//         };

//         let result =
//             ChatMsgService::<EventModel>::update_msg(&db, &src_id, &msg_id, &update_msg).await?;
//         // assert based on the expected outcome of the update
//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_add_chat_to_msg() -> Result<()> {
//         let db = get_test_db().await;
//         let src_id = get_fake_src_id().await;
//         let msg_id = get_fake_msg_id(&db, &src_id).await;
//         let result =
//             ChatMsgService::<EventModel>::add_chat_to_msg(&db, "some_src_id", "some_msg_id")
//                 .await?;
//         // assert based on the expected outcome of adding chat to message
//         Ok(())
//     }
// }
