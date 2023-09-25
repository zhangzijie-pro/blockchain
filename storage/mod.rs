use crate::{error::BlockchainError, block::block::Block};
use std::collections::HashMap;
use crate::transaction::txout::TxOut;
pub mod sleddb;
pub use sleddb::SledDb;

pub const UTXO_SET: &str = "utxos";
pub const TOP_KEY: &str = "tip_key";
pub const HEIGHT: &str = "height";
pub const TABLE_OF_BLOCKS: &str = "blocks";

pub trait Storage: Send + Sync + 'static {
    fn get_tips(&self) -> Result<Option<String>,BlockchainError>;
    fn get_block(&self,key:&str) -> Result<Option<Block>,BlockchainError>;
    fn get_height(&self) -> Result<Option<usize>,BlockchainError>;
    fn update_blocks(&self,key:&str,block:&Block,height:usize);
    fn get_block_iter(&self) -> Result<Box<dyn Iterator<Item = Block>>,BlockchainError>;

    fn get_utxo_set(&self) -> HashMap<String, Vec<TxOut>>;
    fn write_utxo(&self, txid: &str, outs: Vec<TxOut>) -> Result<(), BlockchainError>;
    fn clear_utxo_set(&self);
}

pub struct StorageIterator<T>{
    data: T
}

impl<T> StorageIterator<T>{
    fn new(data : T)->Self{
        Self { data }
    }
}

impl<T> Iterator for StorageIterator<T>
where
    T: Iterator,
    T::Item : Into<Block>
{
    type Item = Block;
    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().map(|v| v.into())
    }
}