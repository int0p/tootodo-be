use crate::infra::db::error::Error as DBError;
use sqlb::{Field, HasFields};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Pool, Postgres};
use uuid::Uuid;
pub type Result<T> = core::result::Result<T, DBError>;

pub trait PostgreRepo {
    const TABLE: &'static str;
    type Entity: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send;
    type Res;
    fn convert_entity_to_response(entity: &Self::Entity) -> Self::Res;
}

pub async fn fetch<S>(db: &Pool<Postgres>, user: &Uuid) -> Result<Vec<S::Res>>
where
    S: PostgreRepo,
{
    let entities: Vec<S::Entity> = sqlb::select()
        .table(S::TABLE)
        .columns(S::Entity::field_names())
        .and_where("user", "=", user)
        .order_by("id")
        .fetch_all(db)
        .await?;

    let mut results = Vec::new();
    for entity in entities {
        results.push(S::convert_entity_to_response(&entity));
    }

    Ok(results)
}

pub async fn create<S, Shcema>(db: &Pool<Postgres>, user: &Uuid, body: Shcema) -> Result<S::Res>
where
    S: PostgreRepo,
    Shcema: HasFields,
{
    let mut fields = body.not_none_fields();
    fields.push(Field::from(("user", user)));

    let (id,) = sqlb::insert()
        .table(S::TABLE)
        .data(fields)
        .returning(&["id"])
        .fetch_one::<_, (Uuid,)>(db)
        .await?;

    let entity = get::<S>(db, user, id).await?;

    Ok(entity)
}

pub async fn get<S>(db: &Pool<Postgres>, user: &Uuid, id: Uuid) -> Result<S::Res>
where
    S: PostgreRepo,
{
    let entity: S::Entity = sqlb::select()
        .table(S::TABLE)
        .columns(S::Entity::field_names())
        .and_where("id", "=", id)
        .and_where("user", "=", user)
        .fetch_optional(db)
        .await?
        .ok_or(DBError::EntityNotFound {
            entity: S::TABLE,
            id,
        })?;

    Ok(S::convert_entity_to_response(&entity))
}

pub async fn update<S, Schema>(
    db: &Pool<Postgres>,
    user: &Uuid,
    id: Uuid,
    body: Schema,
) -> Result<()>
where
    S: PostgreRepo,
    Schema: HasFields,
{
    let fields = body.not_none_fields();
    let count = sqlb::update()
        .table(S::TABLE)
        .and_where("id", "=", id)
        .and_where("user", "=", user)
        .data(fields)
        .exec(db)
        .await?;

    if count == 0 {
        Err(DBError::EntityNotFound {
            entity: S::TABLE,
            id,
        })
    } else {
        Ok(())
    }
}

pub async fn delete<S>(db: &Pool<Postgres>, id: Uuid) -> Result<()>
where
    S: PostgreRepo,
{
    let count = sqlb::delete()
        .table(S::TABLE)
        .and_where("id", "=", id)
        .exec(db)
        .await?;

    if count == 0 {
        Err(DBError::EntityNotFound {
            entity: S::TABLE,
            id,
        })
    } else {
        Ok(())
    }
}

