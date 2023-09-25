use std::{sync::{RwLock, Arc,atomic::{AtomicUsize, Ordering}}, collections::HashMap};
use crate::{storage::{Storage,SledDb}, transaction::{transaction::Transaction, txout::TxOut}, error::BlockchainError};
use crate::block::block::Block;
use tracing::info;


const BITS:usize = 800;

pub struct Blockchain<T = SledDb> {
    storage: Arc<T>,
    tip: Arc<RwLock<String>>,
    height: AtomicUsize,
}

impl<T: Storage> Blockchain<T> {
    pub fn new(storage: Arc<T>) -> Self {
        if let Ok(Some(tip)) = storage.get_tips() {
            let height = storage.get_height().unwrap();
            Self {
                storage,
                tip: Arc::new(RwLock::new(tip)),
                height: AtomicUsize::new(height.unwrap()),
            }
        }else { 
           Self {
                storage,
                tip: Arc::new(RwLock::new(String::new())),
                height: AtomicUsize::new(0),
            }
        }
    }

    pub fn create_genesis_block(&mut self, genesis_addr: &str) {
        let genesis_block = Block::create_genesis_block(BITS, genesis_addr);
        let hash = genesis_block.get_hash();
        self.height.fetch_add(1, Ordering::Relaxed);
        self.storage.update_blocks(&hash, &genesis_block, self.height.load(Ordering::Relaxed));
        let mut tip = self.tip.write().unwrap();
        *tip = hash;
    }

    pub fn mine_block(&mut self, txs: &[Transaction]) -> Block{
        for tx in txs {
            if tx.verify(self) == false {
                panic!("ERROR: Invalid transaction")
            }
        }

        let block = Block::new(txs, &self.tip.read().unwrap(), BITS);
        let hash = block.get_hash();
        self.height.fetch_add(1, Ordering::Relaxed);
        self.storage.update_blocks(&hash, &block, self.height.load(Ordering::Relaxed));
        let mut tip = self.tip.write().unwrap();
        *tip = hash;

        info!("mining block!... ... ...");
        block
  
    }

    pub fn add_block(&mut self, block: Block) -> Result<(), BlockchainError> {
        let hash = block.get_hash();
        if let Some(_) = self.storage.get_block(&hash)? {
            info!("Block {} already exists", hash);
        }else {
            self.height.fetch_add(1, Ordering::Relaxed);
            self.storage.update_blocks(&hash, &block, self.height.load(Ordering::Relaxed));
            let mut tip = self.tip.write().unwrap();
            *tip = hash;
        }
        Ok(())
    }
    
    pub fn find_utxo(&self) -> HashMap<String,Vec<TxOut>>{
        let mut utxo = HashMap::new();
        let mut spent_utxo = HashMap::new();

        let blocks = self.storage.get_block_iter().unwrap();
        for block in blocks{
            for txs in block.get_tran(){
                for (idx,txout) in txs.get_vout().iter().enumerate(){
                    if let Some(outs) = spent_utxo.get(&txs.get_id()){
                        for out in outs{
                            if idx.eq(out){
                                break;
                            }

                            utxo.entry(txs.get_id())
                                .and_modify(|v: &mut Vec<TxOut>| v.push(txout.clone()))
                                .or_insert(vec![txout.clone()]);
                        }
                    }else {
                        utxo.entry(txs.get_id())
                        .and_modify(|v: &mut Vec<TxOut>| v.push(txout.clone()))
                        .or_insert(vec![txout.clone()]);
                    }
                }
                
                for txin in txs.get_vin(){
                    spent_utxo.entry(txin.get_txid())
                    .and_modify(|v: &mut Vec<usize>| v.push(txin.get_vout().clone()))
                    .or_insert(vec![txin.get_vout().clone()]);
                }
            }
        }

        utxo
    }

    pub fn get_transaction(&self,txid:String) -> Option<Transaction>{
        let blocks = self.storage.get_block_iter().unwrap();
        for block in blocks{
            for tx in block.get_tran(){
                if tx.get_id() == txid{
                    return Some(tx.clone());
                }
            }
        }
        None
    }

    pub fn blocks_info(&self) {
        let blocks = self.storage.get_block_iter().unwrap();
        for block in blocks {
            info!("{:#?}", block);
        }
    }

    pub fn get_blocks(&self) -> Vec<Block> {
        self.storage.get_block_iter().unwrap().collect()
    }
    
    pub fn get_tip(&self) -> String {
        self.tip.read().unwrap().to_string()
    }

    pub fn get_height(&self) -> usize {
        self.height.load(Ordering::Relaxed)
    }
}