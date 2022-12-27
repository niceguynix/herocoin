use crate::block::Block;
use crate::message::Messages;
use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{BufReader, Read, Write};
use std::net::{IpAddr, TcpListener, TcpStream};
use std::collections::HashMap;
use super::wallet::Key;

mod block;
mod crypto;
mod ibc;

pub struct Miner {
    private_key: Key,
    public_key: Key,
    accounts: HashMap<Key,u128>,
    blocks: Vec<Block>,
    peers: Vec<IpAddr>,
}

impl Miner {
    pub fn new(private_key: Key, public_key: Key) -> Self {
        let root_peer: IpAddr = match "127.0.0.1".parse() {
            Err(_) => panic!("Woah"),
            Ok(v) => v,
        };

        Self {
            private_key,
            public_key,
            accounts:HashMap::new(),
            blocks: vec![],
            peers: vec![root_peer],
        }
    }

    fn handle_transaction(&mut self, mut stream: TcpStream) {
        let mut data = String::from("");
        stream.read_to_string(&mut data);
        let trans: Messages = serde_json::from_str(&data).expect("Wrong transaction format");
        match trans {
            Messages::GetBlock() => self.send_block(stream),
            Messages::AddBlock() => self.add_block(stream),
            Messages::AddTransaction() => self.add_trans(stream),
        }
    }
}
