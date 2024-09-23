use crate::{
    domain::error::{Error::*, Result},
    infra::db::error::Error as DBError,
    interface::dto::{
        relation::CreateTagRelationReq, tag::res::{SingleTagRes, TagData, TagListRes, TagRes}, tag_group::res::{SingleTagGroupRes, TagGroupData, TagGroupListRes, TagGroupRes}
    },
};
use serde::{Deserialize, Serialize};
use sqlb::HasFields;
use sqlb::{Field, Fields};
use sqlx::{FromRow, Pool, Postgres};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Fields, FromRow)]
pub struct TagRelation {
    pub user: Uuid,
    pub tag_id: Uuid,
    pub group_id: Uuid,
}

trait Relation {
    const TABLE: &'static str;
}

pub struct TagRelationService;

impl Relation for TagRelationService {
    const TABLE: &'static str = "tag_relations";
}

impl TagRelationService {
    // group
    pub async fn fetch_groups_with_tags(
        db: &Pool<Postgres>,
        user: &Uuid,
    ) -> Result<TagGroupListRes> {
        let query = r#"
            SELECT tg.id, tg.user, tg.name, tg.color, 
                ARRAY_AGG(t.name) AS tags
            FROM tag_groups tg
            LEFT JOIN tag_relation tr ON tg.id = tr.group_id
            LEFT JOIN tags t ON tr.tag_id = t.id  
            WHERE tg.user = $1
            GROUP BY tg.id
        "#;

        let res = sqlx::query_as::<_, TagGroupRes>(query)
            .bind(user)
            .fetch_all(db)
            .await
            .map_err(DBError::from)?
            .into_iter()
            .map(TagGroupRes::from)
            .collect::<Vec<_>>();

        Ok(TagGroupListRes {
            status: "success",
            results: res.len(),
            tag_groups: res,
        })
    }

    pub async fn get_group_with_tags(
        db: &Pool<Postgres>,
        user: &Uuid,
        id: Uuid,
    ) -> Result<SingleTagGroupRes> {
        let query = r#"
            SELECT tg.id, tg.user, tg.name, tg.color, 
                ARRAY_AGG(t.name) AS tags
            FROM tag_groups tg
            LEFT JOIN tag_relation tr ON tg.id = tr.group_id
            LEFT JOIN tags t ON tr.tag_id = t.id
            WHERE tg.user = $1
                AND tg.id = $2
        "#;

        let res = sqlx::query_as::<Postgres, TagGroupRes>(query)
            .bind(user)
            .bind(id)
            .fetch_one(db)
            .await
            .map_err(DBError::from)?;

        Ok(SingleTagGroupRes {
            status: "success",
            data: TagGroupData { tag_group: res },
        })
    }

    pub async fn add_tag_to_group(
        db: &Pool<Postgres>,
        user: &Uuid,
        body: CreateTagRelationReq,
    ) -> Result<SingleTagGroupRes> {
        let mut fields = body.not_none_fields();
        fields.push(Field::from(("user", user)));

        let (id,) = sqlb::insert()
            .table(Self::TABLE)
            .data(fields)
            .returning(&["group_id"])
            .fetch_one::<_, (Uuid,)>(db)
            .await
            .map_err(DBError::from)?;

        Ok(Self::get_group_with_tags(db, user, id).await?)
    }

    // tag
    pub async fn fetch_tags_with_groups(db: &Pool<Postgres>, user: &Uuid) -> Result<TagListRes> {
        let query = r#"
            SELECT t.id, t.name,
                ARRAY_AGG(ROW(tg.name, tg.color)) AS groups
            FROM tags t
            LEFT JOIN tag_relation tr ON tg.id = tr.group_id
            LEFT JOIN tag_groups tg ON tr.tag_id = tg.id
            WHERE t.user = $1
            GROUP BY t.id, tg.user, tg.name, tg.color
        "#;

        let res = sqlx::query_as::<_, TagRes>(query)
            .bind(user)
            .fetch_all(db)
            .await
            .map_err(DBError::from)?
            .into_iter()
            .map(TagRes::from)
            .collect::<Vec<_>>();

        Ok(TagListRes {
            status: "success",
            results: res.len(),
            tags: res,
        })
    }

    
    pub async fn get_tag_with_groups(
        db: &Pool<Postgres>,
        user: &Uuid,
        id: Uuid,
    ) -> Result<SingleTagRes> {
        let query = r#"
            SELECT t.id, t.name,
                ARRAY_AGG(ROW(tg.name, tg.color)) AS groups
            FROM tags t
            LEFT JOIN tag_relation tr ON tg.id = tr.group_id
            LEFT JOIN tag_groups tg ON tr.tag_id = tg.id
            WHERE t.user = $1
                AND t.id = $2
        "#;

        let res = sqlx::query_as::<Postgres, TagRes>(query)
            .bind(user)
            .bind(id)
            .fetch_one(db)
            .await
            .map_err(DBError::from)?;

        Ok(SingleTagRes {
            status: "success",
            data: TagData { tag: res },
        })
    }   

    pub async fn assign_group(
        db: &Pool<Postgres>,
        user: &Uuid,
        body: CreateTagRelationReq,
    ) -> Result<SingleTagRes> {
        let mut fields = body.not_none_fields();
        fields.push(Field::from(("user", user)));

        let (id,) = sqlb::insert()
            .table(Self::TABLE)
            .data(fields)
            .returning(&["tag_id"])
            .fetch_one::<_, (Uuid,)>(db)
            .await
            .map_err(DBError::from)?;

        Ok(Self::get_tag_with_groups(db, user, id).await?)
    }

    /// 입력받은 tag_id혹은 group_id와 관련된 row 제거
    pub async fn delete_relation(db: &Pool<Postgres>, id: Uuid) -> Result<()> {
        let count = sqlb::delete()
            .table(Self::TABLE)
            .and_where("id", "=", id)
            .exec(db)
            .await
            .map_err(DBError::from)?;

        if count == 0 {
            Err(DB(DBError::EntityNotFound {
                entity: Self::TABLE,
                id,
            }))
        } else {
            Ok(())
        }
    }
}
