mod block;
mod message;
mod miner;
mod transaction;
mod node;
mod wallet;

use std::io;
use miner::Miner;
use wallet::Key;

fn main() {
    let option = std::env::args().nth(1).expect("Specify miner or node option");
    match option.to_lowercase().as_ref(){
        "miner" =>miner(),
        "node"=>node(),
        _=>panic!("Unknown option"),
    }
}

fn miner(){
}

fn node(){
}


fn get_keys()->(Key,Key){
    print!("Enter public key:");
    let mut s = String::with_capacity(2);
    io::stdin().read_line(&mut s).expect("User input Failed");
    let pbk = s.parse().expect("Wrong type");
    print!("Enter private key:");
    io::stdin().read_line(&mut s).expect("User input Failed");
    let prk = s.parse().expect("Wrong type");
    (pbk,prk)
}
