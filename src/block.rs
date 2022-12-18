use crate::transaction::Transaction;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct Block{
    prev_hash:[u8;32],
    transaction: Transaction,
    hash:[u8;32]
}

impl Block{
    fn new(prev_hash:[u8;32],transaction:Transaction,hash:[u8;32])->Self{
        Self { prev_hash, transaction, hash}
    }


}



