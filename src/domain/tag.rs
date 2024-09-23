use super::repo::base_postgre as base;
use crate::{
    domain::error::Result,
    interface::dto::tag::{
        req::{CreateTagReq, UpdateTagReq},
        res::{SingleTagRes, TagData, TagListRes, TagRes},
    },
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::{FromRow, Pool, Postgres};
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(Debug, Clone, Fields, FromRow, Serialize, Deserialize)]
pub struct TagModel {
    pub id: Uuid,
    pub user: Uuid,
    pub name: String,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

pub struct TagService;

impl base::PostgreRepo for TagService {
    const TABLE: &'static str = "tags";
    type Entity = TagModel;
    type Res = TagRes;
    fn convert_entity_to_response(entity: &Self::Entity) -> Self::Res {
        TagRes::from_entity(entity)
    }
}

impl TagService {
    pub async fn fetch_tags(db: &Pool<Postgres>, user: &Uuid) -> Result<TagListRes> {
        let res = base::fetch::<Self>(db, user).await?;
        Ok(TagListRes {
            status: "success",
            results: res.len(),
            tags: res,
        })
    }

    pub async fn create_tag(
        db: &Pool<Postgres>,
        user: &Uuid,
        body: CreateTagReq,
    ) -> Result<SingleTagRes> {
        let res = base::create::<Self, CreateTagReq>(db, user, body).await?;

        Ok(SingleTagRes {
            status: "success",
            data: TagData { tag: res },
        })
    }

    pub async fn get_tag(db: &Pool<Postgres>, user: &Uuid, id: Uuid) -> Result<SingleTagRes> {
        let res = base::get::<Self>(db, user, id).await?;
        Ok(SingleTagRes {
            status: "success",
            data: TagData { tag: res },
        })
    }

    pub async fn update_tag(
        db: &Pool<Postgres>,
        user: &Uuid,
        id: Uuid,
        body: UpdateTagReq,
    ) -> Result<()> {
        Ok(base::update::<Self, UpdateTagReq>(db, user, id, body).await?)
    }

    pub async fn delete_tag(db: &Pool<Postgres>, id: Uuid) -> Result<()> {
        // TODO: Relation table에서 tag와 관련된 데이터 삭제
        Ok(base::delete::<Self>(db, id).await?)
    }
}
// endregion: --- TaskBmc
