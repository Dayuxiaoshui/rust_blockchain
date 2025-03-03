use std::collections::HashMap;
use crate::blockchain::Blockchain;

#[derive(Debug)]
pub struct Network {
    nodes: HashMap<String, Blockchain>,
}

impl Network {
    pub fn new() -> Self {
        Network { nodes: HashMap::new() }
    }

    pub fn add_node(&mut self, node_id: String, blockchain: Blockchain) {
        self.nodes.insert(node_id, blockchain);
    }

    pub fn broadcast(&mut self, sender_id: &str, new_blockchain: Blockchain) {
        for (node_id, blockchain) in self.nodes.iter_mut() {
            if node_id != sender_id {
                *blockchain = new_blockchain.clone();
            }
        }
    }
}

