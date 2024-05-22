use chrono::Utc;
use futures::StreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::bson::{Bson, Document};
use mongodb::options::{FindOneAndUpdateOptions, FindOptions, IndexOptions, ReturnDocument};
use mongodb::{bson, Database, IndexModel};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::convert::TryFrom;
use std::str::FromStr;
use uuid::Uuid;

use super::error::{Error::*, Result};
use crate::db::error::Error as DBError;

pub trait MongoBMC {
    type Model;
    type ModelResponse;
    const COLL_NAME: &'static str;
    const DOC_COLL_NAME: &'static str;
    fn convert_doc_to_response(doc: &Self::Model) -> Result<Self::ModelResponse>;
    fn create_doc<Schema: Serialize>(user: &Uuid, body: &Schema) -> Result<Document>;
}

pub async fn fetch<MC>(
    db: &Database,
    limit: i64,
    page: i64,
    user: &Uuid,
) -> Result<Vec<MC::ModelResponse>>
where
    MC: MongoBMC,
    MC::Model: DeserializeOwned + Unpin + Send + Sync,
{
    let coll = db.collection::<MC::Model>(MC::COLL_NAME);
    // let doc_coll = db.collection::<Document>(MC::DOC_COLL_NAME);

    let find_options = FindOptions::builder()
        .limit(limit)
        .skip(u64::try_from((page - 1) * limit).unwrap())
        .build();

    let mut cursor = coll
        .find(doc! {"user": user}, find_options)
        .await
        .map_err(|e| DBError::MongoQueryError(e))?;

    let mut json_result: Vec<MC::ModelResponse> = Vec::new();
    while let Some(doc) = cursor.next().await {
        json_result.push(MC::convert_doc_to_response(&doc.unwrap())?);
    }

    Ok(json_result)
}

pub async fn create<MC, Schema>(
    db: &Database,
    body: &Schema,
    user: &Uuid,
) -> Result<MC::ModelResponse>
where
    MC: MongoBMC,
    MC::Model: DeserializeOwned + Unpin + Send + Sync,
    Schema: Serialize,
{
    let coll = db.collection::<MC::Model>(MC::COLL_NAME);
    let doc_coll = db.collection::<Document>(MC::DOC_COLL_NAME);

    let document = MC::create_doc::<Schema>(user, &body)?;

    // let options = IndexOptions::builder().unique(true).build();
    let index = IndexModel::builder()
        .keys(doc! {"user": 1})
        // .options(options)
        .build();

    match coll.create_index(index, None).await {
        Ok(_) => {}
        Err(e) => return Err(DB(DBError::MongoQueryError(e))),
    };

    // 생성된 문서를 db에 추가.
    let insert_result = match doc_coll.insert_one(&document, None).await {
        Ok(result) => result,
        Err(e) => {
            if e.to_string()
                .contains("E11000 duplicate key error collection")
            {
                return Err(MongoDuplicateError(e));
            }
            return Err(DB(DBError::MongoQueryError(e)));
        }
    };

    // 삽입된 문서의 id추출
    let new_id = insert_result
        .inserted_id
        .as_object_id()
        .expect("issue with new _id");

    // 문서 삽입이 잘 되었는지 확인 및 반환.
    let doc = match coll.find_one(doc! {"_id": new_id}, None).await {
        Ok(Some(doc)) => doc,
        Ok(None) => return Err(NotFoundError(new_id.to_string())),
        Err(e) => return Err(DB(DBError::MongoQueryError(e))),
    };

    Ok(MC::convert_doc_to_response(&doc)?)
}

pub async fn get<MC>(db: &Database, id: &str, user: &Uuid) -> Result<MC::ModelResponse>
where
    MC: MongoBMC,
    MC::Model: DeserializeOwned + Unpin + Send + Sync,
{
    let coll = db.collection::<MC::Model>(MC::COLL_NAME);

    // model의 id를 ObjectId로 변환
    let oid = ObjectId::from_str(id).map_err(|e| DBError::MongoGetOidError(e))?;

    // id를 이용해 문서를 찾음.
    let doc = match coll.find_one(doc! {"_id": oid, "user":user}, None).await {
        Ok(Some(doc)) => doc,
        Ok(None) => return Err(NotFoundError(oid.to_string())),
        Err(e) => return Err(DB(DBError::MongoQueryError(e))),
    };

    Ok(MC::convert_doc_to_response(&doc)?)
}

pub async fn update<MC, Schema>(
    db: &Database,
    id: &str,
    body: &Schema,
    user: &Uuid,
) -> Result<MC::ModelResponse>
where
    MC: MongoBMC,
    MC::Model: DeserializeOwned + Unpin + Send + Sync,
    Schema: Serialize,
{
    let coll = db.collection::<MC::Model>(MC::COLL_NAME);

    let oid = ObjectId::from_str(id).map_err(|e| DBError::MongoGetOidError(e))?;

    let mut update_doc =
        bson::to_document(body).map_err(|e| DBError::MongoSerializeBsonError(e))?;
    update_doc.insert("updatedAt", Bson::DateTime(Utc::now().into()));

    let update = doc! {
        "$set": update_doc,
    };

    // option: 문서가 업데이트된 후의 상태를 반환
    let options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();

    let doc = match coll
        .find_one_and_update(doc! {"_id": oid,"user":user}, update, options)
        .await
    {
        Ok(Some(doc)) => doc,
        Ok(None) => return Err(NotFoundError(oid.to_string())),
        Err(e) => return Err(DB(DBError::MongoQueryError(e))),
    };

    Ok(MC::convert_doc_to_response(&doc)?)
}

pub async fn delete<MC: MongoBMC>(db: &Database, id: &str) -> Result<()> {
    let doc_coll = db.collection::<Document>(MC::DOC_COLL_NAME);

    let oid = ObjectId::from_str(id).map_err(|e| DBError::MongoGetOidError(e))?;
    let filter = doc! {"_id": oid };

    let result = doc_coll
        .delete_one(filter, None)
        .await
        .map_err(|e| DBError::MongoQueryError(e))?;

    match result.deleted_count {
        0 => Err(NotFoundError(id.to_string())),
        _ => Ok(()),
    }
}
