use sha2::{Sha256, Digest};

pub struct MerkleTree {
    pub root: String,
}

impl MerkleTree {
    pub fn new(transactions: Vec<Vec<u8>>) -> Self {
        if transactions.is_empty() {
            return MerkleTree { root: String::new() };
        }
        let mut hashes: Vec<String> = transactions.into_iter().map(|tx| Self::hash(tx)).collect();
        while hashes.len() > 1 {
            let mut next_level = Vec::new();
            for i in (0..hashes.len()).step_by(2) {
                let left = &hashes[i];
                let right = if i + 1 < hashes.len() { &hashes[i + 1] } else { left };
                let combined = format!("{}{}", left, right);
                next_level.push(Self::hash(combined.into_bytes()));
            }
            hashes = next_level;
        }
        MerkleTree { root: hashes.pop().unwrap_or_default() }
    }

    fn hash(data: Vec<u8>) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}

