mod block;
mod message;
mod miner;
mod transaction;
mod node;

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
