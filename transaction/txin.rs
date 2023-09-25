use serde::{Deserialize,Serialize};

use crate::wallet::wallet::hash_public_key;

#[derive(Debug,Deserialize,Serialize,Clone,Default,PartialEq, Eq)]
pub struct TxIn{
    txid: String,
    vout: usize,
    signature:Vec<u8>,
    public_key:Vec<u8>
}

#[allow(dead_code)]
impl TxIn{
    pub fn new(txid: String, vout: usize, pub_key:Vec<u8>) -> Self{
        Self{
            txid,
            vout,
            signature:vec![],
            public_key:pub_key
        }
    }

    pub fn unlock_output(&self, public_key: &[u8]) -> bool{
        let hash_key = hash_public_key(&self.public_key);
        hash_key.eq(public_key)
    }

    pub fn get_txid(&self) -> String{
        self.txid.clone()
    }

    pub fn get_vout(&self) -> usize{
        self.vout.clone()
    }

    pub fn get_signature(&self) -> Vec<u8>{
        self.signature.clone()
    }

    pub fn get_public_key(&self) -> Vec<u8>{
        self.public_key.clone()
    }

    pub fn set_signature(&mut self,signature:Vec<u8>){
        self.signature = signature
    }

    pub fn set_public_key(&mut self,pub_key:&[u8]){
        self.public_key = pub_key.to_vec()
    }
}