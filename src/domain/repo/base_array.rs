use super::utils::update_doc_ret_doc;
use crate::domain::error::{Error::*, Result};
use crate::infra::db::error::Error as DBError;
use chrono::Utc;
use futures::TryStreamExt;
use mongodb::bson::Document;
use mongodb::bson::{self, doc, oid::ObjectId, Bson};
use mongodb::options::{AggregateOptions, IndexOptions};
use mongodb::{Database, IndexModel};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::str::FromStr;

pub trait MongoArrayRepo {
    type CollModel: DeserializeOwned + Serialize + Unpin + Send + Sync;
    type ElemModel: DeserializeOwned + Serialize + Unpin + Send + Sync;
    type UpdateElemReq: Serialize;
    type CreateElemReq: Serialize;
    type ElemRes: Serialize + DeserializeOwned + Unpin + Send + Sync;
    const COLL_NAME: &'static str;
    const ARR_NAME: &'static str;
    fn convert_doc_to_response(doc: &Self::ElemModel) -> Result<Self::ElemRes>;
    fn create_doc(body: &Self::CreateElemReq) -> Result<mongodb::bson::Document> {
        let ser_data = bson::to_bson(body).map_err(|e| DB(DBError::MongoSerializeBsonError(e)))?;
        let doc = ser_data.as_document().unwrap();

        let mut doc_with_date = doc! {
            "id": ObjectId::new(),
            "createdAt": Utc::now(),
        };

        doc_with_date.extend(doc.clone());
        Ok(doc_with_date)
    }
}

// S: Service
pub async fn fetch_elems<S>(
    db: &Database,
    src_id: &str,
    limit: Option<i64>,
    page: Option<i64>,
) -> Result<Vec<S::ElemRes>>
where
    S: MongoArrayRepo,
    S::CollModel: DeserializeOwned + Serialize + Unpin + Send + Sync,
{
    let doc_coll = db.collection::<Document>(S::COLL_NAME);
    let oid = ObjectId::from_str(src_id).map_err(DBError::MongoGetOidError)?;

    // 집계 파이프라인 설정
    let pipeline = match (limit, page) {
        (Some(limit), Some(page)) => {
            let skip = (page - 1) * limit;
            vec![
                doc! {
                    "$match": { "_id": oid }
                },
                doc! {
                    "$project": {
                        S::ARR_NAME: {
                            "$slice": [format!("${}", S::ARR_NAME), skip, limit]
                        }
                    }
                },
            ]
        }
        _ => {
            vec![doc! {
                "$match": { "_id": oid }
            }]
        }
    };

    // 집계 쿼리 실행
    let aggregate_options = AggregateOptions::builder().build();
    let mut cursor = doc_coll
        .aggregate(pipeline, aggregate_options)
        .await
        .map_err(DBError::MongoError)?;

    let mut elems: Vec<S::ElemRes> = Vec::new();

    while let Some(doc) = cursor.try_next().await.map_err(DBError::MongoError)? {
        let array = match doc.get_array(S::ARR_NAME) {
            Ok(array) => array,
            Err(e) => return Err(DB(DBError::MongoDataError(e))),
        };
        // 배열 내 원소들의 타입을 Response로 변환후, elems에 추가
        while let Some(elem_bson) = array.iter().next() {
            if let Ok(doc) = elem_bson
                .as_document()
                .ok_or(DBError::MongoDeserializeBsonError)
            {
                let elem: S::ElemModel = bson::from_bson(Bson::Document(doc.clone()))
                    .map_err(DBError::MongoDeserializeBsonError)?;
                elems.push(S::convert_doc_to_response(&elem)?);
            };
        }
    }

    Ok(elems)
}

pub async fn get_elem<S>(db: &Database, src_id: &str, elem_id: &str) -> Result<S::ElemRes>
where
    S: MongoArrayRepo,
    S::CollModel: DeserializeOwned + Serialize + Unpin + Send + Sync,
{
    let doc_coll = db.collection::<Document>(S::COLL_NAME);

    let oid = ObjectId::from_str(src_id).map_err(DBError::MongoGetOidError)?;
    let elem_oid = ObjectId::from_str(elem_id).map_err(DBError::MongoGetOidError)?;

    let doc = match doc_coll.find_one(doc! { "_id": oid }, None).await {
        Ok(Some(doc)) => doc,
        Ok(None) => return Err(NotFoundError(oid.to_string())),
        Err(e) => return Err(DB(DBError::MongoQueryError(e))),
    };

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
                    return S::convert_doc_to_response(&elem);
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
    new_elem: &S::CreateElemReq,
    index: Option<&str>,
) -> Result<S::ElemRes>
where
    S: MongoArrayRepo,
    S::CollModel: DeserializeOwned + Serialize + Unpin + Send + Sync,
{
    let coll = db.collection::<S::CollModel>(S::COLL_NAME);

    // 인덱스 생성
    if let Some(index_field) = index {
        let index_model = IndexModel::builder()
            .keys(doc! { index_field: 1 })
            .options(IndexOptions::builder().unique(false).build())
            .build();
        coll.create_index(index_model, None)
            .await
            .map_err(DBError::MongoError)?;
    }

    let oid = ObjectId::from_str(src_id).map_err(DBError::MongoGetOidError)?;

    // 배열의 맨 앞에 element 추가. -> 최신순
    let new_elem_doc = S::create_doc(new_elem)?;
    let update_doc = doc! {
        "$push": { S::ARR_NAME: {"$each":new_elem_doc,"$position":0 }},
        "$set": { "updatedAt": Bson::DateTime(Utc::now().into()) }
    };

    match update_doc_ret_doc(&coll, &oid, None, update_doc, doc! { "_id": oid }).await {
        Ok(updated_doc) => {
            let array = match updated_doc.get_array(S::ARR_NAME) {
                Ok(array) => array,
                Err(e) => return Err(DB(DBError::MongoDataError(e))),
            };

            // 배열의 첫번째 원소 리턴
            match array.first() {
                Some(first_elem) => {
                    if let Some(doc) = first_elem.as_document() {
                        let elem: S::ElemModel = bson::from_bson(Bson::Document(doc.clone()))
                            .map_err(DBError::MongoDeserializeBsonError)?;
                        S::convert_doc_to_response(&elem)
                    } else {
                        Err(NotFoundError(first_elem.to_string()))
                    }
                }
                None => Err(NotFoundError(format!(
                    "Can not found first element of array:{}",
                    S::ARR_NAME
                ))),
            }
        }
        Err(e) => Err(e),
    }
}

pub async fn remove_elem<S>(db: &Database, src_id: &str, elem_id: &str) -> Result<()>
where
    S: MongoArrayRepo,
    S::CollModel: DeserializeOwned + Serialize + Unpin + Send + Sync,
{
    let coll = db.collection::<S::CollModel>(S::COLL_NAME);

    let oid = ObjectId::from_str(src_id).map_err(DBError::MongoGetOidError)?;
    let elem_oid = ObjectId::from_str(elem_id).map_err(DBError::MongoGetOidError)?;

    let update_doc = doc! {
        "$pull": { S::ARR_NAME: doc! { "_id": elem_oid } },
        "$set": { "updatedAt": Bson::DateTime(Utc::now().into()) }
    };

    match update_doc_ret_doc(&coll, &oid, None, update_doc, doc! { "_id": oid }).await {
        Ok(updated_doc) => {
            let array = updated_doc
                .get_array(S::ARR_NAME)
                .map_err(DBError::MongoDataError)?;

            // 배열 내에 삭제한 값이 없는지 확인
            for elem_bson in array {
                if elem_bson
                    .as_document()
                    .and_then(|doc| doc.get_object_id("_id").ok())
                    .filter(|id| id == &elem_oid)
                    .is_some()
                {
                    return Err(NotRemovedError(elem_oid.to_string()));
                }
            }

            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub async fn update_elem<S>(
    db: &Database,
    src_id: &str,
    elem_id: &str,
    update_elem: &S::UpdateElemReq,
) -> Result<S::ElemRes>
where
    S: MongoArrayRepo,
    S::CollModel: DeserializeOwned + Serialize + Unpin + Send + Sync,
    S::UpdateElemReq: Serialize,
{
    let coll = db.collection::<S::CollModel>(S::COLL_NAME);

    let oid = ObjectId::from_str(src_id).map_err(DBError::MongoGetOidError)?;
    let elem_oid = ObjectId::from_str(elem_id).map_err(DBError::MongoGetOidError)?;

    let mut update_doc = doc! {
        "updatedAt": Bson::DateTime(Utc::now().into())
    };

    let update_elem_bson =
        bson::to_bson(update_elem).map_err(DBError::MongoSerializeBsonError)?;

    if let Bson::Document(update_elem_doc) = update_elem_bson {
        for (key, value) in update_elem_doc {
            update_doc.insert(format!("{}.$[elem].{}", S::ARR_NAME, key), value);
        }
    }

    let array_filters = doc! { "elem._id": elem_oid };

    match update_doc_ret_doc(
        &coll,
        &oid,
        Some(array_filters),
        update_doc,
        doc! { "_id": oid },
    )
    .await
    {
        Ok(updated_doc) => {
            let array = updated_doc
                .get_array(S::ARR_NAME)
                .map_err(DBError::MongoDataError)?;

            // 배열 내에서 업데이트한 원소 찾아 리턴.
            for elem_bson in array {
                if elem_bson
                    .as_document()
                    .and_then(|doc| doc.get_object_id("_id").ok())
                    .filter(|id| id == &elem_oid)
                    .is_some()
                {
                    return S::convert_doc_to_response(
                        &bson::from_bson(elem_bson.clone())
                            .map_err(DBError::MongoDeserializeBsonError)?,
                    );
                }
            }
            Err(NotFoundError(elem_oid.to_string()))
        }
        Err(e) => Err(e),
    }
}
