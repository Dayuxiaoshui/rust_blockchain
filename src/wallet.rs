use ring::signature::{Ed25519KeyPair, KeyPair};
use rand::RngCore;
use hex;

pub struct Wallet {
    key_pair: Ed25519KeyPair,
}

impl Wallet {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut seed = [0u8; 32];
        rng.fill_bytes(&mut seed);
        
        let key_pair = Ed25519KeyPair::from_seed_unchecked(&seed)
            .expect("Failed to generate key pair");
            
        Wallet { key_pair }
    }

    pub fn public_key(&self) -> String {
        hex::encode(self.key_pair.public_key().as_ref())
    }

    pub fn key_pair(&self) -> &Ed25519KeyPair {
        &self.key_pair
    }
}
