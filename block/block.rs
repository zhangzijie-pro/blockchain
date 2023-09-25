use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::{block::pow::ProofWork, transaction::transaction::Transaction, utils::serializer::{serialize, hash_to_str}};

#[derive(Debug, Serialize, Deserialize,Eq,PartialEq,Clone,Default)]
pub struct BlockHeader{
    pub timestamp: i64,
    pub prev_hash: String, 
    pub txin_hash: String,
    pub bits:usize,
    pub nonce: usize
}

impl BlockHeader {
    fn new(prev_hash: &str, bits: usize) -> Self {
        Self {
            timestamp: Utc::now().timestamp(),
            prev_hash: prev_hash.into(),
            txin_hash:String::new(),
            bits,
            nonce: 0,
        }
    }
}

#[derive(Debug,Serialize, Deserialize,Eq,PartialEq,Default,Clone)]
pub struct Block{
    pub header: BlockHeader,
    pub transaction: Vec<Transaction>,
    pub hash: String,
}

#[allow(dead_code)]
impl Block{
    pub fn new(txs: &[Transaction],prev_hash: &str,bits: usize) -> Self{
        let mut block =  Block{
            header: BlockHeader::new(prev_hash, bits),
            transaction: txs.to_vec(),
            hash: String::new(),
        };
        block.set_transaction_hash(txs);

        let pow = ProofWork::new(bits);
        pow.run(&mut block);

        block
    }

    pub fn create_genesis_block(bits: usize,addr_own: &str) -> Self{
        let coinbase = Transaction::new_coinbase(addr_own);
        Self::new(&vec![coinbase], "",bits)
    }

    pub fn get_header(&self)  -> BlockHeader{
        self.header.clone()
    }
    
    pub fn get_hash(&self) -> String{
        self.hash.clone()
    }

    pub fn get_nonce(&mut self,nonce: usize){
        self.header.nonce = nonce;
    }

    pub fn set_hash(&mut self,hash: String){
        self.hash = hash;
    }

    pub fn set_transaction_hash(&mut self,txs: &[Transaction]){
        if let Ok(tx) = serialize(txs){
            self.header.txin_hash = hash_to_str(&tx)
        }
    }

    pub fn get_tran(&self) -> &[Transaction]{
        &self.transaction
    }
}