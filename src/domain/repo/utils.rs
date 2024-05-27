use crate::domain::error::{Error::*, Result};
use crate::infra::db::error::Error as DBError;

use mongodb::bson::{self, Bson};
use mongodb::options::ReturnDocument;
use mongodb::{
    bson::{oid::ObjectId, Document},
    options::FindOneAndUpdateOptions,
    Collection,
};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub async fn find_doc_by_id(
    coll: &Collection<Document>,
    oid: &ObjectId,
    filter: Document,
) -> Result<Document> {
    let doc = match coll.find_one(filter, None).await {
        Ok(Some(doc)) => doc,
        Ok(None) => return Err(NotFoundError(oid.to_string())),
        Err(e) => return Err(DB(DBError::MongoQueryError(e))),
    };

    Ok(doc)
}

/// model에서 arr_name에 해당하는 배열을 추출하여 Vec<ElemModel>로 반환
pub async fn get_m_elems<Model, ElemModel>(arr_name: &str, model: &Model) -> Result<Vec<ElemModel>>
where
    Model: DeserializeOwned + Serialize + Unpin + Send + Sync,
    ElemModel: DeserializeOwned + Serialize + Unpin + Send + Sync,
{
    // 모델을 Document로 변환
    let serialized_data =
        bson::to_bson(model).map_err(|e| DB(DBError::MongoSerializeBsonError(e)))?;
    let doc = serialized_data.as_document().unwrap();

    // 배열 추출
    let array = match doc.get_array(arr_name) {
        Ok(array) => array,
        Err(e) => return Err(DB(DBError::MongoDataError(e))),
    };

    // doc to Vec<Model>
    let elems: Vec<ElemModel> = array
        .iter()
        .filter_map(|elem_bson| {
            if let Some(doc) = elem_bson.as_document() {
                bson::from_bson(Bson::Document(doc.clone())).ok()
            } else {
                None
            }
        })
        .collect();

    Ok(elems)
}

// model doc
pub async fn find_mdoc_by_id<Model>(
    coll: &Collection<Model>,
    oid: &ObjectId,
    filter: Document,
) -> Result<Model>
where
    Model: DeserializeOwned + Unpin + Send + Sync,
{
    let doc = match coll.find_one(filter, None).await {
        Ok(Some(doc)) => doc,
        Ok(None) => return Err(NotFoundError(oid.to_string())),
        Err(e) => return Err(DB(DBError::MongoQueryError(e))),
    };

    Ok(doc)
}

pub async fn update_mdoc_by_id<Model>(
    coll: &Collection<Model>,
    oid: &ObjectId,
    array_filters: Option<Document>,
    update_doc: Document,
    find_filter: Document,
) -> Result<Model>
where
    Model: DeserializeOwned + Unpin + Send + Sync,
{
    let options;

    if let Some(filters) = array_filters {
        options = FindOneAndUpdateOptions::builder()
            .array_filters(Some(vec![filters]))
            .return_document(ReturnDocument::After)
            .build();
    } else {
        options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();
    }

    let doc = match coll
        .find_one_and_update(find_filter, update_doc, options)
        .await
    {
        Ok(Some(doc)) => doc,
        Ok(None) => return Err(NotFoundError(oid.to_string())),
        Err(e) => return Err(DB(DBError::MongoQueryError(e))),
    };

    Ok(doc)
}
