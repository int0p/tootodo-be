use super::utils::{find_doc_by_id, find_mdoc_by_id, update_mdoc_by_id};
use crate::domain::error::{Error::*, Result};
use crate::infra::db::error::Error as DBError;
use chrono::Utc;
use mongodb::bson::Document;
use mongodb::bson::{self, doc, oid::ObjectId, Bson};
use mongodb::Database;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::str::FromStr;
pub trait MongoArrayRepo {
    type CollModel: DeserializeOwned + Unpin + Send + Sync;
    type ElemModel: DeserializeOwned + Serialize + Unpin + Send + Sync;
    type UpdateElemReq: Serialize;
    const COLL_NAME: &'static str;
    const ARR_NAME: &'static str;
}

// S: Service
pub async fn get_elem<S>(db: &Database, src_id: &str, elem_id: &str) -> Result<S::ElemModel>
where
    S: MongoArrayRepo,
    S::CollModel: DeserializeOwned + Unpin + Send + Sync,
{
    let doc_coll = db.collection::<Document>(S::COLL_NAME);

    let oid = ObjectId::from_str(src_id).map_err(DBError::MongoGetOidError)?;
    let elem_oid = ObjectId::from_str(elem_id).map_err(DBError::MongoGetOidError)?;

    // Find array
    let doc = find_doc_by_id(&doc_coll, &oid, doc! { "_id": oid }).await?;
    let array = doc
        .get_array(S::ARR_NAME)
        .map_err(DBError::MongoDataError)?;

    // Find the specific element by ID within the array
    for elem_bson in array {
        match elem_bson.as_document() {
            Some(doc) => match doc.get_object_id("_id") {
                Ok(id) if id == elem_oid => {
                    let elem: S::ElemModel = bson::from_bson(Bson::Document(doc.clone()))
                        .map_err(DBError::MongoDeserializeBsonError)?;
                    return Ok(elem);
                }
                _ => continue,
            },
            None => continue,
        }
    }

    Err(NotFoundError(elem_oid.to_string()))
}

pub async fn add_elem<S>(
    db: &Database,
    src_id: &str,
    new_elem: &S::ElemModel,
) -> Result<S::CollModel>
where
    S: MongoArrayRepo,
    S::CollModel: DeserializeOwned + Unpin + Send + Sync,
{
    let coll = db.collection::<S::CollModel>(S::COLL_NAME);

    let oid = ObjectId::from_str(src_id).map_err(DBError::MongoGetOidError)?;

    let update_doc = doc! {
        "$push": { S::ARR_NAME: bson::to_bson(new_elem).map_err(DBError::MongoSerializeBsonError)? },
        "$set": { "updatedAt": Bson::DateTime(Utc::now().into()) }
    };

    update_mdoc_by_id(&coll, &oid, None, update_doc, doc! { "_id": oid }).await
}

pub async fn fetch_elems<S>(db: &Database, src_id: &str) -> Result<S::CollModel>
where
    S: MongoArrayRepo,
    S::CollModel: DeserializeOwned + Unpin + Send + Sync,
{
    let coll = db.collection::<S::CollModel>(S::COLL_NAME);

    let oid = ObjectId::from_str(src_id).map_err(DBError::MongoGetOidError)?;

    find_mdoc_by_id(&coll, &oid, doc! { "_id": oid }).await
}

pub async fn remove_elem<S>(db: &Database, src_id: &str, elem_id: &str) -> Result<S::CollModel>
where
    S: MongoArrayRepo,
    S::CollModel: DeserializeOwned + Unpin + Send + Sync,
{
    let coll = db.collection::<S::CollModel>(S::COLL_NAME);

    let oid = ObjectId::from_str(src_id).map_err(DBError::MongoGetOidError)?;

    let update_doc = doc! {
        "$pull": { S::ARR_NAME: doc! { "_id": ObjectId::from_str(elem_id).map_err(DBError::MongoGetOidError)? } },
        "$set": { "updatedAt": Bson::DateTime(Utc::now().into()) }
    };

    update_mdoc_by_id(&coll, &oid, None, update_doc, doc! { "_id": oid }).await
}

pub async fn update_elem<S>(
    db: &Database,
    src_id: &str,
    elem_id: &str,
    update_elem: &S::UpdateElemReq,
) -> Result<S::CollModel>
where
    S: MongoArrayRepo,
    S::CollModel: DeserializeOwned + Unpin + Send + Sync,
{
    let coll = db.collection::<S::CollModel>(S::COLL_NAME);

    let oid = ObjectId::from_str(src_id).map_err(DBError::MongoGetOidError)?;

    let mut update_doc = doc! {
        "updatedAt": Bson::DateTime(Utc::now().into())
    };

    // `update_elem`을 BSON Document로 변환
    let update_elem_bson = bson::to_bson(update_elem).map_err(DBError::MongoSerializeBsonError)?;

    if let Bson::Document(update_elem_doc) = update_elem_bson {
        for (key, value) in update_elem_doc {
            update_doc.insert(format!("{}.$[elem].{}", S::ARR_NAME, key), value);
        }
    }

    // 배열 필터 설정
    let array_filters =
        doc! { "elem._id": ObjectId::from_str(elem_id).map_err(DBError::MongoGetOidError)? };

    // MongoDB 업데이트 실행
    update_mdoc_by_id(
        &coll,
        &oid,
        Some(array_filters),
        update_doc,
        doc! { "_id": oid },
    )
    .await
}