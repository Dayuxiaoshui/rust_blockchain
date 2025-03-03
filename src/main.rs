mod block;
mod blockchain;
mod pow;
mod transaction;
mod wallet;
mod merkle_tree;
mod smart_contract;
mod network;
mod cli;
mod storage;

use cli::CLI;

fn main() {
    println!("欢迎使用区块链系统!");
    let mut cli = CLI::new();
    cli.run();
}
