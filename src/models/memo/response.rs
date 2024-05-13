use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct MemoResponse {
    pub id: String,
    pub user:Uuid,
    pub title: String,
    pub content: String,
    pub color: String,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct MemoData {    
    pub memo: MemoResponse,
}

#[derive(Serialize, Debug)]
pub struct SingleMemoResponse {
    pub status: &'static str,
    pub data: MemoData,
}

#[derive(Serialize, Debug)]
pub struct MemoListResponse {
    pub status: &'static str,
    pub results: usize,
    pub memos: Vec<MemoResponse>,
}