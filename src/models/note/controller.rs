
use super::{
    error::{Error::*,Result},
    model::NoteModel,
    response::{NoteData, NoteListResponse, NoteResponse, SingleNoteResponse},
    schema::{CreateNoteSchema, FilterOptions, UpdateNoteSchema},
};
use crate::db::error::Error as DBError;
use chrono::prelude::*;
use futures::StreamExt;
use mongodb::bson::Document;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::options::{FindOneAndUpdateOptions, FindOptions, IndexOptions, ReturnDocument};
use mongodb::{bson, Collection, IndexModel};
use std::convert::TryFrom;
use std::str::FromStr;
use uuid::Uuid;


#[derive(Clone, Debug)]
pub struct MemoBMC {
    pub collection: Collection<NoteModel>, //NoteModel에 정의된 필드에 대한 인덱스를 생성, 검색,
    pub doc_collection: Collection<Document>, //문서 삽입, 삭제
}

impl MemoBMC {
    //mongodb에서 note를 가져옴.
    pub async fn fetch_notes(
        &self,
        limit: i64,
        page: i64,
        user: &Uuid,
    ) -> Result<NoteListResponse> {
        let find_options = FindOptions::builder()
            .limit(limit)
            .skip(u64::try_from((page - 1) * limit).unwrap())
            .build();

        let mut cursor = self
            .collection
            .find(doc! {"user": user}, find_options)
            .await
            .map_err(|e| DB(DBError::MongoQueryError(e)))?;

        let mut json_result: Vec<NoteResponse> = Vec::new();
        while let Some(doc) = cursor.next().await {
            json_result.push(self.doc_to_note(&doc.unwrap())?);
        }

        Ok(NoteListResponse {
            status: "success",
            results: json_result.len(),
            notes: json_result,
        })
    }

    pub async fn create_note(
        &self,
        body: &CreateNoteSchema,
        user: &Uuid,
    ) -> Result<SingleNoteResponse> {
        let published = body.published.to_owned().unwrap_or(false);
        let category = body.category.to_owned().unwrap_or_default();

        let document = self.create_note_document(user, body, published, category)?;

        // title기반 index생성. 중복 title방지.
        // let options = IndexOptions::builder().unique(true).build();
        let index = IndexModel::builder()
            .keys(doc! {"user": 1})
            // .options(options)
            .build();

        match self.collection.create_index(index, None).await {
            Ok(_) => {}
            Err(e) => return Err(DB(DBError::MongoQueryError(e))),
        };

        // 생성된 문서를 db에 추가.
        let insert_result = match self.doc_collection.insert_one(&document, None).await {
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
        let note_doc = match self.collection.find_one(doc! {"_id": new_id}, None).await {
            Ok(Some(doc)) => doc,
            Ok(None) => return Err(NotFoundError(new_id.to_string())),
            Err(e) => return Err(DB(DBError::MongoQueryError(e))),
        };

        Ok(SingleNoteResponse {
            status: "success",
            data: NoteData {
                note: self.doc_to_note(&note_doc)?,
            },
        })
    }

    pub async fn get_note(&self, id: &str, user: &Uuid) -> Result<SingleNoteResponse> {
        // note의 id를 ObjectId로 변환
        let oid = ObjectId::from_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;

        // id를 이용해 문서를 찾음.
        let note_doc = self
            .collection
            .find_one(doc! {"_id":oid, "user":user}, None)
            .await
            .map_err(|e| DB(DBError::MongoQueryError(e)))?;

        match note_doc {
            Some(doc) => {
                let note = self.doc_to_note(&doc)?;
                Ok(SingleNoteResponse {
                    status: "success",
                    data: NoteData { note },
                })
            }
            None => Err(NotFoundError(id.to_string())),
        }
    }

    pub async fn edit_note(
        &self,
        id: &str,
        body: &UpdateNoteSchema,
        user: &Uuid,
    ) -> Result<SingleNoteResponse> {
        let oid = ObjectId::from_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;

        let update = doc! {
            "$set": bson::to_document(body).map_err( |e| DB(DBError::MongoSerializeBsonError(e)))?,
        };

        // option: 문서가 업데이트된 후의 상태를 반환
        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        if let Some(doc) = self
            .collection
            .find_one_and_update(doc! {"_id": oid,"user":user}, update, options)
            .await
            .map_err(|e| DB(DBError::MongoQueryError(e)))?
        {
            let note = self.doc_to_note(&doc)?;
            let note_response = SingleNoteResponse {
                status: "success",
                data: NoteData { note },
            };
            Ok(note_response)
        } else {
            Err(NotFoundError(id.to_string()))
        }
    }

    pub async fn delete_note(&self, id: &str) -> Result<()> {
        let oid = ObjectId::from_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let filter = doc! {"_id": oid };

        let result = self
            .doc_collection
            .delete_one(filter, None)
            .await
            .map_err(|e| DB(DBError::MongoQueryError(e)))?;

        dbg!(&result);

        match result.deleted_count {
            0 => Err(NotFoundError(id.to_string())),
            _ => Ok(()),
        }
    }

    fn doc_to_note(&self, note: &NoteModel) -> Result<NoteResponse> {
        let note_response = NoteResponse {
            user: note.user,
            id: note.id.to_hex(),
            title: note.title.to_owned(),
            content: note.content.to_owned(),
            category: note.category.to_owned().unwrap(),
            published: note.published.unwrap(),
            createdAt: note.createdAt,
            updatedAt: note.updatedAt,
        };

        Ok(note_response)
    }

    fn create_note_document(
        &self,
        user: &Uuid,
        body: &CreateNoteSchema,
        published: bool,
        category: String,
    ) -> Result<bson::Document> {
        let serialized_data =
            bson::to_bson(body).map_err(|e| DB(DBError::MongoSerializeBsonError(e)))?;
        let document = serialized_data.as_document().unwrap();

        let datetime = Utc::now();

        let mut doc_with_dates = doc! {
            "user": user,
            "createdAt": datetime,
            "updatedAt": datetime,
            "published": published,
            "category": category
        };
        doc_with_dates.extend(document.clone());

        Ok(doc_with_dates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::MongoDB;
    use dotenv::dotenv;
    use mongodb::options::UpdateOptions;

    async fn setup() -> MemoBMC {
        dotenv().ok();
        let mongodb = MongoDB::init_test().await.unwrap();
        
        // 시드 데이터 생성
        let user = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let seeds = vec![
            NoteModel {
                id:ObjectId::from_str("507f1f77bcf86cd799439011").unwrap(),
                user,
                title: "첫 번째 노트".to_string(),
                content: "첫 번째 노트의 내용입니다.".to_string(),
                category: Some("일반".to_string()),
                published: Some(true),
                createdAt: Utc::now(),
                updatedAt: Utc::now(),
            },
            NoteModel {
                id:ObjectId::from_str("507f191e810c19729de860ea").unwrap(),
                user,
                title: "두 번째 노트".to_string(),
                content: "두 번째 노트의 내용입니다.".to_string(),
                category: Some("중요".to_string()),
                published: Some(false),
                createdAt: Utc::now(),
                updatedAt: Utc::now(),
            },
            NoteModel {
                id:ObjectId::from_str("507f191e810c19729de860ec").unwrap(),
                user,
                title: "세 번째 노트".to_string(),
                content: "세 번째 노트의 내용입니다.".to_string(),
                category: Some("공부".to_string()),
                published: Some(true),
                createdAt: Utc::now(),
                updatedAt: Utc::now(),
            },
        ];

        // 시드 데이터를 MongoDB에 삽입
        for seed in seeds {
            let filter = doc! { "_id": seed.id };
            let update = doc! { "$setOnInsert": bson::to_bson(&seed).unwrap() };
            let options = UpdateOptions::builder()
                .upsert(true)
                .build();
        
            let result = mongodb
                .note
                .collection
                .update_one(filter, update, options)
                .await
                .expect("cannot insert seed data");
        
            if result.upserted_id.is_some() {
                println!("✅ 새로운 노트 시드 데이터가 추가되었습니다. ID: {}", seed.id);
            } else {
                println!("⚠️ 이미 존재하는 노트 시드 데이터입니다. ID: {}", seed.id);
            }
        }
        
        MemoBMC {
            collection: mongodb.note.collection.clone(),
            doc_collection: mongodb.note.doc_collection.clone(),
        }
    }

    #[tokio::test]
    async fn test_create_note() {
        let memo_bmc = setup().await;
        let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let body = CreateNoteSchema {
            user: user_id,
            title: "Test Note".to_string(),
            content: "This is a test note".to_string(),
            category: Some("test".to_string()),
            published: Some(true),
        };

        let result = memo_bmc.create_note(&body, &user_id).await;
        // dbg!(&result);
        assert!(result.is_ok());

        let note_response = result.unwrap();
        assert_eq!(note_response.status, "success");
        assert_eq!(note_response.data.note.title, body.title);
        assert_eq!(note_response.data.note.content, body.content);
        assert_eq!(note_response.data.note.category, body.category.unwrap());
        assert_eq!(note_response.data.note.published, body.published.unwrap());
    }

    #[tokio::test]
    async fn test_fetch_notes() {
        let memo_bmc = setup().await;
        let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let limit = 10;
        let page = 1;

        let result = memo_bmc.fetch_notes(limit, page, &user_id).await;
        // dbg!(&result);
        assert!(result.is_ok());

        let note_list_response = result.unwrap();
        assert_eq!(note_list_response.status, "success");
    }

    #[tokio::test]
    async fn test_get_note() {
        let memo_bmc = setup().await;
        let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let note_id = "507f1f77bcf86cd799439011";

        let result = memo_bmc.get_note(note_id, &user_id).await;
        // dbg!(&result);
        assert!(result.is_ok());

        let note_response = result.unwrap();
        assert_eq!(note_response.status, "success");
    }

    #[tokio::test]
    async fn test_edit_note() {
        let memo_bmc = setup().await;
        let user_id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let note_id = "507f1f77bcf86cd799439011";
        let body = UpdateNoteSchema {
            user: user_id,
            title: Some("Updated Test Note".to_string()),
            content: Some("This is an updated test note".to_string()),
            category: Some("updated_test".to_string()),
            published: Some(false),
        };

        let result = memo_bmc.edit_note(note_id, &body, &user_id).await;
        // dbg!(&result);
        assert!(result.is_ok());

        let note_response = result.unwrap();
        assert_eq!(note_response.status, "success");
        assert_eq!(note_response.data.note.title, body.title.unwrap());
        assert_eq!(note_response.data.note.content, body.content.unwrap());
        assert_eq!(note_response.data.note.category, body.category.unwrap());
        assert_eq!(note_response.data.note.published, body.published.unwrap());
    }

    #[tokio::test]
    async fn test_delete_note() {
        let memo_bmc = setup().await;
        let note_id = "507f191e810c19729de860ec";

        let result = memo_bmc.delete_note(note_id).await;
        // dbg!(&result);
        assert!(result.is_ok());
    }
}
