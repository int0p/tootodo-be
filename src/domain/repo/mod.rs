use serde::{de::DeserializeOwned, Serialize};
pub mod base;
pub mod base_array;
pub mod utils;
//pub mod base_postgre;

pub trait ElemInfo {
    const ARR_NAME: &'static str;
    type UpdateReq: Serialize;
    type CreateReq: Serialize;
    type Res: DeserializeOwned + Serialize + Unpin + Send + Sync;

    fn convert_to_res(doc: &Self) -> Self::Res;
}

pub trait CollInfo {
    const COLL_NAME: &'static str;
    const ARR_NAME: &'static str;
}
