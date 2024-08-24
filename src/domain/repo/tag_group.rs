use sqlx::{PgPool, Result, Row};
use sea_query::{Condition, Expr, Iden, JoinType, PostgresQueryBuilder, Query};
use crate::{
   domain::error::{Error::*, Result},
   infra::db::error::Error as DBError,
   interface::dto::tag_group::{
       req::{CreateTagGroupReq, UpdateTagGroupReq},
       res::{TagGroupData, TagGroupListRes, TagGroupRes, SingleTagGroupRes},
   },
};
pub struct TagGroupRepository {
    pub pool: PgPool,
}

#[derive(Iden)]
enum TagGroup {
   Table,Id,User,Name,Color,CreatedAt,UpdatedAt
}

impl TagGroupRepository {
    pub async fn create(&self, body: &CreateTagGroupReq) -> Result<TagGroupRes> {
        let query = Query::insert()
            .into_table(TagGroup::Table)
            .columns(&[
               TagGroup::Id,
               TagGroup::User,
               TagGroup::Name,
               TagGroup::Color,
               TagGroup::CreatedAt,
               TagGroup::UpdatedAt                  
            ])
            .values_panic([
                Expr::value(body.user.into()),
                Expr::value(body.name.clone()),
                Expr::value(body.color.clone()),
                Expr::value(chrono::Utc::now()),
                Expr::value(chrono::Utc::now()),
            ])
            .build(PostgresQueryBuilder);

        let id: uuid::Uuid = sqlx::query_scalar(&query.to_string())
            .bind(body.user)
            .bind(body.name)
            .bind(body.color)
            .bind(chrono::Utc::now())
            .bind(chrono::Utc::now())
            .fetch_one(&self.pool)
            .await?;

        Ok(TagGroup {
            id,
            user: body.user,
            name: body.name.clone(),
            color: body.color.clone(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }

    pub async fn update(&self, id: &Uuid, dto: &UpdateTagGroupDto) -> Result<()> {
        let mut query = Query::update()
            .table(TagGroup::TABLE)
            .so_that(Condition::all()
                .add(Expr::col(TagGroup::ID).eq(Expr::value(id))));

        if let Some(name) = &dto.name {
            query = query.value(TagGroup::NAME, Expr::value(name));
        }

        if let Some(color) = &dto.color {
            query = query.value(TagGroup::COLOR, Expr::value(color));
        }

        query = query.value(TagGroup::UPDATED_AT, Expr::value(chrono::Utc::now()));

        sqlx::query(&query.build(PostgresQueryBuilder)).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query(&Query::delete()
            .from_table(TagGroup::TABLE)
            .so_that(Condition::all()
                .add(Expr::col(TagGroup::ID).eq(Expr::value(id))))
            .build(PostgresQueryBuilder))
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn find_by_user_and_name(&self, user: &Uuid, name: &str) -> Result<Option<TagGroup>> {
        let row = sqlx::query_as::<_, (Uuid, String, String, DateTime<Utc>, DateTime<Utc>)>(
            &Query::select()
                .column(TagGroup::ID)
                .column(TagGroup::USER)
                .column(TagGroup::NAME)
                .column(TagGroup::COLOR)
                .column(TagGroup::CREATED_AT)
                .column(TagGroup::UPDATED_AT)
                .from_table(TagGroup::TABLE)
                .so_that(Condition::all()
                    .add(Expr::col(TagGroup::USER).eq(Expr::value(user)))
                    .add(Expr::col(TagGroup::NAME).eq(Expr::value(name))))
                .build(PostgresQueryBuilder),
        )
        .fetch_optional(&self.pool)
        .await?;

        row.map(|(id, user, name, color, created_at, updated_at)| TagGroup {
            id,
            user,
            name,
            color,
            created_at,
            updated_at,
        })
    }
}