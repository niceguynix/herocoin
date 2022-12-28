use super::Miner;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use serde_json;

impl Miner {
    pub fn listen(&mut self) {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_transaction(stream);
        }
    }

    pub fn send_block(&self, mut stream: TcpStream) {
        let msg = serde_json::to_string(&self.blocks).expect("cant be wrong");
        write!(stream, "{}",msg);
    }
}
