use crate::db::error::Error as DBError;
use crate::models::error::{Error::*, Result};
use chrono::Utc;
use mongodb::bson::Document;
use mongodb::bson::{self, doc, oid::ObjectId, Bson};
use mongodb::{options::FindOneAndUpdateOptions, options::ReturnDocument, Database};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::str::FromStr;

pub trait MongoArrayBMC {
    type CollModel: DeserializeOwned + Unpin + Send + Sync;
    type ElemModel: DeserializeOwned + Serialize + Unpin + Send + Sync;
    type UpdateElemSchema: Serialize;
    const COLL_NAME: &'static str;
    const ARR_NAME: &'static str;
}

pub async fn get_elem<MC>(db: &Database, src_id: &str, elem_id: &str) -> Result<MC::ElemModel>
where
    MC: MongoArrayBMC,
    MC::CollModel: DeserializeOwned + Unpin + Send + Sync,
{
    let doc_coll = db.collection::<Document>(MC::COLL_NAME);

    let oid = ObjectId::from_str(src_id).map_err(|e| DBError::MongoGetOidError(e))?;
    let elem_oid = ObjectId::from_str(elem_id).map_err(|e| DBError::MongoGetOidError(e))?;

    // Find array
    let doc = match doc_coll.find_one(doc! { "_id": oid }, None).await {
        Ok(Some(doc)) => doc,
        Ok(None) => return Err(NotFoundError(oid.to_string())),
        Err(e) => return Err(DB(DBError::MongoQueryError(e))),
    };

    let array = doc
        .get_array(MC::ARR_NAME)
        .map_err(|e| DBError::MongoDataError(e))?;

    // Find the specific element by ID within the array
    for elem_bson in array {
        match elem_bson.as_document() {
            Some(doc) => match doc.get_object_id("_id") {
                Ok(id) if id == elem_oid => {
                    let elem: MC::ElemModel = bson::from_bson(Bson::Document(doc.clone()))
                        .map_err(|e| DBError::MongoDeserializeBsonError(e))?;
                    return Ok(elem);
                }
                _ => continue,
            },
            None => continue,
        }
    }

    Err(NotFoundError(elem_oid.to_string()))
}
pub async fn add_elem<MC>(
    db: &Database,
    src_id: &str,
    new_elem: &MC::ElemModel,
) -> Result<MC::CollModel>
where
    MC: MongoArrayBMC,
    MC::CollModel: DeserializeOwned + Unpin + Send + Sync,
{
    let coll = db.collection::<MC::CollModel>(MC::COLL_NAME);

    let oid = ObjectId::from_str(src_id).map_err(|e| DBError::MongoGetOidError(e))?;

    let update_doc = doc! {
        "$push": { MC::ARR_NAME: bson::to_bson(new_elem).map_err(|e| DBError::MongoSerializeBsonError(e))? },
        "$set": { "updatedAt": Bson::DateTime(Utc::now().into()) }
    };

    let options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();

    let doc = match coll
        .find_one_and_update(doc! {"_id": oid}, update_doc, options)
        .await
    {
        Ok(Some(doc)) => doc,
        Ok(None) => return Err(NotFoundError(oid.to_string())),
        Err(e) => return Err(DB(DBError::MongoQueryError(e))),
    };

    Ok(doc)
}

pub async fn fetch_elems<MC>(db: &Database, src_id: &str) -> Result<MC::CollModel>
where
    MC: MongoArrayBMC,
    MC::CollModel: DeserializeOwned + Unpin + Send + Sync,
{
    let coll = db.collection::<MC::CollModel>(MC::COLL_NAME);

    let oid = ObjectId::from_str(src_id).map_err(|e| DBError::MongoGetOidError(e))?;

    let doc = match coll.find_one(doc! {"_id": oid}, None).await {
        Ok(Some(doc)) => doc,
        Ok(None) => return Err(NotFoundError(oid.to_string())),
        Err(e) => return Err(DB(DBError::MongoQueryError(e))),
    };

    Ok(doc)
}

pub async fn remove_elem<MC>(db: &Database, src_id: &str, elem_id: &str) -> Result<MC::CollModel>
where
    MC: MongoArrayBMC,
    MC::CollModel: DeserializeOwned + Unpin + Send + Sync,
{
    let coll = db.collection::<MC::CollModel>(MC::COLL_NAME);

    let oid = ObjectId::from_str(src_id).map_err(|e| DBError::MongoGetOidError(e))?;

    let update_doc = doc! {
        "$pull": { MC::ARR_NAME: doc! { "_id": ObjectId::from_str(elem_id).map_err(|e| DBError::MongoGetOidError(e))? } },
        "$set": { "updatedAt": Bson::DateTime(Utc::now().into()) }
    };

    let options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();

    let doc = match coll
        .find_one_and_update(doc! {"_id": oid}, update_doc, options)
        .await
    {
        Ok(Some(doc)) => doc,
        Ok(None) => return Err(NotFoundError(oid.to_string())),
        Err(e) => return Err(DB(DBError::MongoQueryError(e))),
    };

    Ok(doc)
}

pub async fn update_elem<MC>(
    db: &Database,
    src_id: &str,
    elem_id: &str,
    update_elem: &MC::UpdateElemSchema,
) -> Result<MC::CollModel>
where
    MC: MongoArrayBMC,
    MC::CollModel: DeserializeOwned + Unpin + Send + Sync,
{
    let coll = db.collection::<MC::CollModel>(MC::COLL_NAME);

    let oid = ObjectId::from_str(src_id).map_err(|e| DBError::MongoGetOidError(e))?;

    let mut update_doc = doc! {
        "updatedAt": Bson::DateTime(Utc::now().into())
    };

    // `update_elem`을 BSON Document로 변환
    let update_elem_bson =
        bson::to_bson(update_elem).map_err(|e| DBError::MongoSerializeBsonError(e))?;

    if let Bson::Document(update_elem_doc) = update_elem_bson {
        for (key, value) in update_elem_doc {
            update_doc.insert(format!("{}.$[elem].{}", MC::ARR_NAME, key), value);
        }
    }

    // 배열 필터 설정
    let array_filters =
        doc! { "elem._id": ObjectId::from_str(elem_id).map_err(|e| DBError::MongoGetOidError(e))? };
    let options = FindOneAndUpdateOptions::builder()
        .array_filters(Some(vec![array_filters]))
        .return_document(ReturnDocument::After)
        .build();

    // MongoDB 업데이트 실행
    let doc = match coll
        .find_one_and_update(doc! {"_id": oid}, doc! { "$set": update_doc }, options)
        .await
    {
        Ok(Some(doc)) => doc,
        Ok(None) => return Err(NotFoundError(oid.to_string())),
        Err(e) => return Err(DB(DBError::MongoQueryError(e))),
    };

    Ok(doc)
}
