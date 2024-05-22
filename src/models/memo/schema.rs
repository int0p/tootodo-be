use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Memo
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMemoSchema {
    pub user: Uuid,
    pub title: String,
    pub color: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateMemoSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
