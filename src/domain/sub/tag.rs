use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};


#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TagModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub groups: Vec<ObjectId>,
}

// TODO: tagGroup에 tag추가 로직
