use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use serde::{Deserialize, Serialize};

use crate::domain::error::Result;

use crate::domain::repo::base_array::{self, MongoArrayRepo};
use crate::domain::task::TaskModel;
use crate::infra::types::BlockType;
use crate::interface::dto::sub::task_block::req::{CreateBlockReq, UpdateBlockReq};
use crate::interface::dto::sub::task_block::res::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub src_task_id: ObjectId,
    pub block_type: BlockType,
    pub body: String,
}

impl BlockModel {
    pub fn new_from(src_id: ObjectId) -> Self {
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
    type ElemRes = BlockRes;

    const COLL_NAME: &'static str = "tasks";
    const ARR_NAME: &'static str = "blocks";

    fn convert_doc_to_response(doc: &BlockModel) -> Result<Self::ElemRes> {
        Ok(BlockRes::from_model(doc))
    }
}

impl BlockService {
    pub async fn get_block(
        db: &Database,
        category_id: &str,
        prop_id: &str,
    ) -> Result<SingleBlockRes> {
        let result = base_array::get_elem::<Self>(db, category_id, prop_id).await?;
        Ok(SingleBlockRes {
            status: "success",
            data: BlockData { block: result },
        })
    }

    pub async fn add_block(
        db: &Database,
        category_id: &str,
        new_prop: &CreateBlockReq,
    ) -> Result<SingleBlockRes> {
        let result = base_array::add_elem::<Self>(db, category_id, new_prop, None).await?;
        Ok(SingleBlockRes {
            status: "success",
            data: BlockData { block: result },
        })
    }

    pub async fn fetch_blocks(db: &Database, category_id: &str) -> Result<BlockListRes> {
        let results = base_array::fetch_elems::<Self>(db, category_id, None, None).await?;
        Ok(BlockListRes {
            status: "success",
            results: results.len(),
            blocks: results,
        })
    }

    pub async fn update_block(
        db: &Database,
        category_id: &str,
        prop_id: &str,
        new_prop: &UpdateBlockReq,
    ) -> Result<SingleBlockRes> {
        let result = base_array::update_elem::<Self>(db, category_id, prop_id, new_prop).await?;
        Ok(SingleBlockRes {
            status: "success",
            data: BlockData { block: result },
        })
    }

    pub async fn remove_block(db: &Database, category_id: &str, prop_id: &str) -> Result<()> {
        base_array::remove_elem::<Self>(db, category_id, prop_id).await?;
        Ok(())
    }
}
