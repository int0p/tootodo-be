pub mod req {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateMemoReq {
        pub title: String,
        pub color: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateMemoReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub content: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub color: Option<String>,
    }
}

pub mod res {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use crate::domain::memo::MemoModel;

    #[allow(non_snake_case)]
    #[derive(Deserialize, Serialize, Debug)]
    pub struct MemoRes {
        pub id: String,
        pub user: Uuid,
        pub title: String,
        pub content: String,
        pub color: String,
        pub createdAt: DateTime<Utc>,
        pub updatedAt: DateTime<Utc>,
    }

    impl MemoRes {
        pub fn from_model(memo: &MemoModel) -> Self {
            Self {
                id: memo.id.to_hex(),
                user: memo.user,
                title: memo.title.to_owned(),
                content: memo.content.to_owned(),
                color: memo.color.to_owned(),
                createdAt: memo.createdAt,
                updatedAt: memo.updatedAt,
            }
        }
    }
    #[derive(Serialize, Debug)]
    pub struct MemoData {
        pub memo: MemoRes,
    }

    #[derive(Serialize, Debug)]
    pub struct SingleMemoRes {
        pub status: &'static str,
        pub data: MemoData,
    }

    #[derive(Serialize, Debug)]
    pub struct MemoListRes {
        pub status: &'static str,
        pub results: usize,
        pub memos: Vec<MemoRes>,
    }
}
