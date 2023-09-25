use ring::signature::{KeyPair,EcdsaKeyPair,ECDSA_P256_SHA256_FIXED_SIGNING};
use serde::{Serialize,Deserialize};
use crate::utils::encryption::{new_private_key,base58_encode,sha3_digest,ripem160_digest};

pub const VERSION:u8 = 0x00;
pub const ADDRESS_CHECKSUM_LEN:usize = 4;

#[derive(Serialize,Deserialize,Clone)]
pub struct Wallet{
    pkcs:Vec<u8>,
    public_key: Vec<u8>
}


impl Wallet {
    pub fn new() -> Self{
        let pkcs = new_private_key();
        let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &pkcs.as_ref()).unwrap();
        let public_key = key_pair.public_key().as_ref().to_vec();

        Self{
            pkcs,
            public_key
        }
    }

    pub fn get_address(&self) -> String{
        let pub_key_hash = hash_public_key(&self.public_key.as_slice());
        let mut payload= vec![];
        payload.push(VERSION);
        payload.extend(pub_key_hash.as_slice());
        let checksum = checksum(payload.as_slice());
        payload.extend(checksum);
        base58_encode(payload.as_slice())
    }

    pub fn get_pkcs(&self) -> &[u8]{
        &self.pkcs.as_slice()
    }

    pub fn get_public_key(&self) -> &[u8]{
        &self.public_key.as_slice()
    }
}

pub fn hash_public_key(data: &[u8]) -> Vec<u8>{
    let pk_sha256 = sha3_digest(data);
    let result = ripem160_digest(&pk_sha256);
    result
}

pub fn checksum(payload: &[u8]) -> Vec<u8> {
    let first_sha = sha3_digest(payload);
    let second_sha = sha3_digest(&first_sha);
    second_sha[0..ADDRESS_CHECKSUM_LEN].to_vec()
}