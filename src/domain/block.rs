use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use serde::{Deserialize, Serialize};

use crate::domain::error::Result;

use super::repo::base_array::{self, MongoArrayRepo};
use super::task::TaskModel;
use super::types::BlockType;
use crate::interface::dto::task::req::UpdateBlockReq;

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
    const COLL_NAME: &'static str = "categories";
    const ARR_NAME: &'static str = "blocks";
}

impl BlockService {
    pub async fn get_block(db: &Database, category_id: &str, prop_id: &str) -> Result<BlockModel> {
        let doc = base_array::get_elem::<BlockService>(db, category_id, prop_id).await?;
        Ok(doc)
    }

    pub async fn add_block(
        db: &Database,
        category_id: &str,
        new_prop: &BlockModel,
    ) -> Result<Vec<BlockModel>> {
        let doc: TaskModel =
            base_array::add_elem::<BlockService>(db, category_id, new_prop).await?;
        Ok(doc.blocks)
    }

    pub async fn fetch_blocks(db: &Database, category_id: &str) -> Result<Vec<BlockModel>> {
        let doc: TaskModel = base_array::fetch_elems::<BlockService>(db, category_id).await?;
        Ok(doc.blocks)
    }

    pub async fn update_block(
        db: &Database,
        category_id: &str,
        prop_id: &str,
        new_prop: &UpdateBlockReq,
    ) -> Result<Vec<BlockModel>> {
        let doc: TaskModel =
            base_array::update_elem::<BlockService>(db, category_id, prop_id, new_prop).await?;
        Ok(doc.blocks)
    }

    pub async fn remove_block(
        db: &Database,
        category_id: &str,
        prop_id: &str,
    ) -> Result<Vec<BlockModel>> {
        let doc: TaskModel =
            base_array::remove_elem::<BlockService>(db, category_id, prop_id).await?;
        Ok(doc.blocks)
    }
}
