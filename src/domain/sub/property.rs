use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use serde::{Deserialize, Serialize};

use crate::domain::types::PropertyType;
use crate::interface::dto::task::req::CreatePropertyReq;
use crate::{domain::error::Result, interface::dto::category::req::UpdatePropertyReq};

use crate::domain::{
    category::CategoryModel,
    repo::base_array::{self, MongoArrayRepo},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PropertyModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub prop_type: PropertyType,
    pub options: Option<Vec<String>>,
}

impl PropertyModel {
    pub fn new(
        id: ObjectId,
        name: String,
        prop_type: PropertyType,
        options: Option<Vec<String>>,
    ) -> Self {
        let options = match prop_type {
            PropertyType::MultiSelect | PropertyType::SingleSelect => options,
            _ => None,
        };

        PropertyModel {
            id,
            name,
            prop_type,
            options,
        }
    }
}

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
        Ok(base_array::add_elem::<Self>(db, category_id, new_prop, Some("prop_type")).await?)
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
