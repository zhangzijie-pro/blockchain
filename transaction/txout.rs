use serde::{Deserialize,Serialize};

use crate::utils::encryption::base58_deencode;
use crate::wallet::wallet::ADDRESS_CHECKSUM_LEN;

#[derive(Debug,Deserialize,Serialize,Clone,Default,PartialEq, Eq)]
pub struct TxOut{
    value: i32,
    pub_key: Vec<u8>
}

#[allow(dead_code)]
impl TxOut{
    pub fn new(value:i32, to_addr: &str) -> Self{
        let mut txout = TxOut{
            value,
            pub_key: vec![]
        };

        txout.lock(to_addr);
        txout

    }

    pub fn lock(&mut self,address: &str){
        let payload = base58_deencode(address);
        let pub_key_hash = payload[0..payload.len()-ADDRESS_CHECKSUM_LEN].to_vec();
        self.pub_key = pub_key_hash
    }
    
    pub fn is_lock(&self, pub_key:&[u8]) -> bool{
        self.pub_key.eq(pub_key)
    }

    pub fn get_value(&self) -> i32{
        self.value.clone()
    }

    pub fn get_pub_key(&self) -> &[u8]{
        self.pub_key.as_slice()
    }
}