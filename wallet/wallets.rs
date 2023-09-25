use std::{collections::HashMap, env::current_dir, fs};

use serde::{Serialize, Deserialize};
use tracing::info;

use crate::utils::serializer::{deserialize,serialize};
use crate::wallet::wallet::Wallet;
use crate::error::BlockchainError;

pub const WALLET_FILE:&str = "wallet.dat";

#[derive(Serialize,Deserialize)]
pub struct Wallets{
    pub wallet: HashMap<String,Wallet>,
}

impl Wallets {
    pub fn new() -> Result<Self,BlockchainError>{
        let wallets = Self::load_wallet_from_file();
        wallets
    }

    pub fn create_new_wallet(&mut self) -> String{
        let wallet = Wallet::new();
        let address = wallet.get_address();
        self.wallet.insert(address.clone(), wallet);
        Self::load_wallet_from_file().unwrap();
        address
    }

    pub fn get_wallet(&self,address: &str) -> Option<&Wallet>{
        self.wallet.get(address)
    }

    pub fn get_address(&self) -> Vec<&String>{
        self.wallet.keys().collect()
    }

    pub fn save_wallet_to_file(&self) -> Result<(),BlockchainError>{
        let path = current_dir().unwrap().join(WALLET_FILE);
        let wallet_ser = serialize(&self)?;
        fs::write(path, &wallet_ser).unwrap();
        Ok(())
    }

    pub fn load_wallet_from_file() -> Result<Self,BlockchainError>{
        let path = current_dir().unwrap().join(WALLET_FILE);
        info!("wallet: {:?}",path);
        if !path.exists(){
            let wallets = Wallets{
                wallet: HashMap::new()
            };

            return Ok(wallets);
        }

        let wallets_ser = fs::read(&path).unwrap();
        let wallets = deserialize(&wallets_ser);
        wallets
    }
}