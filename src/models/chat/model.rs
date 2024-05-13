use std::time::Duration;

use chrono::prelude::*;
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatModel{
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub src_item_id: ObjectId,
    pub src_item_title:String,
    pub src_item_type:ChatType,
    pub duration:Duration,
    pub messages: Vec<MsgModel>,
    pub bookmarks: Vec<BookMarkModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChatType{
    Gpt,
    Event,
    Task,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MsgModel{
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub src_chat_id: ObjectId,
    pub src_chat_type:ChatType,
    pub msg_type:MsgType,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub asked:bool,
    pub booked:bool,
    pub replies:Vec<MsgModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MsgType{
    Text,
    Ask,
    Image,
    File,
    Link,
    Video,
    Audio,
    Location,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BookMarkModel{
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub src_msg_id: ObjectId,
    pub src_msg_type:MsgType,
    pub src_msg_title:String,
}
