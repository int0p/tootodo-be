use super::repo::base_postgre as base;
use crate::{
    domain::error::Result,
    interface::dto::tag_group::{
        req::{CreateTagGroupReq, UpdateTagGroupReq},
        res::{SingleTagGroupRes, TagGroupData, TagGroupListRes, TagGroupRes},
    },
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::{FromRow, Pool, Postgres};
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Fields, FromRow)]
pub struct TagGroupModel {
    pub id: Uuid,
    pub user: Uuid,
    pub name: String,
    pub color: String,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

pub struct TagGroupService;

impl base::PostgreRepo for TagGroupService {
    const TABLE: &'static str = "tag_groups";
    type Entity = TagGroupModel;
    type Res = TagGroupRes;
    fn convert_entity_to_response(entity: &Self::Entity) -> Self::Res {
        TagGroupRes::from_entity(entity)
    }
}

impl TagGroupService {
    pub async fn fetch_groups(db: &Pool<Postgres>, user: &Uuid) -> Result<TagGroupListRes> {
        let res = base::fetch::<Self>(db, user).await?;
        Ok(TagGroupListRes {
            status: "success",
            results: res.len(),
            tag_groups: res,
        })
    }

    pub async fn create_group(
        db: &Pool<Postgres>,
        user: &Uuid,
        body: CreateTagGroupReq,
    ) -> Result<SingleTagGroupRes> {
        let res = base::create::<Self, CreateTagGroupReq>(db, user, body).await?;

        Ok(SingleTagGroupRes {
            status: "success",
            data: TagGroupData { tag_group: res },
        })
    }

    pub async fn get_group(db: &Pool<Postgres>, user: &Uuid, id: Uuid) -> Result<SingleTagGroupRes> {
        let res = base::get::<Self>(db, user, id).await?;
        Ok(SingleTagGroupRes {
            status: "success",
            data: TagGroupData { tag_group: res },
        })
    }

    pub async fn update_group(
        db: &Pool<Postgres>,
        user: &Uuid,
        id: Uuid,
        body: UpdateTagGroupReq,
    ) -> Result<()> {
        Ok(base::update::<Self, UpdateTagGroupReq>(db, user, id, body).await?)
    }

    pub async fn delete_group(db: &Pool<Postgres>, id: Uuid) -> Result<()> {
        Ok(base::delete::<Self>(db, id).await?)
    }

    
}
