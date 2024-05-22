use serde::{Deserialize, Serialize};

use super::model::*;

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

// Category
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCategorySchema {
    pub name: String,
    pub color: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCategorySchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePropertySchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prop_type: Option<PropertyType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}
