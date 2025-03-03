#[derive(Debug, Clone)]
pub struct SmartContract;

impl SmartContract {
    pub fn execute(&self, transaction: &super::transaction::Transaction) -> bool {
        transaction.amount > 0
    }
}
