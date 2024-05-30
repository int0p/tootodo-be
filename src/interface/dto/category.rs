pub mod req {
    use mongodb::bson::{self};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    // Category
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateCategoryReq {
        pub name: String,
        pub color: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateCategoryReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub color: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FilterCategoryReq {
        #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
        pub user: Uuid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
}

pub mod res {
    use crate::domain::{category::CategoryModel, sub::property::PropertyModel};
    use crate::infra::types::StatusType;
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[allow(non_snake_case)]
    #[derive(Deserialize, Serialize, Debug)]
    pub struct CategoryRes {
        pub id: String,
        pub user: Uuid,
        pub name: String,
        pub color: String,
        pub status: StatusType,
        pub props: Vec<PropertyModel>,
        pub createdAt: DateTime<Utc>,
        pub updatedAt: DateTime<Utc>,
    }

    impl CategoryRes {
        pub fn from_model(category: &CategoryModel) -> Self {
            Self {
                id: category.id.to_hex(),
                user: category.user,
                name: category.name.to_owned(),
                color: category.color.to_owned(),
                props: category.props.to_owned(),
                createdAt: category.createdAt,
                updatedAt: category.updatedAt,
                status: category.status.to_owned(),
            }
        }
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
