use chrono::{DateTime,  Utc};

use crate::domain::note::NoteModel;

use super::
    note_block::BlockModel
;


use mongodb::{
   bson::{self, doc, oid::ObjectId},
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageModel{
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub user: Uuid,
    pub title: String,
    pub connected_note: ObjectId,
    pub contents: Vec<BlockModel>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>,
}

impl PageModel {
    pub fn new_page(original_note: &NoteModel) -> Self {
        Self {
            id: ObjectId::new(),
            user: original_note.user,
            title: "New Subnote".to_string(),
            connected_note: original_note.id.clone(),
            contents: Vec::new(),
            createdAt: Utc::now(),
            updatedAt: Utc::now(),
        }
    }
}
