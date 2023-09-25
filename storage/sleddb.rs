use std::collections::HashMap;
use std::path::Path;
use sled::{Db,IVec,transaction::TransactionResult};
use crate::transaction::txout::TxOut;
use crate::error::BlockchainError;
use crate::storage::{TABLE_OF_BLOCKS,TOP_KEY,HEIGHT,UTXO_SET};
use crate::block::block::Block;
use crate::utils::serializer::{deserialize,serialize};

use super::{Storage, StorageIterator};

pub struct SledDb{
    db:Db
}

impl SledDb {
    pub fn new(path: impl AsRef<Path>) -> Self{
        Self{
            db: sled::open(path).unwrap()
        }
    }

    pub fn get_full_key(key: &str,table: &str) -> String{
        format!("{}:{}",table,key)
    }
}

#[allow(unused_must_use)]
impl Storage for SledDb{
    fn get_tips(&self) -> Result<Option<String>,BlockchainError> {
        let result = self.db.get(TOP_KEY).unwrap().map(|v| deserialize::<String>(&v.to_vec()));
        result.map_or(Ok(None), |v| v.map(Some))
    }

    fn get_block(&self,key:&str) -> Result<Option<Block>,BlockchainError> {
        let name = Self::get_full_key(TABLE_OF_BLOCKS, key);
        let result = self.db.get(name).unwrap().map(|v| v.into());
        Ok(result)
    }

    fn get_height(&self) -> Result<Option<usize>,BlockchainError> {
        let height = self.db.get(HEIGHT).unwrap().map(|v| deserialize::<usize>(&v.to_vec()));
        height.map_or(Ok(None), |v| v.map(Some))    
    }

    fn update_blocks(&self,key:&str,block:&Block,height:usize) {
        let _ : TransactionResult<(),()> = self.db.transaction(|db| {
            let name = Self::get_full_key(TABLE_OF_BLOCKS, key);
            db.insert(name.as_str(), serialize(block).unwrap());
            db.insert(TOP_KEY, serialize(key).unwrap());
            db.insert(HEIGHT, serialize(&height).unwrap());
            db.flush();
            Ok(())
        });
    }

    fn get_block_iter(&self) -> Result<Box<dyn Iterator<Item = Block>>,BlockchainError> {
        let prefix = format!("{}:", TABLE_OF_BLOCKS);
        let iter = StorageIterator::new(self.db.scan_prefix(prefix));
        Ok(Box::new(iter))
    }


    fn clear_utxo_set(&self) {
        let prefix = format!("{}:", UTXO_SET);
        self.db.remove(prefix).unwrap();
    }

    fn get_utxo_set(&self) -> std::collections::HashMap<String, Vec<crate::transaction::txout::TxOut>> {
        let mut map = HashMap::new();

        let prefix = format!("{}:",UTXO_SET);

        for item in self.db.scan_prefix(prefix){
            let (k,v) = item.unwrap();
            let txid = String::from_utf8(k.to_vec()).unwrap();
            let txid = txid.split(":").collect::<Vec<_>>()[1].into();
            let outputs = deserialize::<Vec<TxOut>>(&v.to_vec()).unwrap();

            map.insert(txid, outputs);
        }

        map
    }

    fn write_utxo(&self, txid: &str, outs: Vec<crate::transaction::txout::TxOut>) -> Result<(), BlockchainError> {
        let name = format!("{}: {}",UTXO_SET,txid);
        self.db.insert(name, serialize(&outs)?);
        Ok(())
    }
}

impl From<IVec> for Block{
    fn from(value: IVec) -> Self {
        let result = deserialize::<Block>(&value.to_vec());
        match result {
            Ok(block) => block,
            Err(_) => Block::default()
        }
    }
}

impl From<Result<(IVec,IVec),sled::Error>> for Block {
    fn from(value: Result<(IVec,IVec),sled::Error>) -> Self {
        match value {
            Ok((_, v)) => match deserialize::<Block>(&v.to_vec()) {
                Ok(block) => block,
                Err(_) => Block::default(),
        },
        Err(_) => Block::default(),
        }
    }
}