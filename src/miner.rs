use std::net::{IpAddr , TcpListener , TcpStream};
use std::io::{BufReader , Read ,Write};
use crate::transaction::Transaction;
use serde::{Serialize, Deserialize};
use serde_json;
use crate::message::Messages;
use crate::block::Block;


struct Miner{
    private_key:[u8;32],
    public_key:[u8;32],

    blocks:Vec<Block>,
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
        let trans:Messages = serde_json::from_str(&data).expect("Wrong transaction format");
        match trans{
            Messages::GetBlock()=>self.send_block(stream),
            Messages::AddBlock()=>self.add_block(stream),
            Messages::AddTransaction()=>self.add_trans(stream),
        }


    }


    fn send_block(&self,stream:TcpStream){
        write!(stream,"lmao");
    }

    fn add_block(&self,stream:TcpStream){
        let mut data = String::from("");
        stream.read_to_string(&mut data);
        let block:Block = serde_json::from_str(&data).expect("Wrong transaction format");
        if !Self::verify_block(){
            return
        }
        self.blocks.push(block);
    }

    fn add_trans(&self,stream:TcpStream){
        let mut data = String::from("");
        stream.read_to_string(&mut data);
        let trans:Transaction = serde_json::from_str(&data).expect("Wrong transaction format");
        self.create_block(trans);
    }

    fn create_block(&self,trans:Transaction){

        let last_block=  match self.blocks.last(){
            Some(n)=>n,
            None =>panic!("Error no genesis block created"),
        };
        let hash = self.compute_hash();
        let block = Block::new(last_block.hash,self.public_key,trans, hash);    
    }

    fn compute_hash(&self)->[u8;32]{
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
    }

    fn verify_block()->bool{
        true
    }
}