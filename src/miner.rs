use std::net::{IpAddr , TcpListener , TcpStream};
use std::io::{BufReader , Read};
use crate::transaction::Transaction;
use serde::{Serialize, Deserialize};
use serde_json;


struct Miner{
    private_key:[u8;32],
    public_key:[u8;32],

    peers:Vec<IpAddr>,

}

impl Miner{

    fn new(private_key:[u8;32],public_key:[u8;32])->Self{
        let root_peer:IpAddr = match "127.0.0.1".parse(){
            Err(_)=>panic!("Woah"),
            Ok(v) =>v,
        };

        Self{private_key,public_key,peers:vec![root_peer]}
    }

    fn listen(&self){
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_transaction(stream);
        }
    }

    fn handle_transaction(&self,mut stream:TcpStream){
        let mut data =String::from("");
        stream.read_to_string(&mut data);
        let trans:Transaction = serde_json::from_str(&data).expect("Wrong transaction format");


    }
}