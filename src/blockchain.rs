use std::collections::HashMap;
use crate::block::Block;
use crate::pow::ProofOfWork;
use crate::transaction::Transaction;
use crate::smart_contract::SmartContract;
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct Blockchain {
    chain: Vec<Block>,
    pow: ProofOfWork,
    block_map: HashMap<String, Block>,
    smart_contract: SmartContract,
    pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let pow = ProofOfWork::new(difficulty);
        let genesis_block = Block::genesis();
        let mined_block = pow.mine_block(genesis_block);

        let mut block_map = HashMap::new();
        block_map.insert(mined_block.hash.clone(), mined_block.clone());

        Blockchain {
            chain: vec![mined_block],
            pow,
            block_map,
            smart_contract: SmartContract,
            pending_transactions: Vec::new(),
        }
    }

    pub fn add_block(&mut self, miner_address: &str) {
        let previous_block = self.chain.last().expect("Chain should have genesis block");
        
        let mut transactions = std::mem::take(&mut self.pending_transactions)
            .into_iter()
            .filter(|tx| tx.verify())
            .collect::<Vec<_>>();

        transactions.push(Transaction::new(
            "0".to_string(),
            miner_address.to_string(),
            50
        ));

        let new_block = Block::new(
            previous_block.index + 1,
            Utc::now().timestamp(),
            transactions,
            previous_block.hash.clone(),
            0,
        );

        let mined_block = self.pow.mine_block(new_block);
        self.block_map.insert(mined_block.hash.clone(), mined_block.clone());
        self.chain.push(mined_block);
    }

    pub fn is_valid(&self) -> bool {
        if self.chain.is_empty() {
            return false;
        }

        // 检查创世块
        let genesis = &self.chain[0];
        if genesis.index != 0 || genesis.previous_hash != "0" || !genesis.transactions.is_empty() {
            return false;
        }

        // 检查后续区块
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let prev = &self.chain[i - 1];
            
            if current.hash != current.calculate_hash() ||
               current.previous_hash != prev.hash ||
               !self.pow.is_valid(current)
            {
                return false;
            }
        }
        true
    }

    pub fn chain(&self) -> &[Block] {
        &self.chain
    }

    pub fn chain_mut(&mut self) -> &mut Vec<Block> {
        &mut self.chain
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        self.pending_transactions.push(tx);
    }

    pub fn pending_transactions(&self) -> &[Transaction] {
        &self.pending_transactions
    }
}
