use super::model::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateChatSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_type: Option<ChatType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilterChatSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_item_type: Option<ChatType>,
}

// msg
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMsgSchema {
    pub msg_type: MsgType,
    pub content: String,
    pub booked: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateMsgSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<MsgType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub booked: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilterMsgSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_chat_type: Option<ChatType>,
}
