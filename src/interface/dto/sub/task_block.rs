pub mod req {
    use crate::infra::types::BlockType;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreateBlockReq {
        pub src_task_id: String,
        pub block_type: BlockType,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct UpdateBlockReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub src_task_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub block_type: Option<BlockType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub body: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct FilterBlockReq {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub src_task_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub block_type: Option<BlockType>,
    }
}

pub mod res {
    use crate::domain::sub::task_block::BlockModel;
    use crate::infra::types::BlockType;
    use serde::{Deserialize, Serialize};

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize, Debug)]
    pub struct BlockRes {
        pub id: String,
        pub src_task_id: String,
        pub block_type: BlockType,
        pub body: String,
    }

    impl BlockRes {
        pub fn from_model(block: &BlockModel) -> Self {
            Self {
                id: block.id.to_hex(),
                src_task_id: block.src_task_id.to_hex(),
                block_type: block.block_type.to_owned(),
                body: block.body.clone(),
            }
        }
    }

    #[derive(Serialize, Debug)]
    pub struct BlockData {
        pub block: BlockRes,
    }

    #[derive(Serialize, Debug)]
    pub struct SingleBlockRes {
        pub status: &'static str,
        pub data: BlockData,
    }

    #[derive(Serialize, Debug)]
    pub struct BlockListRes {
        pub status: &'static str,
        pub results: usize,
        pub blocks: Vec<BlockRes>,
    }
}
