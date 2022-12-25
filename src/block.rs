use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Block {
    prev_hash: [u8; 32],
    miner: [u8; 32],
    transaction: Transaction,
    pub hash: [u8; 32],
}

impl Block {
    pub fn new(
        prev_hash: [u8; 32],
        public_key: [u8; 32],
        transaction: Transaction,
        hash: [u8; 32],
    ) -> Self {
        Self {
            prev_hash,
            miner: public_key,
            transaction,
            hash,
        }
    }
}
