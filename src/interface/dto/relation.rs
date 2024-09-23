use serde::{Deserialize, Serialize};
use sqlb::{Field, Fields};
use sqlx::{FromRow,};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Fields, FromRow)]
pub struct CreateTagRelationReq {
    pub tag_id: Uuid,
    pub group_id: Uuid,
}
