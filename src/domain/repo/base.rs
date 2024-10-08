use chrono::Utc;
use futures::StreamExt;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::bson::{Bson, Document};
use mongodb::options::{FindOptions, IndexOptions};
use mongodb::{bson, Database, IndexModel};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::str::FromStr;
use uuid::Uuid;

use crate::domain::error::{Error::*, Result};
use crate::infra::db::error::Error as DBError;
use crate::infra::types::QueryFilterOptions;

use super::utils::{find_mdoc_by_id, update_doc_ret_model};

pub trait MongoRepo {
    type Model: Debug;
    type ModelResponse;
    const COLL_NAME: &'static str;
    fn convert_doc_to_response(doc: &Self::Model) -> Self::ModelResponse;
    fn create_doc<Schema: Serialize>(user: &Uuid, body: &Schema) -> Result<Document>;
}

// S: Service
pub async fn fetch<S>(
    db: &Database,
    filter_opts: QueryFilterOptions,
    user: &Uuid,
) -> Result<Vec<S::ModelResponse>>
where
    S: MongoRepo,
    S::Model: DeserializeOwned + Serialize + Unpin + Send + Sync,
{
    let coll = db.collection::<S::Model>(S::COLL_NAME);

    let (find_filter, proj_opts, limit, page) = (
        filter_opts.find_filter.unwrap_or(doc! {"user": user}),
        filter_opts.proj_opts.unwrap_or_default(),
        filter_opts.limit,
        filter_opts.page,
    );

    let find_options = FindOptions::builder()
        .projection(proj_opts)
        .sort(doc!{"parent_id":1})
        .limit(limit)
        .skip(u64::try_from((page - 1) * limit).unwrap())
        .build();

    let mut cursor = coll
        .find(find_filter, find_options)
        .await
        .map_err(DBError::MongoQueryError)?;

    let mut json_result: Vec<S::ModelResponse> = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(doc) => {
                // tracing::info!("doc: {:?}", &doc);
                let res = S::convert_doc_to_response(&doc);
                json_result.push(res);
            }
            Err(e) => return Err(DB(DBError::MongoQueryError(e))),
        }
    }

    Ok(json_result)
}

pub async fn create<S, Schema>(
    db: &Database,
    body: &Schema,
    user: &Uuid,
    indexes: Option<Vec<&str>>,
) -> Result<S::ModelResponse>
where
    S: MongoRepo,
    S::Model: DeserializeOwned + Serialize + Unpin + Send + Sync,
    Schema: Serialize,
{
    let coll = db.collection::<S::Model>(S::COLL_NAME);
    let doc_coll = db.collection::<Document>(S::COLL_NAME);

    let document = S::create_doc::<Schema>(user, body)?;

    if let Some(index_fields) = indexes {
        let mut index_doc = doc! {};
        for field in index_fields {
            index_doc.insert(field, 1);
        }

        index_doc.insert("user", 1);

        // TODO: unique index 설정(tag group, tag의 경우 동일 이름을 가진 tag group을 생성할 수 없음)
        let index = IndexModel::builder()
            .keys(index_doc)
            .options(IndexOptions::builder().unique(false).build())
            .build();

        coll.create_index(index, None)
            .await
            .map_err(DBError::MongoError)?;
    }

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
    let doc = find_mdoc_by_id(&coll, &new_id, doc! {"_id": new_id, "user":user}).await?;

    Ok(S::convert_doc_to_response(&doc))
}

pub async fn get<S>(db: &Database, id: &str, user: &Uuid) -> Result<S::ModelResponse>
where
    S: MongoRepo,
    S::Model: DeserializeOwned + Serialize + Unpin + Send + Sync,
{
    let coll = db.collection::<S::Model>(S::COLL_NAME);

    // model의 id를 ObjectId로 변환
    let oid = ObjectId::from_str(id).map_err(DBError::MongoGetOidError)?;

    // id를 이용해 문서를 찾음.
    let doc = find_mdoc_by_id(&coll, &oid, doc! {"_id": oid, "user":user}).await?;

    Ok(S::convert_doc_to_response(&doc))
}

pub async fn update<S, Schema>(
    db: &Database,
    id: &str,
    body: &Schema,
    user: &Uuid,
) -> Result<S::ModelResponse>
where
    S: MongoRepo,
    S::Model: DeserializeOwned + Serialize + Unpin + Send + Sync,
    Schema: Serialize,
{
    let coll = db.collection::<S::Model>(S::COLL_NAME);

    let oid = ObjectId::from_str(id).map_err(DBError::MongoGetOidError)?;

    let mut update_doc = bson::to_document(body).map_err(DBError::MongoSerializeBsonError)?;
    update_doc.insert("updatedAt", Bson::DateTime(Utc::now().into()));

    let doc = update_doc_ret_model(
        &coll,
        &oid,
        None,
        doc! {
            "$set": update_doc,
        },
        doc! {"_id": oid,"user":user},
    )
    .await?;

    Ok(S::convert_doc_to_response(&doc))
}

pub async fn update_unset_fields<S>(
    db: &Database,
    id: &str,
    unset_fields: &[&str],
    user: &Uuid,
) -> Result<S::ModelResponse>
where
    S: MongoRepo,
    S::Model: DeserializeOwned + Serialize + Unpin + Send + Sync,
{
    let coll = db.collection::<S::Model>(S::COLL_NAME);

    let oid = ObjectId::from_str(id).map_err(DBError::MongoGetOidError)?;

    // $unset 연산자를 위한 Document 생성
    let mut unset_doc = Document::new();
    for field in unset_fields {
        unset_doc.insert(*field, Bson::Int32(1)); // 값은 1로 설정
    }

    // 업데이트 연산자 생성 ($unset 및 $set)
    let mut update_operator = Document::new();
    update_operator.insert("$unset", unset_doc);
    update_operator.insert("$set", doc! {
        "updatedAt": Bson::DateTime(Utc::now().into()),
    });

    // 업데이트 실행
    let doc = update_doc_ret_model(
        &coll,
        &oid,
        None,
        update_operator,
        doc! {"_id": oid, "user": user},
    )
    .await?;

    Ok(S::convert_doc_to_response(&doc))
}


pub async fn delete<S: MongoRepo>(db: &Database, id: &str) -> Result<()> {
    let doc_coll = db.collection::<Document>(S::COLL_NAME);

    let oid = ObjectId::from_str(id).map_err(DBError::MongoGetOidError)?;
    let filter = doc! {"_id": oid };

    let result = doc_coll
        .delete_one(filter, None)
        .await
        .map_err(DBError::MongoQueryError)?;

    match result.deleted_count {
        0 => Err(NotFoundError(id.to_string())),
        _ => Ok(()),
    }
}
