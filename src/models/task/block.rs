use mongodb::Database;

use crate::models::base_array::{self, MongoArrayBMC};
use crate::models::error::Result;

use super::model::{BlockModel, TaskModel};
use super::schema::UpdateBlockSchema;

pub struct BlockBMC;

impl MongoArrayBMC for BlockBMC {
    type CollModel = TaskModel;
    type ElemModel = BlockModel;
    type UpdateElemSchema = UpdateBlockSchema;
    const COLL_NAME: &'static str = "categories";
    const ARR_NAME: &'static str = "blocks";
}

impl BlockBMC {
    pub async fn get_block(db: &Database, category_id: &str, prop_id: &str) -> Result<BlockModel> {
        let doc = base_array::get_elem::<BlockBMC>(db, category_id, prop_id).await?;
        Ok(doc)
    }

    pub async fn add_block(
        db: &Database,
        category_id: &str,
        new_prop: &BlockModel,
    ) -> Result<Vec<BlockModel>> {
        let doc: TaskModel = base_array::add_elem::<BlockBMC>(db, category_id, new_prop).await?;
        Ok(doc.blocks)
    }

    pub async fn fetch_blocks(db: &Database, category_id: &str) -> Result<Vec<BlockModel>> {
        let doc: TaskModel = base_array::fetch_elems::<BlockBMC>(db, category_id).await?;
        Ok(doc.blocks)
    }

    pub async fn update_block(
        db: &Database,
        category_id: &str,
        prop_id: &str,
        new_prop: &UpdateBlockSchema,
    ) -> Result<Vec<BlockModel>> {
        let doc: TaskModel =
            base_array::update_elem::<BlockBMC>(db, category_id, prop_id, new_prop).await?;
        Ok(doc.blocks)
    }

    pub async fn remove_block(
        db: &Database,
        category_id: &str,
        prop_id: &str,
    ) -> Result<Vec<BlockModel>> {
        let doc: TaskModel = base_array::remove_elem::<BlockBMC>(db, category_id, prop_id).await?;
        Ok(doc.blocks)
    }
}
