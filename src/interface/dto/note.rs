pub mod req {
    use crate::domain::{
        note::NoteModel, sub::{note_block::BlockModel, note_page::PageModel, note_propV::PropValueModel}
    };
    use uuid::Uuid;

    use crate::infra::types::ChatType;
    use chrono::{DateTime, NaiveDate, Utc};
    use mongodb::bson::{self, Document};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateNoteReq {
        pub title: String,
        pub category_id: String,
        pub category_color: String,
        pub category_name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub prop_values: Option<Vec<PropValueModel>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pages: Option<Vec<PageModel>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parent_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub chat_type: Option<ChatType>,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct UpdateNoteReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub category_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub category_color: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub category_name: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub parent_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pages: Option<Vec<NoteModel>>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub chat_type: Option<ChatType>,
    }

    #[allow(non_snake_case)]
    #[derive(Deserialize, Serialize, Debug)]
    pub struct NoteFetchOptions {
        pub id: String,
        pub user: Uuid,
        pub title: String,

        pub category_id: String,
        pub prop_values: Vec<PropValueModel>,

        pub pages: Vec<PageModel>,
        pub parent_id: String,

        pub createdAt: DateTime<Utc>,
        pub updatedAt: DateTime<Utc>,
    }

    impl NoteFetchOptions {
        pub fn build_projection() -> Document {
            let mut projection = Document::new();

            let fields = vec![
                "_id",
                "user",
                "title",
                "category_id",
                "prop_values",
                "pages",
                "parent_id",
                "createdAt",
                "updatedAt",
            ];

            for field in fields {
                projection.insert(field, 1);
            }

            projection
        }
    }
}

pub mod res {
    use crate::domain::{
        note::NoteModel, sub::{chat::MsgModel,  note_page::PageModel, note_propV::PropValueModel}
    };
    use crate::infra::types::ChatType;

    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[allow(non_snake_case)]
    #[derive(Serialize, Debug)]
    pub struct NoteRes {
        pub id: String,
        pub user: Uuid,
        pub title: String,

        pub category_id: String,
        pub category_color: String,
        pub category_name: String,
        pub prop_values: Vec<PropValueModel>,

        pub pages: Vec<PageModel>,
        pub connected_task: Option<String>,
        pub parent_id: Option<String>,

        pub chat_type: ChatType,
        pub chat_msgs: Option<Vec<MsgModel>>,

        pub createdAt: DateTime<Utc>,
        pub updatedAt: DateTime<Utc>,
    }

    impl NoteRes {
        pub fn from_model(note: &NoteModel) -> Self {
            Self {
                id: note.id.to_hex(),
                user: note.user,
                title: note.title.to_owned(),
                category_id: note.category_id.to_hex(),
                category_color: note.category_color.to_owned(),
                category_name: note.category_name.to_owned(),
                prop_values: note.prop_values.clone(),
                pages: note.pages.to_owned(),
                connected_task: note.connected_task.as_ref().map(|id| id.to_hex()),
                parent_id: note.parent_id.as_ref().map(|id| id.to_hex()),
                chat_type: note.chat_type.to_owned(),
                chat_msgs: note.chat_msgs.to_owned(),
                createdAt: note.createdAt,
                updatedAt: note.updatedAt,
            }
        }
    }

    #[derive(Serialize, Debug)]
    pub struct NoteData {
        pub note: NoteRes,
    }

    #[derive(Serialize, Debug)]
    pub struct SingleNoteRes {
        pub status: &'static str,
        pub data: NoteData,
    }

    #[derive(Serialize, Debug)]
    pub struct NoteListRes {
        pub status: &'static str,
        pub results: usize,
        pub notes: Vec<NoteRes>,
    }
}
