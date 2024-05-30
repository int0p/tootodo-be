pub mod req {
    use crate::infra::types::{PropValueType, PropertyType};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreatePropValueReq {
        pub prop_id: String,
        pub prop_name: String,
        pub value: PropValueType,
        pub prop_type: PropertyType,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdatePropValueReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub values: Option<Vec<PropValueType>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prop_type: Option<PropertyType>,
    }
}
pub mod res {
    use crate::domain::sub::task_propV::PropValueModel;
    use crate::infra::types::{PropValueType, PropertyType};

    use serde::{Deserialize, Serialize};

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct PropValueRes {
        pub id: String,
        pub prop_name: String,
        pub value: Option<PropValueType>,
        pub prop_type: PropertyType,
    }

    impl PropValueRes {
        pub fn from_model(propV: &PropValueModel) -> Self {
            Self {
                id: propV.prop_id.to_hex(),
                prop_name: propV.prop_name.clone(),
                value: propV.values.clone(),
                prop_type: propV.prop_type.to_owned(),
            }
        }
    }

    #[derive(Serialize, Debug)]
    pub struct PropValueData {
        pub propV: PropValueRes,
    }

    #[derive(Serialize, Debug)]
    pub struct SinglePropValueRes {
        pub status: &'static str,
        pub data: PropValueData,
    }

    #[derive(Serialize, Debug)]
    pub struct PropValueListRes {
        pub status: &'static str,
        pub results: usize,
        pub propVs: Vec<PropValueRes>,
    }
}
