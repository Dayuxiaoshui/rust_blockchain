use crate::block::Block;

#[derive(Debug, Clone)]
pub struct ProofOfWork {
    pub difficulty: usize,
}

impl ProofOfWork {
    pub fn new(difficulty: usize) -> Self {
        ProofOfWork { difficulty }
    }

    pub fn mine_block(&self, mut block: Block) -> Block {
        let target = "0".repeat(self.difficulty);
        while !block.hash.starts_with(&target) {
            block.nonce += 1;
            block.hash = block.calculate_hash();
        }
        block
    }

    pub fn is_valid(&self, block: &Block) -> bool {
        block.hash.starts_with(&"0".repeat(self.difficulty))
    }
}
