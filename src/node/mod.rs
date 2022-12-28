use std::net::IpAddr;
use super::wallet::Key;
use std::str::FromStr;
use super::transaction::Transaction;
use std::clone;
use std::io::Write;

mod ibc;

pub struct Node{
    private_key: Key,
    public_key: Key,

    peers: Vec<IpAddr>,
}

impl Node{
    pub fn new(private_key:Key,public_key:Key)->Self{
        Self{private_key,public_key,peers:vec!["172.17.0.1".parse().expect("parse error")]}
    }

    pub fn run(&self){
        let mut ch="y".to_owned();
        while ch=="y"{
            print!("Enter recieving wallet no: ");
            let reciever = get_user_input();
            print!("Enter amount: ");
            let amount = get_user_input();
            let trans = Transaction::new(self.public_key ,reciever,amount);

            self.transmit_transaction(trans);

            print!("Another Transaction(y/n): ");
            ch = get_user_input();
        }

    }


}



fn get_user_input<T: FromStr>() -> T {
    std::io::stdout().flush().unwrap();
    let mut s = String::with_capacity(2);
    std::io::stdin().read_line(&mut s).expect("User input Failed");
    match s.trim().parse() {
        Ok(n) => n,
        Err(err) => panic!("Wrong input!"),
    }
}
