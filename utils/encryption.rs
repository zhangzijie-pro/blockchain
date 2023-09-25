use std::iter::repeat;

use crypto::digest::Digest;
use crypto::ripemd160::Ripemd160;
use ring::rand::SystemRandom;
use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING, ECDSA_P256_SHA256_FIXED};
use ring::digest::{Context, SHA256};

pub fn new_private_key() -> Vec<u8>{
    let rng = SystemRandom::new();
    let pcks = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
    pcks.as_ref().to_vec()
}

pub fn sha3_digest(data:&[u8]) -> Vec<u8>{
    let mut context = Context::new(&SHA256);
    context.update(data);
    context.finish().as_ref().to_vec()
}

pub fn ripem160_digest(data:&[u8]) -> Vec<u8>{
    let mut ripe = Ripemd160::new();
    ripe.input(data);
    let mut buf: Vec<u8> = repeat(0).take(ripe.output_bytes()).collect();
    ripe.result(&mut buf);
    return buf;
}

pub fn base58_encode(data:&[u8]) -> String {
        bs58::encode(data).into_string()
}

pub fn base58_deencode(data:&str) -> Vec<u8>{
    bs58::decode(data).into_vec().unwrap()
}

pub fn sign_digest(pkcs:&[u8],message: &[u8]) -> Vec<u8>{
    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs).unwrap();
    let rng = SystemRandom::new();
    key_pair.sign(&rng, message).unwrap().as_ref().to_vec()
}

pub fn verification(public_key:&[u8],signature:&[u8],message: &[u8]) -> bool{
    let public_key = ring::signature::UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, public_key);
    let result = public_key.verify(message, signature.as_ref());
    result.is_ok()
}