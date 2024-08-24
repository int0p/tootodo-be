pub mod req {
    use modql::field::Fields;
    use modql::filter::FilterNodes;
    use mongodb::bson::{self};
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

    #[derive(FilterNodes, Serialize, Deserialize, Debug,FromRow)]
    pub struct FilterTagGroupReq {
        #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
        pub user: Uuid,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
}

pub mod res {
    use crate::domain::tag_group::TagGroupModel;
    use chrono::{DateTime, Utc};
    use modql::field::Fields;
    use serde::{Deserialize, Serialize};
    use sqlx::FromRow;
    use uuid::Uuid;

    #[allow(non_snake_case)]
    #[derive(Deserialize, Serialize, Debug,Fields,FromRow)]
    pub struct TagGroupRes {
        pub id: String,
        pub user: Uuid,
        pub name: String,
        pub color: String,
        pub createdAt: DateTime<Utc>,
        pub updatedAt: DateTime<Utc>,
    }

    impl TagGroupRes {
        pub fn from_model(tag_group: &TagGroupModel) -> Self {
            Self {
                id: tag_group.id.to_string(),
                user: tag_group.user,
                name: tag_group.name.to_owned(),
                color: tag_group.color.to_owned(),
                createdAt: tag_group.createdAt,
                updatedAt: tag_group.updatedAt,
            }
        }
    }

    #[derive(Serialize, Debug)]
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
