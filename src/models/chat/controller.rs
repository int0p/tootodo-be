use mongodb::bson::{self, doc, oid::ObjectId, Bson, Document};
use chrono::{DateTime, Utc};
use mongodb::{Client, Collection, Database, options::FindOneAndUpdateOptions, options::ReturnDocument};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

use crate::models::{event::{model::EventModel,response::EventResponse,controller::EventBMC}, task::model::TaskModel};
use crate::models::error::{Error::*, Result};
use crate::db::error::Error as DBError;
use super::{model::*, schema::UpdateMsgSchema};



pub trait HasChatMsgs {
    fn msgs(&self) -> Option<Vec<MsgModel>>;
}

impl HasChatMsgs for EventModel {
    fn msgs(&self) -> Option<Vec<MsgModel>> {
        self.chat_msgs.clone()
    }
}

// impl HasChatMsgs for TaskModel {
//     fn msgs(&self) -> Option<Vec<MsgModel>> {
//         self.chat_msgs.clone()
//     }
// }

impl ChatMsgBMC for EventBMC {
    type TargetModel = EventModel;
    type TargetResponse = EventResponse;
    const COLL_NAME: &'static str = "events";
}

// impl ChatMsgBMC for TaskBMC {
//     type TargetModel = Self;
//     const COLL_NAME: &'static str = "tasks";
// }

pub trait ChatMsgBMC {
    type TargetModel: DeserializeOwned + Unpin + Send + Sync + HasChatMsgs ;
    type TargetResponse;
    // const DOC_COLL_NAME: &'static str;
    const COLL_NAME: &'static str; 

    async fn add_msg(
        db: &Database,
        src_id: &str,
        new_msg: MsgModel,
    ) -> Result<Vec<MsgModel>>
    {
        let coll = db.collection::<Self::TargetModel>(Self::COLL_NAME);

        let oid = ObjectId::from_str(src_id).map_err(|e| DBError::MongoGetOidError(e))?;

        let update_doc = doc! {
            "$push": { "chat_msgs": bson::to_bson(&new_msg).map_err(|e| DBError::MongoSerializeBsonError(e))? },
            "$set": { "updatedAt": Bson::DateTime(Utc::now().into()) }
        };

        let options = FindOneAndUpdateOptions::builder()
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

    async fn fetch_msgs(
        db: &Database,
        src_id: &str,
    ) -> Result<Vec<MsgModel>>
    {
        let coll = db.collection::<Self::TargetModel>(Self::COLL_NAME);

        let oid = ObjectId::from_str(src_id).map_err(|e| DBError::MongoGetOidError(e))?;

        let doc = match coll
            .find_one(doc! {"_id": oid}, None)
            .await{
                Ok(Some(doc)) => doc,
                Ok(None) => return Err(NotFoundError(oid.to_string())),
                Err(e) => return Err(DB(DBError::MongoQueryError(e))),
            };

        Ok(doc.msgs().unwrap_or_default())
    }

    async fn remove_msg(
        db: &Database,
        src_id: &str,
        msg_id: &str,
    ) -> Result<Vec<MsgModel>>
    {
        let coll = db.collection::<Self::TargetModel>(Self::COLL_NAME);

        let oid = ObjectId::from_str(src_id).map_err(|e| DBError::MongoGetOidError(e))?;

        let update_doc = doc! {
            "$pull": { "chat_msgs": { "_id": ObjectId::from_str(msg_id).map_err(|e| DBError::MongoGetOidError(e))?  } },
            "$set": { "updatedAt": Bson::DateTime(Utc::now().into()) }
        };

        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        let doc = match coll
            .find_one_and_update(doc! {"_id": oid}, update_doc, options)            
            .await{
                Ok(Some(doc)) => doc,
                Ok(None) => return Err(NotFoundError(oid.to_string())),
                Err(e) => return Err(DB(DBError::MongoQueryError(e))),
            };

        Ok(doc.msgs().unwrap_or_default())
    }

    async fn update_msg(
        db: &Database,
        src_id: &str,
        msg_id: &str,
        new_msg: &UpdateMsgSchema,
    ) -> Result<Vec<MsgModel>>
    {
        let coll = db.collection::<Self::TargetModel>(Self::COLL_NAME);

        let oid = ObjectId::from_str(src_id).map_err(|e| DBError::MongoGetOidError(e))?;

        let mut update_doc = doc! {
            "updatedAt": Bson::DateTime(Utc::now().into())
        };

        if let Some(msg_type) = &new_msg.msg_type {
            update_doc.insert("chat_msgs.$[msg].msg_type", bson::to_bson(&msg_type).map_err(|e| DBError::MongoSerializeBsonError(e))?);
        }
        if let Some(content) = &new_msg.content {
            update_doc.insert("chat_msgs.$[msg].content", bson::to_bson(&content).map_err(|e| DBError::MongoSerializeBsonError(e))?);
        }
        if let Some(booked) = &new_msg.booked {
            update_doc.insert("chat_msgs.$[msg].booked", bson::to_bson(&booked).map_err(|e| DBError::MongoSerializeBsonError(e))?);
        }

        let array_filters = bson::doc! { "msg._id": ObjectId::from_str(msg_id).map_err(|e| DBError::MongoGetOidError(e))? };
        let options = FindOneAndUpdateOptions::builder()
            .array_filters(Some(vec![array_filters]))
            .return_document(ReturnDocument::After)
            .build();

        let doc = match coll
        .find_one_and_update(doc! {"_id": oid}, doc! { "$set": update_doc }, options)
        .await
        {
            Ok(Some(doc)) => doc,
            Ok(None) => return Err(NotFoundError(oid.to_string())),
            Err(e) => return Err(DB(DBError::MongoQueryError(e))),
        };

    Ok(doc.msgs().unwrap_or_default())
    }

    async fn add_chat_to_msg(
        db: &Database,
        src_id: &str,
        msg_id: &str,
    ) -> Result<Vec<MsgModel>>
    where
        Self: Sized + DeserializeOwned,
    {
        let coll = db.collection::<Self::TargetModel>(Self::COLL_NAME);

        let oid = ObjectId::from_str(src_id).map_err(|e| DBError::MongoGetOidError(e))?;

        let new_chat = ChatModel {
            src_type: ChatType::Ask,
            msgs: None,
        };

        let update_doc = doc! {
            "$push": { "chat_msgs.$[msg].chat": bson::to_bson(&new_chat).map_err(|e| DBError::MongoSerializeBsonError(e))? },
            "$set": { "updatedAt": Bson::DateTime(Utc::now().into()) }
        };

        let array_filters = bson::doc! { "msg._id": ObjectId::from_str(msg_id).map_err(|e| DBError::MongoGetOidError(e))?};
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

