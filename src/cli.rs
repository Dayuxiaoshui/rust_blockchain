use std::io::{self, Write};
use crate::blockchain::Blockchain;
use crate::wallet::Wallet;
use crate::transaction::Transaction;
use crate::storage::{save_blockchain, load_blockchain};

pub struct CLI {
    wallets: Vec<Wallet>,
    blockchain: Blockchain,
}

impl CLI {
    pub fn new() -> Self {
        CLI {
            wallets: Vec::new(),
            blockchain: load_blockchain("blockchain.json", 4),
        }
    }

    pub fn run(&mut self) {
        loop {
            println!("\n1. 创建钱包");
            println!("2. 查看区块链");
            println!("3. 添加交易");
            println!("4. 挖矿");
            println!("5. 验证区块链");
            println!("6. 退出");
            print!("请选择操作: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            match input.trim() {
                "1" => self.create_wallet(),
                "2" => self.view_blockchain(),
                "3" => self.add_transaction(),
                "4" => self.mine_block(),
                "5" => self.validate_blockchain(),
                "6" => break,
                _ => println!("无效输入"),
            }
        }
        save_blockchain(&self.blockchain, "blockchain.json");
    }

    fn create_wallet(&mut self) {
        let wallet = Wallet::new();
        println!("新钱包创建成功，地址: {}", wallet.public_key());
        self.wallets.push(wallet);
    }

    fn view_blockchain(&self) {
        for block in self.blockchain.chain() {
            println!("{:#?}", block);
        }
    }

    fn add_transaction(&mut self) {
        print!("发送者地址: ");
        io::stdout().flush().unwrap();
        let mut sender = String::new();
        io::stdin().read_line(&mut sender).unwrap();
        sender = sender.trim().to_string();
        print!("接收者地址: ");
        io::stdout().flush().unwrap();
        let mut receiver = String::new();
        io::stdin().read_line(&mut receiver).unwrap();
        receiver = receiver.trim().to_string();
        print!("金额: ");
        io::stdout().flush().unwrap();
        let mut amount = String::new();
        io::stdin().read_line(&mut amount).unwrap();
        let amount: u64 = amount.trim().parse().unwrap();
        if let Some(wallet) = self.wallets.iter().find(|w| w.public_key() == sender) {
            let mut tx = Transaction::new(sender, receiver, amount);
            tx.sign(wallet.key_pair());
            self.blockchain.add_transaction(tx);
            println!("交易已添加");
        } else {
            println!("未找到发送者钱包");
        }
    }

    fn mine_block(&mut self) {
        print!("矿工地址: ");
        io::stdout().flush().unwrap();
        let mut addr = String::new();
        io::stdin().read_line(&mut addr).unwrap();
        self.blockchain.add_block(addr.trim());
        println!("新区块已挖出");
    }

    fn validate_blockchain(&self) {
        println!("区块链有效性: {}", self.blockchain.is_valid());
    }
}
