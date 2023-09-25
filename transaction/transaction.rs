use crate::block::blockchain::Blockchain;
use crate::wallet::wallet::hash_public_key;
use crate::wallet::wallets::Wallets;
use crate::{transaction::txin::TxIn, storage::Storage};
use crate::transaction::txout::TxOut;
use serde::{Deserialize,Serialize};
use crate::utils::serializer::{serialize,hash_to_str};
use crate::utils::encryption::{sign_digest,verification};

use super::ux_set::UtxoSet;

pub const SUBSIDY: i32 = 10;


#[derive(Debug,Serialize, Deserialize, Clone, Default,PartialEq, Eq)]
pub struct Transaction{
    version: i64,
    pub id: String,
    pub txin: Vec<TxIn>,
    pub txout: Vec<TxOut>,
}

impl Transaction{
    pub fn new_coinbase(recipent: &str) -> Self{
        let txin = TxIn::default();
        let txout = TxOut::new(SUBSIDY, recipent);
        let mut tx = Transaction{
            version: 1,
            id: String::new(),
            txin: vec![txin],
            txout: vec![txout]
        };

        tx.set_hash();
        tx
    }

    pub fn new_utxo<T: Storage>(from: &str, to:&str, amout: i32, utxo_set: &UtxoSet<T>,bc:&Blockchain<T>) -> Self{
        let wallets = Wallets::new().unwrap();
        let wallet = wallets.get_wallet(from).unwrap();
        let public_key_hash = hash_public_key(wallet.get_public_key());

        let (accumulated, vaild_output) = utxo_set.find_spendable_outputs(&public_key_hash, amout);
        if accumulated < amout{
            panic!("your amout not enough");
        }

        let mut inputs = vec![];
        for (txid,output) in vaild_output{
            for id in output{
                let input = TxIn::new(txid.clone(), id.clone(),wallet.get_public_key().to_vec());
                inputs.push(input);
            }
        }

        let mut outputs = vec![TxOut::new(amout, to)];
        if accumulated > amout {
            outputs.push(TxOut::new(accumulated - amout, &from));
        }

        let mut tx = Transaction{
            version:1,
            id: String::new(),
            txin: inputs,
            txout: outputs
        };
        tx.set_hash();
        tx.sign(bc, wallet.get_pkcs());
        
        tx

    }

    fn sign<T:Storage>(&mut self, bc: &Blockchain<T>, pkcs8: &[u8]) {
        let mut tx_copy = self.trimmed_copy();

        for (idx, vin) in self.txin.iter_mut().enumerate() {
            // 查找输入引用的交易
            let prev_tx_option = bc.get_transaction(vin.get_txid());
            if prev_tx_option.is_none() {
                panic!("ERROR: Previous transaction is not correct")
            }
            let prev_tx = prev_tx_option.unwrap();
            tx_copy.txin[idx].set_signature(vec![]);
            tx_copy.txin[idx].set_public_key(prev_tx.txout[vin.get_vout()].get_pub_key());
            tx_copy.set_hash();

            tx_copy.txin[idx].set_public_key(&vec![]);

            // 使用私钥对数据签名
            let signature = sign_digest(pkcs8, tx_copy.id.as_bytes());
            vin.set_signature(signature);
        }
    }

    pub fn verify<T: Storage>(&self, bc: &Blockchain<T>) -> bool {
        if self.is_coinbase() {
            return true;
        }
        let mut tx_copy = self.trimmed_copy();
        for (idx, vin) in self.txin.iter().enumerate() {
            let prev_tx_option = bc.get_transaction(vin.get_txid());
            if prev_tx_option.is_none() {
                panic!("ERROR: Previous transaction is not correct")
            }
            let prev_tx = prev_tx_option.unwrap();
            tx_copy.txin[idx].set_signature(vec![]);
            tx_copy.txin[idx].set_public_key(prev_tx.txout[vin.get_vout()].get_pub_key());
            tx_copy.set_hash();

            tx_copy.txin[idx].set_public_key(&vec![]);

            // 使用公钥验证签名
            let verify = verification(
                &vin.get_public_key(),
                &vin.get_signature(),
                tx_copy.id.as_bytes(),
            );
            if !verify {
                return false;
            }
        }
        true
    }


    pub fn is_coinbase(&self) -> bool{
        self.txin.len() == 1 && self.txin[0].get_public_key().len() == 0
    }

    fn trimmed_copy(&self) -> Transaction{
        let mut inputs = vec![];
        let mut outputs = vec![];
        for input in &self.txin{
            let txin = TxIn::new(input.get_txid(), input.get_vout(), vec![]);
            inputs.push(txin);
        }

        for output in &self.txout {
            outputs.push(output.clone());
        }

        Transaction{
            version:1,
            id: self.id.clone(),
            txin:inputs,
            txout:outputs
        }

    }

    pub fn set_hash(&mut self){
        if let Ok(addr) = serialize(self){
            self.id = hash_to_str(&addr)
        }
    }
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_vout(&self) -> &[TxOut] {
        self.txout.as_slice()
    }

    pub fn get_vin(&self) -> &[TxIn] {
        self.txin.as_slice()
    }
}