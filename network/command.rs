use serde::{Deserialize,Serialize};
use crate::block::block::Block;

#[derive(Debug,Serialize,Deserialize)]
pub enum Commands{
    Genesis(String),
    Blocks(String),
    Sync(String),
    CreateWallet(String),
    Address(String),
    Trans{
        from: String,
        to : String ,
        amount:String
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub enum Message {
    Version{
        best_height:usize,
        from_addr:String
    },
    Blocks{
        blocks:Vec<Block>,
        height:usize,
        to_addr: String,
    },
    Block{
        block: Block
    }
}