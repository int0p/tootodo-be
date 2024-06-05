use crate::domain::error::{Error::*, Result};
use crate::infra::db::error::Error as DBError;

use mongodb::bson::{self};
use mongodb::options::ReturnDocument;
use mongodb::{
    bson::{oid::ObjectId, Document},
    options::FindOneAndUpdateOptions,
    Collection,
};
use serde::de::DeserializeOwned;
use serde::Serialize;

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

pub async fn update_doc_ret_doc<Model>(
    coll: &Collection<Model>,
    oid: &ObjectId,
    array_filters: Option<Document>,
    update_doc: Document,
    find_filter: Document,
) -> Result<Document>
where
    Model: DeserializeOwned + Serialize + Unpin + Send + Sync,
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

    match coll
        .find_one_and_update(find_filter, update_doc, options)
        .await
    {
        Ok(Some(doc)) => {
            let serialized_data =
                bson::to_bson(&doc).map_err(|e| DB(DBError::MongoSerializeBsonError(e)))?;
            return Ok(serialized_data.as_document().unwrap().clone());
        }
        Ok(None) => Err(NotFoundError(oid.to_string())),
        Err(e) => Err(DB(DBError::MongoQueryError(e))),
    }
}

pub async fn update_doc_ret_model<Model>(
    coll: &Collection<Model>,
    oid: &ObjectId,
    array_filters: Option<Document>,
    update_doc: Document,
    find_filter: Document,
) -> Result<Model>
where
    Model: DeserializeOwned + Serialize + Unpin + Send + Sync,
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

    match coll
        .find_one_and_update(find_filter, update_doc, options)
        .await
    {
        Ok(Some(doc)) => {
            Ok(doc)
        }
        Ok(None) => Err(NotFoundError(oid.to_string())),
        Err(e) => Err(DB(DBError::MongoQueryError(e))),
    }
}
