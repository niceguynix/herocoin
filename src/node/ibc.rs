use super::Node;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use serde_json;
use crate::transaction::Transaction;

impl Node {

    pub fn transmit_transaction(&self,trans:Transaction){
        let msg = serde_json::to_string(&trans).expect("somthing wrong i can feel it");

        for i in &self.peers{
            let mut stream = TcpStream::connect((i.clone(),7878)).expect("node not available");
            write!(stream, "{}",msg);
        }

    }

}
