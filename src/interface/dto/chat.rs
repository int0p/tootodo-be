pub mod req {
    use crate::domain::types::{ChatType, MsgType};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateChatReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub src_type: Option<ChatType>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FilterChatReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub src_item_type: Option<ChatType>,
    }

    // msg
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateMsgReq {
        pub msg_type: MsgType,
        pub content: String,
        pub booked: bool,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateMsgReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub msg_type: Option<MsgType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub content: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub booked: Option<bool>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FilterMsgReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub src_chat_type: Option<ChatType>,
    }
}

pub mod res {
    use crate::domain::{
        sub::chat::MsgModel,
        types::{ChatType, MsgType},
    };
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct MsgRes {
        pub id: String,
        pub msg_type: MsgType,
        pub content: String,
        pub created_at: DateTime<Utc>,
        pub booked: bool,
        pub chat_type: Option<ChatType>,
        pub chat_msgs: Option<Vec<MsgModel>>,
    }

    #[derive(Serialize, Debug)]
    pub struct MsgData {
        pub msg: MsgRes,
    }

    #[derive(Serialize, Debug)]
    pub struct SingleMsgRes {
        pub status: &'static str,
        pub data: MsgData,
    }

    #[derive(Serialize, Debug)]
    pub struct MsgListRes {
        pub status: &'static str,
        pub results: usize,
        pub msgs: Vec<MsgRes>,
    }
}
