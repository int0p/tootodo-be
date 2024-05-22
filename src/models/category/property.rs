use mongodb::Database;

use crate::models::base_array::{self, MongoArrayBMC};
use crate::models::error::Result;

use super::model::{CategoryModel, PropertyModel};
use super::schema::UpdatePropertySchema;

pub struct PropertyBMC;

impl MongoArrayBMC for PropertyBMC {
    type CollModel = CategoryModel;
    type ElemModel = PropertyModel;
    type UpdateElemSchema = UpdatePropertySchema;
    const COLL_NAME: &'static str = "categories";
    const ARR_NAME: &'static str = "properties";
}

impl PropertyBMC {
    pub async fn get_property(
        db: &Database,
        category_id: &str,
        prop_id: &str,
    ) -> Result<PropertyModel> {
        let doc = base_array::get_elem::<PropertyBMC>(db, category_id, prop_id).await?;
        Ok(doc)
    }

    pub async fn add_property(
        db: &Database,
        category_id: &str,
        new_prop: &PropertyModel,
    ) -> Result<Vec<PropertyModel>> {
        let doc: CategoryModel =
            base_array::add_elem::<PropertyBMC>(db, category_id, new_prop).await?;
        Ok(doc.properties)
    }

    pub async fn fetch_properties(db: &Database, category_id: &str) -> Result<Vec<PropertyModel>> {
        let doc: CategoryModel = base_array::fetch_elems::<PropertyBMC>(db, category_id).await?;
        Ok(doc.properties)
    }

    pub async fn update_property(
        db: &Database,
        category_id: &str,
        prop_id: &str,
        new_prop: &UpdatePropertySchema,
    ) -> Result<Vec<PropertyModel>> {
        // TODO: property type변경 제한
        let doc: CategoryModel =
            base_array::update_elem::<PropertyBMC>(db, category_id, prop_id, new_prop).await?;
        Ok(doc.properties)
    }

    pub async fn remove_property(
        db: &Database,
        category_id: &str,
        prop_id: &str,
    ) -> Result<Vec<PropertyModel>> {
        let doc: CategoryModel =
            base_array::remove_elem::<PropertyBMC>(db, category_id, prop_id).await?;
        Ok(doc.properties)
    }
}
