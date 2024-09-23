pub mod req {
    use sqlb::Fields;
    use serde::{Deserialize, Serialize};
    use sqlx::FromRow;
    use uuid::Uuid;
    // TagGroup
    #[derive(Fields, Serialize, Deserialize, Debug,FromRow)]
    pub struct CreateTagGroupReq {
        pub name: String,
        pub color: String,
    }

    #[derive(Fields, Serialize, Deserialize, Debug,FromRow)]
    pub struct UpdateTagGroupReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub color: Option<String>,
    }

    #[derive( Serialize, Deserialize, Debug,FromRow)]
    pub struct FilterTagGroupReq {
        pub user: Uuid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub color: Option<String>,
    }
}

pub mod res {
    use crate::domain::tag_group::TagGroupModel;
    use serde::{Deserialize, Serialize};
    use sqlx::{Decode, FromRow};
    use uuid::Uuid;

    #[allow(non_snake_case)]
    #[derive(Deserialize, Serialize, Debug,FromRow, Decode)]
    pub struct TagGroupRes {
        pub id: String,
        pub user: Uuid,
        pub name: String,
        pub color: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tags: Option<Vec<String>>,
    }

    impl TagGroupRes {
        pub fn from_entity(tag_group: &TagGroupModel) -> Self {
            Self {
                id: tag_group.id.to_string(),
                user: tag_group.user,
                name: tag_group.name.to_owned(),
                color: tag_group.color.to_owned(),
                tags: None,
            }
        }
    }

    #[derive(Serialize, Debug, FromRow)]
    pub struct TagGroupData {
        pub tag_group: TagGroupRes,
    }

    #[derive(Serialize, Debug)]
    pub struct SingleTagGroupRes {
        pub status: &'static str,
        pub data: TagGroupData,
    }
    
    #[derive(Serialize, Debug)]
    pub struct TagGroupListRes {
        pub status: &'static str,
        pub results: usize,
        pub tag_groups: Vec<TagGroupRes>,
    }
}
