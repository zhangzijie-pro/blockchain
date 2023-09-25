use std::{collections::HashMap, sync::Arc};

use crate::storage::Storage;
use crate::block::blockchain::Blockchain;


pub struct UtxoSet<T>{
    storage: Arc<T>
}

impl <T:Storage> UtxoSet<T> {
    pub fn new(storage: Arc<T>) -> Self{
        Self{
            storage
        }
    }
    
    pub fn reindex(&self,bc: &Blockchain<T>) -> Result<(),Blockchain>{
        self.storage.clear_utxo_set();
        let map = bc.find_utxo();
        for (txid,outs) in map{
            self.storage.write_utxo(&txid, outs).unwrap();
        }

        Ok(())
    }

    pub fn find_spendable_outputs(&self, public_key_hash: &[u8],amout: i32) -> (i32,HashMap<String,Vec<usize>>){
        let mut unspent_uxto = HashMap::new();
        let mut accumulated = 0;
        let uxset = self.storage.get_utxo_set();

        for (idx,outs) in uxset.iter(){
            for (id,out) in outs.iter().enumerate(){
                if out.is_lock(public_key_hash) && accumulated < amout{
                    accumulated+= out.get_value();
                    unspent_uxto.entry(idx.to_string())
                        .and_modify(|v: &mut Vec<usize>| v.push(id) )
                        .or_insert(vec![id]);
                }
            }
        }

        (accumulated,unspent_uxto)
    }
}