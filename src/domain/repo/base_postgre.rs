use crate::domain::error::{Error, Result};
use crate::infra::db::error::Error as DBError;
use modql::field::HasFields;
use sea_query::{Condition, Expr, Iden, IntoIden, PostgresQueryBuilder, Query, TableRef};
use sea_query_binder::SqlxBinder;
use serde::Serialize;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Pool, Postgres};
use uuid::Uuid;

use modql::filter::{FilterGroups, ListOptions};
use modql::SIden;

const LIST_LIMIT_DEFAULT: i64 = 300;
const LIST_LIMIT_MAX: i64 = 1000;

#[derive(Iden)]
pub enum CommonIden {
    Id,
    User,
}

pub trait PostgreRepo {
    const TABLE: &'static str;
    type ModelResponse;

    fn table_ref() -> TableRef {
        TableRef::Table(SIden(Self::TABLE).into_iden())
    }
}

// S: Service
pub async fn fetch<S, F>(
    db: &Pool<Postgres>,
    filters: Option<F>,
    list_options: Option<ListOptions>,
	 user: &Uuid,
) -> Result<Vec<S::ModelResponse>>
where
    S: PostgreRepo,
    F: Into<FilterGroups>,
    S::ModelResponse: for<'r> FromRow<'r, PgRow> + Unpin + Send,
    S::ModelResponse: HasFields,
{
	//TODO: filter에 user추가.
    // -- Build query
    let mut query = Query::select();
    query.from(S::table_ref()).columns(S::ModelResponse::field_column_refs());

    // condition from filter
    if let Some(filters) = filters {
        let filters: FilterGroups = filters.into();
        let cond: Condition = filters.try_into().map_err(|e| DBError::ModqlIntoSea(e))?;
        query.cond_where(cond);
    }

    // list options
    let list_options = finalize_list_options(list_options)?;
    list_options.apply_to_sea_query(&mut query);

    // -- Exec query
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let entities = sqlx::query_as_with::<_, S::ModelResponse, _>(&sql, values)
        .fetch_all(db)
        .await
        .map_err(|e| DBError::Sqlx(e))?;

    Ok(entities)
}

pub async fn create<S, Schema>(db: &Pool<Postgres>, body: Schema, user: &Uuid) -> Result<S::ModelResponse>
where
    S: PostgreRepo,
    Schema: Serialize + HasFields,
{
    // -- Prep data
    let fields = body.not_none_fields();
    let (mut columns, mut sea_values) = fields.for_sea_insert();

    // Add user info
    columns.push(CommonIden::User.into_iden());
    sea_values.push(Expr::value(user.clone()));

    // -- Build query
    let mut query = Query::insert();
    query
        .into_table(S::table_ref())
        .columns(columns)
        .values(sea_values)
        .map_err(|e| DBError::SeaQuery(e))?
        .returning(Query::returning().columns([CommonIden::Id]));

    // -- Exec query
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let (id,) = sqlx::query_as_with::<_, (i64,), _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| DBError::Sqlx(e))?;

    // TODO: 생성한 tag group db에서 찾고, 반환
    Ok(id)
}

pub async fn get<S>(db: &Pool<Postgres>, id: i64, user: &Uuid) -> Result<S::ModelResponse>
where
    S: PostgreRepo,
    S::ModelResponse: for<'r> FromRow<'r, PgRow> + Unpin + Send,
    S::ModelResponse: HasFields,
{
    // -- Build query
    let mut query = Query::select();
    query
        .from(S::table_ref())
        .columns(S::ModelResponse::field_column_refs())
        .and_where(Expr::col(CommonIden::Id).eq(id))
        .and_where(Expr::col(CommonIden::User).eq(user.clone()));

    // -- Exec query
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let entity = sqlx::query_as_with::<_, S::ModelResponse, _>(&sql, values)
        .fetch_optional(db)
        .await
        .map_err(|e| DBError::Sqlx(e))?
        .ok_or(Error::EntityNotFound {
            entity: S::TABLE,
            id,
        })?;

    Ok(entity)
}

pub async fn update<S, Schema>(
    db: &Pool<Postgres>,
    id: i64,
    body: Schema,
    user: &Uuid,
) -> Result<S::ModelResponse>
where
    S: PostgreRepo,
    Schema: for<'r> FromRow<'r, PgRow> + Unpin + Send,
    Schema: HasFields,
{
    // -- Prep data
    let fields = body.not_none_fields();
    let fields = fields.for_sea_update();

    // -- Build query
    let mut query = Query::update();
    query
        .table(S::table_ref())
        .values(fields)
        .and_where(Expr::col(CommonIden::Id).eq(id))
        .and_where(Expr::col(CommonIden::User).eq(user.clone()));

    // -- Exec query
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let count = sqlx::query_with(&sql, values)
        .execute(db)
        .await
        .map_err(|e| DBError::Sqlx(e))?
        .rows_affected();

    // -- Check result
    if count == 0 {
        Err(Error::EntityNotFound {
            entity: S::TABLE,
            id,
        })
    } else {
        Ok(())
    }
}

pub async fn delete<S>(db: &Pool<Postgres>, id: i64) -> Result<()>
where
    S: PostgreRepo,
{
    // -- Build query
    let mut query = Query::delete();
    query
        .from_table(S::table_ref())
        .and_where(Expr::col(CommonIden::Id).eq(id));

    // -- Exec query
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let count = sqlx::query_with(&sql, values)
        .execute(db)
        .await
        .map_err(|e| DBError::Sqlx(e))?
        .rows_affected();

    // -- Check result
    if count == 0 {
        Err(Error::EntityNotFound {
            entity: S::TABLE,
            id,
        })
    } else {
        Ok(())
    }
}

// TODO: 왜씀
pub fn finalize_list_options(list_options: Option<ListOptions>) -> Result<ListOptions> {
    // When Some, validate limit
    if let Some(mut list_options) = list_options {
        // Validate the limit.
        if let Some(limit) = list_options.limit {
            if limit > LIST_LIMIT_MAX {
                return Err(Error::ListLimitOverMax {
                    max: LIST_LIMIT_MAX,
                    actual: limit,
                });
            }
        }
        // Set the default limit if no limit
        else {
            list_options.limit = Some(LIST_LIMIT_DEFAULT);
        }
        Ok(list_options)
    }
    // When None, return default
    else {
        Ok(ListOptions {
            limit: Some(LIST_LIMIT_DEFAULT),
            offset: None,
            order_bys: Some("id".into()),
        })
    }
}
