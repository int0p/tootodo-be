pub mod req {
    use sqlb::Fields;
    use serde::{Deserialize, Serialize};
    use sqlx::FromRow;
    use uuid::Uuid;

    #[derive(Fields, Serialize, Deserialize, Debug,FromRow)]
    pub struct CreateTagReq {
        pub name: String,
    }

    #[derive(Fields, Serialize, Deserialize, Debug,FromRow)]
    pub struct UpdateTagReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>
    }

    #[derive( Serialize, Deserialize, Debug,FromRow)]
    pub struct FilterTagReq {
        pub user: Uuid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
}

pub mod res {
    use crate::domain::tag::TagModel;
    use serde::{Deserialize, Serialize};
    use sqlx::FromRow;
    use uuid::Uuid;

    #[allow(non_snake_case)]
    #[derive(Deserialize, Serialize, Debug,FromRow)]
    pub struct TagRes {
        pub id: String,
        pub user: Uuid,
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub groups: Option<Vec<(String,String)>>,
    }

    impl TagRes {
        pub fn from_entity(tag: &TagModel) -> Self {
            Self {
                id: tag.id.to_string(),
                user: tag.user,
                name: tag.name.to_owned(),
                groups: None,
            }
        }
    }

    #[derive(Serialize, Debug)]
    pub struct TagData {
        pub tag: TagRes,
    }

    #[derive(Serialize, Debug)]
    pub struct SingleTagRes {
        pub status: &'static str,
        pub data: TagData,
    }

    #[derive(Serialize, Debug)]
    pub struct TagListRes {
        pub status: &'static str,
        pub results: usize,
        pub tags: Vec<TagRes>,
    }
}
