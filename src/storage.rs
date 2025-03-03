use std::fs;
use serde_json;
use crate::blockchain::Blockchain;

pub fn save_blockchain(blockchain: &Blockchain, filename: &str) {
    let json = serde_json::to_string(blockchain.chain()).unwrap();
    fs::write(filename, json).unwrap();
}

pub fn load_blockchain(filename: &str, difficulty: usize) -> Blockchain {
    let json = fs::read_to_string(filename).unwrap_or_else(|_| "[]".to_string());
    let chain: Vec<crate::block::Block> = serde_json::from_str(&json).unwrap_or_default();
    
    let mut blockchain = Blockchain::new(difficulty);
    
    if !chain.is_empty() {
        *blockchain.chain_mut() = chain;
    }
    
    blockchain
}

