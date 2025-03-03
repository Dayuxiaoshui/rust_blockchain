use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Serialize, Deserialize};
use hex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub signature: Vec<u8>,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: u64) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
            signature: Vec::new(),
        }
    }

    pub fn sign(&mut self, key_pair: &Ed25519KeyPair) {
        self.signature = key_pair.sign(self.get_message().as_ref()).as_ref().to_vec();
    }

    pub fn verify(&self) -> bool {
        let public_key_bytes = match hex::decode(&self.sender) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };
        
        let public_key = ring::signature::UnparsedPublicKey::new(
            &ring::signature::ED25519, 
            &public_key_bytes
        );
        
        public_key.verify(
            self.get_message().as_ref(), 
            self.signature.as_slice()
        ).is_ok()
    }

    pub fn get_message(&self) -> Vec<u8> {
        format!("{}{}{}", self.sender, self.receiver, self.amount).into_bytes()
    }
}
