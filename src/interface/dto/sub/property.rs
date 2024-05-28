pub mod req {
    use crate::domain::types::PropertyType;
    use serde::{Deserialize, Serialize};

    // property
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreatePropertyReq {
        pub name: String,
        pub prop_type: PropertyType,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdatePropertyReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prop_type: Option<PropertyType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub options: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FilterPropertyReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prop_type: Option<PropertyType>,
    }
}
pub mod res {
    use crate::domain::{sub::property::PropertyModel, types::PropertyType};
    use serde::{Deserialize, Serialize};

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct PropertyRes {
        pub id: String,
        pub name: String,
        pub prop_type: PropertyType,
        pub options: Option<Vec<String>>,
    }

    impl PropertyRes {
        pub fn from_model(prop: &PropertyModel) -> Self {
            PropertyRes {
                id: prop.id.to_hex(),
                name: prop.name.clone(),
                prop_type: prop.prop_type.to_owned(),
                options: prop.options.clone(),
            }
        }
    }

    #[derive(Serialize, Debug)]
    pub struct PropertyData {
        pub prop: PropertyRes,
    }

    #[derive(Serialize, Debug)]
    pub struct SinglePropertyRes {
        pub status: &'static str,
        pub data: PropertyData,
    }

    #[derive(Serialize, Debug)]
    pub struct PropertyListRes {
        pub status: &'static str,
        pub results: usize,
        pub props: Vec<PropertyRes>,
    }
}
