use serde::{Deserialize, Serialize};
use super::wallet::Key;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize,Hash)]
pub struct Transaction {
    sender: Key,
    reciever: Key,
    amount: u128,
}

//impl Transaction{
//    fn calc_hash(Self::){
//        let mut s = DefaultHasher::new();
//        t.hash(&mut s);
//        s.finish()
//    }
//}
