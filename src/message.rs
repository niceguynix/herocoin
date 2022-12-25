use crate::block::Block;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{IpAddr, TcpStream};

#[derive(Serialize, Deserialize)]
pub enum Messages {
    GetBlock(),
    AddBlock(),
    AddTransaction(),
}

fn getBlocks(peers: Vec<IpAddr>) -> Vec<Block> {
    let mut blocks = Vec::new();
    for i in peers {
        let mut stream = TcpStream::connect((i, 8080)).expect("failed");
        let msg = match serde_json::to_string(&Messages::GetBlock()) {
            Ok(v) => v,
            Err(_) => panic!("error"),
        };
        write!(stream, "{}", msg);
        let mut data = String::from("");
        stream.read_to_string(&mut data);
        let block: Vec<Block> = serde_json::from_str(&data).expect("Wrong transaction format");
        blocks.extend(block);
    }
    blocks
}
