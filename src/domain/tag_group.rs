use super::repo::base_postgre as base;
use crate::interface::dto::tag_group::req::FilterTagGroupReq;
use crate::{
    domain::error::{Error::*, Result},
    infra::db::error::Error as DBError,
    interface::dto::tag_group::{
        req::{CreateTagGroupReq, UpdateTagGroupReq},
        res::{SingleTagGroupRes, TagGroupData, TagGroupListRes, TagGroupRes},
    },
};
use chrono::{DateTime, Utc};
use modql::field::Fields;
use modql::filter::ListOptions;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone, Fields, FromRow)]
pub struct TagGroupModel {
    #[serde(rename = "_id")]
    pub id: Uuid,
    pub user: Uuid,
    pub name: String,
    pub color: String,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct TagGroupService;

impl base::PostgreRepo for TagGroupService {
    type ModelResponse = TagGroupRes;
    const TABLE: &'static str = "tag_group";
}

impl TagGroupService {
    //mongodb에서 tag_group를 가져옴.
    pub async fn fetch_tag_groups(
        db: &Pool<Postgres>,
        list_options: Option<ListOptions>,
        user: &Uuid,
    ) -> Result<TagGroupListRes> {
        let tag_groups_result =
            base::fetch::<Self, FilterTagGroupReq>(db, None, list_options, user)
                .await
                .expect("tag_group 응답을 받아오지 못했습니다.");

        Ok(TagGroupListRes {
            status: "success",
            results: tag_groups_result.len(),
            tag_groups: tag_groups_result,
        })
    }

    pub async fn create_tag_group(
        db: &Pool<Postgres>,
        body: CreateTagGroupReq,
        user: &Uuid,
    ) -> Result<SingleTagGroupRes> {
        let tag_group_result = base::create::<Self, CreateTagGroupReq>(db, body, user)
            .await
            .expect("tag_group 생성에 실패했습니다.");

        Ok(SingleTagGroupRes {
            status: "success",
            data: TagGroupData {
                tag_group: tag_group_result,
            },
        })
    }

    pub async fn get_tag_group(
        db: &Pool<Postgres>,
        id: i64,
        user: &Uuid,
    ) -> Result<SingleTagGroupRes> {
        let tag_group_result = base::get::<Self>(db, id, user)
            .await
            .expect("tag_group를 가져오는데 실패했습니다.");

        Ok(SingleTagGroupRes {
            status: "success",
            data: TagGroupData {
                tag_group: tag_group_result,
            },
        })
    }

    pub async fn update_tag_group(
        db: &Pool<Postgres>,
        id: i64,
        body: UpdateTagGroupReq, //color, name
        user: &Uuid,
    ) -> Result<SingleTagGroupRes> {
        let tag_group_result = base::update::<Self, UpdateTagGroupReq>(db, id, body, user)
            .await
            .expect("tag_group 업데이트에 실패했습니다.");

        Ok(SingleTagGroupRes {
            status: "success",
            data: TagGroupData {
                tag_group: tag_group_result,
            },
        })
    }

    pub async fn delete_tag_group(db: &Pool<Postgres>, id: i64) -> Result<()> {
        base::delete::<Self>(db, id).await
    }
}
