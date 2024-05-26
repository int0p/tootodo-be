use mongodb::Database;

use crate::{domain::error::Result, interface::dto::category::req::UpdatePropertyReq};

use super::{
    category::{CategoryModel, PropertyModel},
    repo::base_array::{self, MongoArrayRepo},
};

pub struct PropertyService;

impl MongoArrayRepo for PropertyService {
    type CollModel = CategoryModel;
    type ElemModel = PropertyModel;
    type UpdateElemReq = UpdatePropertyReq;
    const COLL_NAME: &'static str = "categories";
    const ARR_NAME: &'static str = "properties";
}

impl PropertyService {
    pub async fn get_property(
        db: &Database,
        category_id: &str,
        prop_id: &str,
    ) -> Result<PropertyModel> {
        let doc = base_array::get_elem::<PropertyService>(db, category_id, prop_id).await?;
        Ok(doc)
    }

    pub async fn add_property(
        db: &Database,
        category_id: &str,
        new_prop: &PropertyModel,
    ) -> Result<Vec<PropertyModel>> {
        let doc: CategoryModel =
            base_array::add_elem::<PropertyService>(db, category_id, new_prop).await?;
        Ok(doc.properties)
    }

    pub async fn fetch_properties(db: &Database, category_id: &str) -> Result<Vec<PropertyModel>> {
        let doc: CategoryModel =
            base_array::fetch_elems::<PropertyService>(db, category_id).await?;
        Ok(doc.properties)
    }

    pub async fn update_property(
        db: &Database,
        category_id: &str,
        prop_id: &str,
        new_prop: &UpdatePropertyReq,
    ) -> Result<Vec<PropertyModel>> {
        // TODO: property type변경 제한
        let doc: CategoryModel =
            base_array::update_elem::<PropertyService>(db, category_id, prop_id, new_prop).await?;
        Ok(doc.properties)
    }

    pub async fn remove_property(
        db: &Database,
        category_id: &str,
        prop_id: &str,
    ) -> Result<Vec<PropertyModel>> {
        let doc: CategoryModel =
            base_array::remove_elem::<PropertyService>(db, category_id, prop_id).await?;
        Ok(doc.properties)
    }
}
