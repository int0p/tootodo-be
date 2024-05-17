use chrono::{DateTime, Utc};
use std::time::Duration;
use serde::Serialize;
use uuid::Uuid;
use super::model::*;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct ChatResponse {
    pub id: String,
    pub src_type:ChatType,
    pub duration:Duration,
    pub messages: Vec<MsgModel>,
}


#[derive(Serialize, Debug, Clone)]
pub struct MsgResponse{
    pub msg_type:MsgType,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub booked:bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat:Option<Vec<ChatModel>>,
}

#[derive(Serialize, Debug)]
pub struct ChatData {    
    pub chat: ChatResponse,
}

#[derive(Serialize, Debug)]
pub struct SingleChatResponse {
    pub status: &'static str,
    pub data: ChatData,
}

#[derive(Serialize, Debug)]
pub struct ChatListResponse {
    pub status: &'static str,
    pub results: usize,
    pub chats: Vec<ChatResponse>,
}