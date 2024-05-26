use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::str::FromStr;

use chrono::{DateTime, Utc};
use mongodb::bson::{self, doc, oid::ObjectId, Bson};
use mongodb::{options::FindOneAndUpdateOptions, options::ReturnDocument, Database};

use crate::interface::dto::chat::req::UpdateMsgReq;

use crate::{
    domain::error::{Error::*, Result},
    domain::repo::base_array::{self, MongoArrayRepo},
    infra::db::error::Error as DBError,
};

use super::event::EventModel;
use super::task::TaskModel;
use super::types::{ChatType, MsgType};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MsgModel {
    pub id: ObjectId,
    pub msg_type: MsgType,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub booked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<ChatType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_msgs: Option<Vec<MsgModel>>,
}

pub trait HasChatMsgs {
    fn msgs(&self) -> Option<Vec<MsgModel>>;
}

impl HasChatMsgs for EventModel {
    fn msgs(&self) -> Option<Vec<MsgModel>> {
        self.chat_msgs.clone()
    }
}

impl HasChatMsgs for TaskModel {
    fn msgs(&self) -> Option<Vec<MsgModel>> {
        self.chat_msgs.clone()
    }
}

pub struct ChatMsgRepo<Model> {
    _phantom: std::marker::PhantomData<Model>,
}

pub trait CollInfo {
    const COLL_NAME: &'static str;
    const ARR_NAME: &'static str;
}

impl CollInfo for EventModel {
    const COLL_NAME: &'static str = "events";
    const ARR_NAME: &'static str = "chat_msgs";
}

impl CollInfo for TaskModel {
    const COLL_NAME: &'static str = "tasks";
    const ARR_NAME: &'static str = "chat_msgs";
}

impl<Model> MongoArrayRepo for ChatMsgRepo<Model>
where
    Model: HasChatMsgs + DeserializeOwned + Unpin + Send + Sync + CollInfo,
{
    type CollModel = Model;
    type ElemModel = MsgModel;
    type UpdateElemReq = UpdateMsgReq;
    const COLL_NAME: &'static str = Model::COLL_NAME;
    const ARR_NAME: &'static str = Model::ARR_NAME;
}

impl<Model> ChatMsgRepo<Model>
where
    Model: HasChatMsgs + DeserializeOwned + Unpin + Send + Sync + CollInfo,
{
    pub async fn get_msg(db: &Database, src_id: &str, msg_id: &str) -> Result<MsgModel> {
        let doc: MsgModel = base_array::get_elem::<Self>(db, src_id, msg_id).await?;
        Ok(doc)
    }

    pub async fn add_msg(db: &Database, src_id: &str, new_msg: &MsgModel) -> Result<Vec<MsgModel>> {
        let doc: Model = base_array::add_elem::<Self>(db, src_id, new_msg).await?;
        Ok(doc.msgs().unwrap_or_default())
    }

    pub async fn fetch_msgs(db: &Database, src_id: &str) -> Result<Vec<MsgModel>> {
        let doc: Model = base_array::fetch_elems::<Self>(db, src_id).await?;
        Ok(doc.msgs().unwrap_or_default())
    }

    pub async fn remove_msg(db: &Database, src_id: &str, msg_id: &str) -> Result<Vec<MsgModel>> {
        let doc: Model = base_array::remove_elem::<Self>(db, src_id, msg_id).await?;
        Ok(doc.msgs().unwrap_or_default())
    }

    pub async fn update_msg(
        db: &Database,
        src_id: &str,
        msg_id: &str,
        new_msg: &UpdateMsgReq,
    ) -> Result<Vec<MsgModel>> {
        let doc: Model = base_array::update_elem::<Self>(db, src_id, msg_id, new_msg).await?;
        Ok(doc.msgs().unwrap_or_default())
    }

    pub async fn add_chat_to_msg(
        db: &Database,
        src_id: &str,
        msg_id: &str,
    ) -> Result<Vec<MsgModel>> {
        let coll = db.collection::<Model>(Self::COLL_NAME);

        let oid = ObjectId::from_str(src_id).map_err(DBError::MongoGetOidError)?;

        let update_doc = doc! {
            "$push": { "chat_msgs.$[msg].chat_type": bson::to_bson(&ChatType::Ask).map_err(DBError::MongoSerializeBsonError)? },
            "$set": { "updatedAt": Bson::DateTime(Utc::now().into()) }
        };
        let array_filters =
            bson::doc! { "msg._id": ObjectId::from_str(msg_id).map_err(DBError::MongoGetOidError)?};

        let options = FindOneAndUpdateOptions::builder()
            .array_filters(Some(vec![array_filters]))
            .return_document(ReturnDocument::After)
            .build();

        let doc = match coll
            .find_one_and_update(doc! {"_id": oid}, update_doc, options)
            .await
        {
            Ok(Some(doc)) => doc,
            Ok(None) => return Err(NotFoundError(oid.to_string())),
            Err(e) => return Err(DB(DBError::MongoQueryError(e))),
        };

        Ok(doc.msgs().unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::types::{ChatType, MsgType},
        infra::db::MongoDB,
    };
    use dotenv::dotenv;
    use uuid::Uuid;

    async fn get_test_db() -> Database {
        dotenv().ok();
        std::env::set_var("RUST_BACKTRACE", "0");
        let mongodb = MongoDB::init_test().await.unwrap();

        mongodb.db
    }

    async fn get_fake_src_id() -> String {
        let db = get_test_db().await;
        let coll = db.collection::<EventModel>("events");

        let new_event = EventModel {
            id: ObjectId::new(),
            user: Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            title: "test chatting".to_string(),
            complete: false,
            start_date: None,
            due_at: None,
            location: None,
            chat_type: ChatType::Event,
            chat_msgs: None,
            createdAt: Utc::now(),
            updatedAt: Utc::now(),
        };

        let result = coll.insert_one(new_event, None).await.unwrap();
        result.inserted_id.as_object_id().unwrap().to_hex()
    }

    async fn get_fake_msg_id(db: &Database, src_id: &str) -> String {
        let coll = db.collection::<EventModel>("events");

        let oid = ObjectId::from_str(src_id).unwrap();

        let new_msg = MsgModel {
            id: ObjectId::new(),
            msg_type: MsgType::Text,
            content: "fake 새로운 메세지".to_string(),
            created_at: Utc::now(),
            booked: false,
            chat_msgs: None,
            chat_type: None,
        };

        let update_doc = doc! {
            "$push": { "chat_msgs": bson::to_bson(&new_msg).unwrap() },
            "$set": { "updatedAt": Bson::DateTime(Utc::now().into()) }
        };

        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        let doc = coll
            .find_one_and_update(doc! {"_id": oid}, update_doc, options)
            .await
            .unwrap()
            .unwrap();

        doc.msgs().unwrap().last().unwrap().id.to_hex()
    }

    #[tokio::test]
    async fn test_add_msg() -> Result<()> {
        let db = get_test_db().await;
        let src_id = get_fake_src_id().await;
        let new_msg = MsgModel {
            id: ObjectId::new(),
            msg_type: MsgType::Text,
            content: "배고파요".to_string(),
            created_at: Utc::now(),
            booked: false,
            chat_msgs: None,
            chat_type: None,
        };

        let result = ChatMsgRepo::<EventModel>::add_msg(&db, &src_id, &new_msg).await?;
        assert!(result.contains(&new_msg));
        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_msgs() -> Result<()> {
        let db = get_test_db().await;
        let src_id = get_fake_src_id().await;
        let result = ChatMsgRepo::<EventModel>::fetch_msgs(&db, &src_id).await?;
        assert!(!result.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_remove_msg() -> Result<()> {
        let db = get_test_db().await;
        let src_id = get_fake_src_id().await;
        let msg_id = get_fake_msg_id(&db, &src_id).await;
        let result = ChatMsgRepo::<EventModel>::remove_msg(&db, &src_id, &msg_id).await?;

        assert!(!result.iter().any(|msg| msg.id.to_hex() == msg_id));
        Ok(())
    }

    #[tokio::test]
    async fn test_update_msg() -> Result<()> {
        let db = get_test_db().await;
        let src_id = get_fake_src_id().await;
        let msg_id = get_fake_msg_id(&db, &src_id).await;
        let update_msg = UpdateMsgReq {
            msg_type: todo!(),
            content: todo!(),
            booked: todo!(),
        };

        let result =
            ChatMsgRepo::<EventModel>::update_msg(&db, &src_id, &msg_id, &update_msg).await?;
        // assert based on the expected outcome of the update
        Ok(())
    }

    #[tokio::test]
    async fn test_add_chat_to_msg() -> Result<()> {
        let db = get_test_db().await;
        let src_id = get_fake_src_id().await;
        let msg_id = get_fake_msg_id(&db, &src_id).await;
        let result =
            ChatMsgRepo::<EventModel>::add_chat_to_msg(&db, "some_src_id", "some_msg_id").await?;
        // assert based on the expected outcome of adding chat to message
        Ok(())
    }
}