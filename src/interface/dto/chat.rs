pub mod req{
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

pub mod res{

}