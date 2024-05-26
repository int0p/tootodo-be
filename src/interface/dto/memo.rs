pub mod req {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateMemoReq {
        pub user: Uuid,
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
    
    #[derive(Deserialize, Debug, Default)]
    pub struct FilterOptions {
        pub page: Option<usize>,
        pub limit: Option<usize>,
    }
}

pub mod res {
    use chrono::{DateTime, Utc};
    use serde::Serialize;
    use uuid::Uuid;

    #[derive(Serialize)]
pub struct GenericRes {
    pub status: String,
    pub message: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct MemoRes {
    pub id: String,
    pub user: Uuid,
    pub title: String,
    pub content: String,
    pub color: String,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
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
