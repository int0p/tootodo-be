use chrono::{DateTime, NaiveDate, Utc};
use crate::models::chat::model::{ChatModel, ChatType, MsgModel};
use std::time::Duration;
use serde::Serialize;
use uuid::Uuid;

use super::model::PropertyModel;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct CategoryResponse {
    pub id: String,
    pub user:Uuid,
    pub name:String,
    pub color:String,
    pub properties: Vec<PropertyModel>,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct CategoryData {    
    pub category: CategoryResponse,
}

#[derive(Serialize, Debug)]
pub struct SingleCategoryResponse {
    pub status: &'static str,
    pub data: CategoryData,
}

#[derive(Serialize, Debug)]
pub struct CategoryListResponse {
    pub status: &'static str,
    pub results: usize,
    pub categories: Vec<CategoryResponse>,
}