use anyhow::Result;
use crate::utils::serializer::{serialize, hash_to_u8, hash_to_str};
use bigint::U256;
use std::ops::Shl;

// 工作量证明POW
use super::block::Block;

const MAX_NONCE: usize = usize::MAX;

pub struct ProofWork{
    target: U256
}



impl ProofWork {
    pub fn new(bits: usize) -> Self{
        let target = U256::from(1 as usize);
        let target = target.shl(256 - bits);

        Self { 
            target 
        }
    }

    pub fn run(&self,block: &mut Block){
        let mut nonce = 0;
        while nonce < MAX_NONCE {
            if let Ok(pre_hash) = Self::prepare_data(block, nonce){
                let mut hash_u8:[u8;32] = [0;32];
                hash_to_u8(&pre_hash, &mut hash_u8);
                let pre_hash_int = U256::from(hash_u8.as_slice());

                if pre_hash_int.lt(&(self.target)){
                    block.set_hash(hash_to_str(&pre_hash));
                    break;
                }else {
                    nonce += 1;
                }
            }
        }
    }

    fn prepare_data(block: &mut Block,nonce: usize) -> Result<Vec<u8>>{
        block.get_nonce(nonce);
        Ok(serialize(&(block.get_header())).unwrap())
    }
}