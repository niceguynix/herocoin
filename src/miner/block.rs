use super::Miner;
use crate::block::Block;
use crate::transaction::Transaction;
use serde_json;
use std::io::Read;
use std::net::TcpStream;

impl Miner {
    pub fn add_block(&mut self, mut stream: TcpStream) {
        let mut data = String::from("");
        stream.read_to_string(&mut data);
        let block: Block = serde_json::from_str(&data).expect("Wrong transaction format");
        if !Self::verify_block() {
            return;
        }
        self.blocks.push(block);
    }

    pub fn create_block(&self, trans: Transaction) {
        let last_block = match self.blocks.last() {
            Some(n) => n,
            None => panic!("Error no genesis block created"),
        };
        let hash = self.compute_hash();
        let block = Block::new(last_block.hash, self.public_key, trans, hash);
    }

    pub fn add_trans(&self, mut stream: TcpStream) {
        let mut data = String::from("");
        stream.read_to_string(&mut data);
        let trans: Transaction = serde_json::from_str(&data).expect("Wrong transaction format");
        self.create_block(trans);
    }
}
