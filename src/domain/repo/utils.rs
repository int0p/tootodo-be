use crate::domain::error::{Error::*, Result};
use crate::infra::db::error::Error as DBError;

use mongodb::options::ReturnDocument;
use mongodb::{
    bson::{oid::ObjectId, Document},
    options::FindOneAndUpdateOptions,
    Collection,
};
use serde::de::DeserializeOwned;

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

pub async fn update_mdoc_by_id<T>(
    coll: &Collection<T>,
    oid: &ObjectId,
    array_filters: Option<Document>,
    update_doc: Document,
    filter: Document,
) -> Result<T>
where
    T: DeserializeOwned + Unpin + Send + Sync,
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

    let doc = match coll.find_one_and_update(filter, update_doc, options).await {
        Ok(Some(doc)) => doc,
        Ok(None) => return Err(NotFoundError(oid.to_string())),
        Err(e) => return Err(DB(DBError::MongoQueryError(e))),
    };

    Ok(doc)
}
