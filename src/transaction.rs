use serde::{Deserialize, Serialize};
use super::wallet::Key;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize,Hash)]
pub struct Transaction {
    sender: Key,
    reciever: Key,
    amount: u128,
    sign: String,
}

impl Transaction{
    pub fn new(sender:Key,reciever:Key,amount:u128)->Self{
//        let sign = calc_hash();
        let sign = "00000".to_owned();

        Self{
            sender,reciever,amount,sign
        }
    }
}


//impl Transaction{
//    fn calc_hash(Self::){
//        let mut s = DefaultHasher::new();
//        t.hash(&mut s);
//        s.finish()
//    }
//}
