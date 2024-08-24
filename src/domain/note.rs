use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use std::str::FromStr;

use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    options::AggregateOptions,
    Database,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::sub::{
    chat::MsgModel,
    property::PropertyService,
    note_propV::{PropValueModel, PropValueService},
};

use crate::{
    domain::{
        error::{Error::*, Result},
        repo::base::{self, MongoRepo},
        sub::note_page::PageModel,
    },
    infra::{
        db::error::Error as DBError,
        types::{ChatType, PropertyType, QueryFilterOptions},
    },
    interface::dto::{
        sub::note_propV::{req::UpdatePropValueReq, res::PropValueListRes},
        note::{
            req::{CreateNoteReq, NoteFetchOptions, UpdateNoteReq},
            res::{SingleNoteRes, NoteData, NoteListRes, NoteRes},
        },
    },
};


#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoteModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(with = "bson::serde_helpers::uuid_1_as_binary")]
    pub user: Uuid,
    pub title: String,

    pub category_id: ObjectId,
    pub category_color: String,
    pub category_name: String,

    pub prop_values: Vec<PropValueModel>,

    pub pages: Vec<PageModel>,
    pub connected_task: Option<ObjectId>,
    pub parent_id: Option<ObjectId>,

    pub chat_type: ChatType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_msgs: Option<Vec<MsgModel>>,

    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub createdAt: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updatedAt: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct NoteService;

impl MongoRepo for NoteService {
    const COLL_NAME: &'static str = "notes";
    type Model = NoteModel;
    type ModelResponse = NoteRes;

    fn convert_doc_to_response(note: &NoteModel) -> Result<NoteRes> {
        Ok(NoteRes::from_model(note))
    }

    fn create_doc<CreateNoteReq: Serialize>(user: &Uuid, body: &CreateNoteReq) -> Result<Document> {
        let serialized_data =
            bson::to_bson(body).map_err(|e| DB(DBError::MongoSerializeBsonError(e)))?;
        let document = serialized_data.as_document().unwrap();

        let datetime = Utc::now();

        let mut doc_with_dates = doc! {
            "user": user,
            "complete":false,
            "chat_type": "Note",
            "createdAt": datetime,
            "updatedAt": datetime,
        };
        doc_with_dates.extend(document.clone());
        Ok(doc_with_dates)
    }
}

impl NoteService {
    pub async fn fetch_notes(
        db: &Database,
        limit: i64,
        page: i64,
        user: &Uuid,
    ) -> Result<NoteListRes> {
        let filter_opts = QueryFilterOptions {
            find_filter: Some(doc! {"user": user}),
            proj_opts: Some(NoteFetchOptions::build_projection()),
            limit,
            page,
        };
        let notes_result = base::fetch::<Self>(db, filter_opts, user)
            .await
            .expect("note 응답을 받아오지 못했습니다.");

        Ok(NoteListRes {
            status: "success",
            results: notes_result.len(),
            notes: notes_result,
        })
    }

    pub async fn fetch_notes_by_category(
        db: &Database,
        limit: i64,
        page: i64,
        category_id: &str,
        user: &Uuid,
    ) -> Result<NoteListRes> {
        let filter_opts = QueryFilterOptions {
            find_filter: Some(doc! {"user": user,"category_id":category_id}),
            proj_opts: Some(NoteFetchOptions::build_projection()),
            limit,
            page,
        };

        let notes_result = base::fetch::<Self>(db, filter_opts, user)
            .await
            .expect("note 응답을 받아오지 못했습니다.");

        Ok(NoteListRes {
            status: "success",
            results: notes_result.len(),
            notes: notes_result,
        })
    }

    pub async fn create_note(
        db: &Database,
        body: &mut CreateNoteReq,
        user: &Uuid,
    ) -> Result<SingleNoteRes> {
        if let Ok(prop_values) = Self::get_prop_values(db, &body.category_id).await {
            body.prop_values = Some(prop_values);
        }
        let note_result =
            base::create::<Self, CreateNoteReq>(db, body, user, Some(vec!["category_id"]))
                .await
                .expect("note 생성에 실패했습니다.");

        Ok(SingleNoteRes {
            status: "success",
            data: NoteData { note: note_result },
        })
    }

    pub async fn get_note(db: &Database, id: &str, user: &Uuid) -> Result<SingleNoteRes> {
        let note_result = base::get::<Self>(db, id, user)
            .await
            .expect("note를 가져오는데 실패했습니다.");

        Ok(SingleNoteRes {
            status: "success",
            data: NoteData { note: note_result },
        })
    }

    pub async fn update_note(
        db: &Database,
        id: &str,
        body: &UpdateNoteReq,
        user: &Uuid,
    ) -> Result<SingleNoteRes> {
        let note_result = base::update::<Self, UpdateNoteReq>(db, id, body, user)
            .await
            .expect("note 업데이트에 실패했습니다.");

        Ok(SingleNoteRes {
            status: "success",
            data: NoteData { note: note_result },
        })
    }

    pub async fn delete_note(db: &Database, id: &str) -> Result<()> {
        base::delete::<Self>(db, id).await
    }

    // category 갱신에 따른 note 갱신
    pub async fn update_notes_for_category_change(
        db: &Database,
        category_id: &str,
        new_category_name: &str,
        new_category_color: &str,
        user: &Uuid,
    ) -> Result<NoteListRes> {
        let notes_collection = db.collection::<NoteModel>("notes");

        let mut cursor = notes_collection
            .find(doc! { "category_id": category_id, "user": user }, None)
            .await?;

        let mut notes_results = Vec::new();

        while let Some(note) = cursor.try_next().await? {
            let update_note_req = UpdateNoteReq {
                category_name: Some(new_category_name.to_string()),
                category_color: Some(new_category_color.to_string()),
                ..Default::default()
            };

            let note_result =
                base::update::<Self, UpdateNoteReq>(db, &note.id.to_hex(), &update_note_req, user)
                    .await
                    .expect("note 업데이트에 실패했습니다.");

            notes_results.push(note_result);
        }

        Ok(NoteListRes {
            status: "success",
            results: notes_results.len(),
            notes: notes_results,
        })
    }

    pub async fn update_notes_for_property_change(
        db: &Database,
        category_id: &str,
        prop_id: &str,
        new_prop_name: &str,
        new_prop_type: &PropertyType,
        user_id: &Uuid,
    ) -> Result<PropValueListRes> {
        let notes_collection = db.collection::<NoteModel>("notes");
        let category_oid = ObjectId::from_str(category_id).map_err(DBError::MongoGetOidError)?;
        let prop_oid = ObjectId::from_str(prop_id).map_err(DBError::MongoGetOidError)?;
        let mut cursor = notes_collection
            .find(doc! { "category_id": category_oid, "user": user_id }, None)
            .await?;

        let mut prop_results = Vec::new();

        while let Some(note) = cursor.try_next().await? {
            if let Some(prop_value) = note.prop_values.iter().find(|p| p.prop_id == prop_oid) {
                let values = prop_value.values.as_ref().map(|val| vec![val.clone()]);

                let update_prop_req = UpdatePropValueReq {
                    name: Some(new_prop_name.to_string()),
                    values,
                    prop_type: Some(new_prop_type.to_owned()),
                };

                let prop_result =
                    PropValueService::update_propV(db, category_id, prop_id, &update_prop_req)
                        .await?;

                prop_results.push(prop_result.data.propV);
            }
        }

        Ok(PropValueListRes {
            status: "success",
            results: prop_results.len(),
            propVs: prop_results,
        })
    }

    // utils
    pub async fn get_prop_values(db: &Database, category_id: &str) -> Result<Vec<PropValueModel>> {
        let properties = PropertyService::fetch_properties(db, category_id)
            .await
            .expect("properties를 가져오는데 실패했습니다.")
            .props;

        let prop_values = properties
            .iter()
            .map(|prop| {
                let prop_oid = ObjectId::from_str(&prop.id)
                    .map_err(DBError::MongoGetOidError)
                    .unwrap();
                PropValueModel {
                    prop_id: prop_oid,
                    prop_name: prop.name.clone(),
                    prop_type: prop.prop_type.clone(),
                    values: None,
                }
            })
            .collect::<Vec<PropValueModel>>();
        Ok(prop_values)
    }

    // subnote
    pub async fn add_page(self, db: &Database, id: &str, user: &Uuid) -> Result<SingleNoteRes> {
        let note_oid = ObjectId::from_str(id).map_err(DBError::MongoGetOidError)?;

        // Retrieve the original note
        let coll = db.collection::<NoteModel>("notes");
        let mut original_note = match coll
            .find_one(doc! { "_id": &note_oid, "user": user }, None)
            .await
        {
            Ok(Some(doc)) => doc,
            Ok(None) => return Err(NotFoundError(note_oid.to_string())),
            Err(e) => return Err(DB(DBError::MongoQueryError(e))),
        };

        // Add the new subnote to the original note's subnotes
        let new_page = PageModel::new_page(&original_note);
        original_note.pages.push(new_page);

        // Update the original note in the database
        let subnotes_bson = bson::to_bson(&original_note.pages)
            .map_err(|e| DB(DBError::MongoSerializeBsonError(e)))?;

        coll.update_one(
            doc! { "_id": &note_oid, "user": user },
            doc! { "$set": { "subnotes": subnotes_bson }},
            None,
        )
        .await?;

        let note_result = Self::convert_doc_to_response(&original_note)?;

        Ok(SingleNoteRes {
            status: "success",
            data: NoteData { note: note_result },
        })
    }

    pub async fn get_subnotes(db: &Database, id: &str, user: &Uuid) -> Result<SingleNoteRes> {
        // Parse the note id
        let note_oid = ObjectId::from_str(id).map_err(DBError::MongoGetOidError)?;

        // Define the aggregation pipeline
        let pipeline = vec![
            doc! { "$match": { "_id": &note_oid, "user": user }},
            doc! {
                "$graphLookup": {
                    "from": "notes",
                    "startWith": "$_id",
                    "connectFromField": "_id",
                    "connectToField": "parent_id",
                    "as": "subnotes",
                    "maxDepth": 5,
                    "depthField": "depth"
                }
            },
        ];

        let options = AggregateOptions::builder().allow_disk_use(true).build();
        let notes_collection = db.collection::<NoteModel>("notes");
        let mut cursor = notes_collection.aggregate(pipeline, options).await?;
        let mut notes = Vec::new();

        while let Some(result) = cursor.try_next().await? {
            notes.push(bson::from_document::<NoteModel>(result));
        }

        // Assuming there's only one note matched, as we queried by _id
        let updated_note = match notes.into_iter().next() {
            Some(Ok(note)) => note,
            Some(Err(e)) => return Err(DB(DBError::MongoDeserializeBsonError(e))),
            None => return Err(NotFoundError(note_oid.to_string())),
        };

        // convert doc to Res
        let note_result = Self::convert_doc_to_response(&updated_note)?;

        // Return the updated note
        Ok(SingleNoteRes {
            status: "success",
            data: NoteData { note: note_result },
        })
    }
}
