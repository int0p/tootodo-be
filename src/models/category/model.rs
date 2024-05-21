use chrono::{DateTime, Utc};
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryModel{
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub user: Uuid,
    pub name:String,
    pub color:String,
    pub status: StatusType,
    pub properties: Vec<PropertyModel>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PropertyModel{
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name:String,
    pub prop_type:PropertyType,
    pub options:Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum StatusType {    
    InProgress,
    Archived,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PropertyType{
    MultiSelect,
    SingleSelect,    
    
    Text,
    Number,
    DateTime,
    File,
    Image,
    Link,
    Email,
    Phone,
    Location,    
}

impl PropertyModel {
    pub fn new(id: ObjectId, name: String, prop_type: PropertyType, options: Option<Vec<String>>) -> Self {
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

    pub fn update(&mut self, name: String, prop_type: PropertyType, options: Option<Vec<String>>) {
        self.name = name;
        self.prop_type = prop_type; //TODO select -> text, others -> select or text
        self.options = match prop_type {
            PropertyType::MultiSelect | PropertyType::SingleSelect => options,
            _ => None,
        };
    }
}
