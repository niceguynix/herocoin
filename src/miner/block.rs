use super::Miner;
use crate::block::Block;
use crate::transaction::Transaction;
use serde_json;
use std::io::Read;
use std::net::TcpStream;

impl Miner {
    pub fn add_block(&mut self, data:&str) {
        let block: Block = serde_json::from_str(data).expect("Wrong transaction format");
        if !Self::verify_block() {
            return;
        }
        self.blocks.push(block);
    }

    pub fn create_block(&mut self, trans: Transaction) {
        let last_hash = match self.blocks.last() {
            Some(n) => n.hash,
            None => Self::genesis_block().hash,
        };
        let hash = self.compute_hash();
        self.accounts.insert(trans.reciever,trans.amount);
        let block = Block::new(last_hash, self.public_key, trans, hash);

        self.blocks.push(block);
    }

    pub fn add_trans(&mut self, data:&str) {
        let trans: Transaction = serde_json::from_str(&data).expect("Wrong transaction format");
        self.create_block(trans);
    }
}
