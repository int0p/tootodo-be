
use std::time::Duration;

use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
use super::model::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateChatSchema {
    pub src_item_id: ObjectId,
    pub src_item_title: String,
    pub src_item_type: ChatType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateChatSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_item_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Duration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<MsgModel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bookmarks: Option<Vec<BookMarkModel>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilterChatSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_item_type: Option<ChatType>,
}

// msg
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMsgSchema {
    pub src_chat_id: ObjectId,
    pub src_chat_type: ChatType,
    pub msg_type: MsgType,
    pub content: String,
    pub asked: bool,
    pub booked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<MsgModel>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateMsgSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub booked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<MsgModel>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilterMsgSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_chat_type: Option<ChatType>,
}

// bookmark
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateBookMarkSchema {
    pub src_msg_id: ObjectId,
    pub src_msg_type: MsgType,
    pub src_msg_title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateBookMarkSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_msg_title: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct FilterBookMarkSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_msg_type: Option<MsgType>,
}