use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatModel{
    pub src_type: ChatType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msgs: Option<Vec<MsgModel>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChatType{
    Ask,
    Event,
    Task,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MsgModel{
    pub msg_type:MsgType,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub booked:bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat:Option<Vec<ChatModel>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MsgType{
    Text,
    Ask,
    Answer,
    Image,
    File,
    Link,
    Video,
    Audio,
    Location,
}
