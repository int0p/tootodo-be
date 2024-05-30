use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use serde::{Deserialize, Serialize};

use crate::infra::types::PropertyType;
use crate::interface::dto::sub::property::{req::*, res::*};

use crate::domain::{
    category::CategoryModel,
    error::Result,
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
    type ElemRes = PropertyRes;

    const COLL_NAME: &'static str = "categories";
    const ARR_NAME: &'static str = "props";

    fn convert_doc_to_response(doc: &PropertyModel) -> Result<Self::ElemRes> {
        Ok(PropertyRes::from_model(doc))
    }
}

impl PropertyService {
    pub async fn get_property(
        db: &Database,
        category_id: &str,
        prop_id: &str,
    ) -> Result<SinglePropertyRes> {
        let result = base_array::get_elem::<Self>(db, category_id, prop_id).await?;
        Ok(SinglePropertyRes {
            status: "success",
            data: PropertyData { prop: result },
        })
    }

    pub async fn add_property(
        db: &Database,
        category_id: &str,
        new_prop: &CreatePropertyReq,
    ) -> Result<SinglePropertyRes> {
        let result =
            base_array::add_elem::<Self>(db, category_id, new_prop, Some("prop_type")).await?;
        Ok(SinglePropertyRes {
            status: "success",
            data: PropertyData { prop: result },
        })
    }

    pub async fn fetch_properties(db: &Database, category_id: &str) -> Result<PropertyListRes> {
        let results = base_array::fetch_elems::<Self>(db, category_id, None, None).await?;
        Ok(PropertyListRes {
            status: "success",
            results: results.len(),
            props: results,
        })
    }

    pub async fn update_property(
        db: &Database,
        category_id: &str,
        prop_id: &str,
        new_prop: &UpdatePropertyReq,
    ) -> Result<SinglePropertyRes> {
        let result = base_array::update_elem::<Self>(db, category_id, prop_id, new_prop).await?;
        Ok(SinglePropertyRes {
            status: "success",
            data: PropertyData { prop: result },
        })
    }

    pub async fn remove_property(db: &Database, category_id: &str, prop_id: &str) -> Result<()> {
        base_array::remove_elem::<Self>(db, category_id, prop_id).await?;
        Ok(())
    }
}
