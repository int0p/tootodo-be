use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use serde::{Deserialize, Serialize};

use crate::domain::error::Result;

use crate::domain::repo::base_array::{self, MongoArrayRepo};
use crate::domain::task::TaskModel;
use crate::domain::types::BlockType;
use crate::interface::dto::task::req::{CreateBlockReq, UpdateBlockReq};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub src_task_id: ObjectId,
    pub block_type: BlockType,
    pub body: String,
}

impl BlockModel {
    pub fn new(src_id: ObjectId) -> Self {
        Self {
            id: ObjectId::new(),
            src_task_id: src_id,
            block_type: BlockType::Editor,
            body: "".to_string(),
        }
    }
}

pub struct BlockService;

impl MongoArrayRepo for BlockService {
    type CollModel = TaskModel;
    type ElemModel = BlockModel;
    type UpdateElemReq = UpdateBlockReq;
    type CreateElemReq = CreateBlockReq;
    const COLL_NAME: &'static str = "categories";
    const ARR_NAME: &'static str = "blocks";
}

impl BlockService {
    pub async fn get_block(db: &Database, category_id: &str, prop_id: &str) -> Result<BlockModel> {
        Ok(base_array::get_elem::<BlockService>(db, category_id, prop_id).await?)
    }

    pub async fn add_block(
        db: &Database,
        category_id: &str,
        new_prop: &CreateBlockReq,
    ) -> Result<Vec<BlockModel>> {
        Ok(base_array::add_elem::<BlockService>(db, category_id, new_prop, None).await?)
    }

    pub async fn fetch_blocks(db: &Database, category_id: &str) -> Result<Vec<BlockModel>> {
        Ok(base_array::fetch_elems::<BlockService>(db, category_id).await?)
    }

    pub async fn update_block(
        db: &Database,
        category_id: &str,
        prop_id: &str,
        new_prop: &UpdateBlockReq,
    ) -> Result<Vec<BlockModel>> {
        Ok(base_array::update_elem::<BlockService>(db, category_id, prop_id, new_prop).await?)
    }

    pub async fn remove_block(
        db: &Database,
        category_id: &str,
        prop_id: &str,
    ) -> Result<Vec<BlockModel>> {
        Ok(base_array::remove_elem::<BlockService>(db, category_id, prop_id).await?)
    }
}
