use mongodb::bson::oid::ObjectId;
use mongodb::bson::Document;
use mongodb::Database;
use serde::{Deserialize, Serialize};

use crate::domain::error::{Error::*, Result};

use crate::domain::repo::base_array::{self, MongoArrayRepo};
use crate::domain::task::TaskModel;
use crate::infra::types::{PropValueType, PropertyType};
use crate::interface::dto::sub::task_propV::req::*;
use crate::interface::dto::sub::task_propV::res::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PropValueModel {
    pub prop_id: ObjectId,
    pub prop_name: String,
    pub prop_type: PropertyType,

    pub values: Option<PropValueType>,
}

impl PropValueModel {
    pub fn new(
        prop_id: ObjectId,
        prop_name: String,
        prop_type: PropertyType,
        value: PropValueType,
    ) -> Result<Self> {
        let value =
            match (&prop_type, &value) {
                (PropertyType::MultiSelect, PropValueType::Multiple(_))
                | (PropertyType::SingleSelect, PropValueType::Multiple(_)) => Some(value),
                (PropertyType::MultiSelect, PropValueType::Single(_))
                | (PropertyType::SingleSelect, PropValueType::Single(_)) => {
                    return Err(TypedError(
                        "MultiSelect or SingleSelect types must have Multiple(Vec<String>) value"
                            .to_string(),
                    ))
                }
                (_, PropValueType::Single(_)) => Some(value),
                (_, PropValueType::Multiple(_)) => return Err(TypedError(
                    "Only MultiSelect or SingleSelect types can have Multiple(Vec<String>) value"
                        .to_string(),
                )),
            };

        Ok(Self {
            prop_id,
            prop_name,
            prop_type,
            values: value,
        })
    }
}

pub struct PropValueService;

impl MongoArrayRepo for PropValueService {
    type CollModel = TaskModel;
    type ElemModel = PropValueModel;
    type UpdateElemReq = UpdatePropValueReq;
    type CreateElemReq = CreatePropValueReq;
    type ElemRes = PropValueRes;

    const COLL_NAME: &'static str = "categories";
    const ARR_NAME: &'static str = "prop_values";

    fn convert_doc_to_response(doc: &PropValueModel) -> Result<Self::ElemRes> {
        Ok(PropValueRes::from_model(doc))
    }

    fn create_doc(body: &CreatePropValueReq) -> Result<Document> {
        todo!()
    }
}

impl PropValueService {
    pub async fn get_propV(
        db: &Database,
        category_id: &str,
        prop_id: &str,
    ) -> Result<SinglePropValueRes> {
        let result = base_array::get_elem::<Self>(db, category_id, prop_id).await?;
        Ok(SinglePropValueRes {
            status: "success",
            data: PropValueData { propV: result },
        })
    }

    pub async fn add_propV(
        db: &Database,
        category_id: &str,
        mut new_propV: CreatePropValueReq,
    ) -> Result<SinglePropValueRes> {
        new_propV.value = validate_value(&new_propV.prop_type, &new_propV.value)?;

        let result = base_array::add_elem::<Self>(db, category_id, &new_propV, None).await?;
        Ok(SinglePropValueRes {
            status: "success",
            data: PropValueData { propV: result },
        })
    }

    pub async fn fetch_propVs(db: &Database, category_id: &str) -> Result<PropValueListRes> {
        let results = base_array::fetch_elems::<Self>(db, category_id, None, None).await?;
        Ok(PropValueListRes {
            status: "success",
            results: results.len(),
            propVs: results,
        })
    }

    pub async fn update_propV(
        db: &Database,
        category_id: &str,
        prop_id: &str,
        new_prop: &UpdatePropValueReq,
    ) -> Result<SinglePropValueRes> {
        let result = base_array::update_elem::<Self>(db, category_id, prop_id, new_prop).await?;
        Ok(SinglePropValueRes {
            status: "success",
            data: PropValueData { propV: result },
        })
    }

    pub async fn remove_propV(db: &Database, category_id: &str, prop_id: &str) -> Result<()> {
        base_array::remove_elem::<Self>(db, category_id, prop_id).await?;
        Ok(())
    }
}

fn validate_value(prop_type: &PropertyType, value: &PropValueType) -> Result<PropValueType> {
    match prop_type {
        PropertyType::MultiSelect | PropertyType::SingleSelect => {
            if let PropValueType::Multiple(_) = value {
                Ok(value.clone())
            } else {
                Err(TypedError(
                    "MultiSelect or SingleSelect types must have Multiple(Vec<String>) value"
                        .to_string(),
                ))
            }
        }
        _ => {
            if let PropValueType::Single(_) = value {
                Ok(value.clone())
            } else {
                Err(TypedError(
                    "Non-MultiSelect or SingleSelect types must have Single(String) value"
                        .to_string(),
                ))
            }
        }
    }
}
