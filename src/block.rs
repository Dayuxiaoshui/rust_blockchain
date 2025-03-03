use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;
use crate::merkle_tree::MerkleTree;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub index: u32,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub merkle_root: String,
}

impl Block {
    pub fn new(index: u32, timestamp: i64, transactions: Vec<Transaction>, previous_hash: String, nonce: u64) -> Self {
        let merkle_tree = MerkleTree::new(transactions.iter().map(|tx| tx.get_message()).collect());
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce,
            merkle_root: merkle_tree.root,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let input = format!("{}{}{}{}{}", self.index, self.timestamp, self.merkle_root, self.previous_hash, self.nonce);
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }

    pub fn genesis() -> Self {
        Block::new(0, 1630000000, Vec::new(), "0".to_string(), 0)
    }
}

