use mongodb::Database;

use crate::interface::dto::task::req::CreatePropertyReq;
use crate::{domain::error::Result, interface::dto::category::req::UpdatePropertyReq};

use crate::domain::{
    category::{CategoryModel, PropertyModel},
    repo::base_array::{self, MongoArrayRepo},
};

pub struct PropertyService;

impl MongoArrayRepo for PropertyService {
    type CollModel = CategoryModel;
    type ElemModel = PropertyModel;
    type UpdateElemReq = UpdatePropertyReq;
    type CreateElemReq = CreatePropertyReq;
    const COLL_NAME: &'static str = "categories";
    const ARR_NAME: &'static str = "properties";
}

impl PropertyService {
    pub async fn get_property(
        db: &Database,
        category_id: &str,
        prop_id: &str,
    ) -> Result<PropertyModel> {
        let doc = base_array::get_elem::<Self>(db, category_id, prop_id).await?;
        Ok(doc)
    }

    pub async fn add_property(
        db: &Database,
        category_id: &str,
        new_prop: &CreatePropertyReq,
    ) -> Result<Vec<PropertyModel>> {
        Ok(base_array::add_elem::<Self>(db, category_id, new_prop).await?)
    }

    pub async fn fetch_properties(db: &Database, category_id: &str) -> Result<Vec<PropertyModel>> {
        Ok(base_array::fetch_elems::<Self>(db, category_id).await?)
    }

    pub async fn update_property(
        db: &Database,
        category_id: &str,
        prop_id: &str,
        new_prop: &UpdatePropertyReq,
    ) -> Result<Vec<PropertyModel>> {
        // TODO: property type변경 제한
        Ok(base_array::update_elem::<Self>(db, category_id, prop_id, new_prop).await?)
    }

    pub async fn remove_property(
        db: &Database,
        category_id: &str,
        prop_id: &str,
    ) -> Result<Vec<PropertyModel>> {
        Ok(base_array::remove_elem::<Self>(db, category_id, prop_id).await?)
    }
}
