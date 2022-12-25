use super::Miner;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

impl Miner {
    fn listen(&mut self) {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_transaction(stream);
        }
    }

    pub fn send_block(&self, mut stream: TcpStream) {
        write!(stream, "lmao");
    }
}
