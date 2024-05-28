pub mod req {
    use serde::{Deserialize, Serialize};

    use crate::domain::types::{PropertyType, StatusType};

    #[derive(Deserialize, Debug, Default)]
    pub struct FilterOptions {
        pub page: Option<usize>,
        pub limit: Option<usize>,
    }

    // Category
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateCategoryReq {
        pub name: String,
        pub color: String,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateCategoryReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub color: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<StatusType>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdatePropertyReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prop_type: Option<PropertyType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub options: Option<Vec<String>>,
    }
}

pub mod res {
    use crate::domain::{sub::property::PropertyModel, types::StatusType};
    use chrono::{DateTime, Utc};
    use serde::Serialize;
    use uuid::Uuid;

    #[allow(non_snake_case)]
    #[derive(Serialize, Debug)]
    pub struct CategoryRes {
        pub id: String,
        pub user: Uuid,
        pub name: String,
        pub color: String,
        pub status: StatusType,
        pub properties: Vec<PropertyModel>,
        pub createdAt: DateTime<Utc>,
        pub updatedAt: DateTime<Utc>,
    }

    #[derive(Serialize, Debug)]
    pub struct CategoryData {
        pub category: CategoryRes,
    }

    #[derive(Serialize, Debug)]
    pub struct SingleCategoryRes {
        pub status: &'static str,
        pub data: CategoryData,
    }

    #[derive(Serialize, Debug)]
    pub struct CategoryListRes {
        pub status: &'static str,
        pub results: usize,
        pub categories: Vec<CategoryRes>,
    }
}
