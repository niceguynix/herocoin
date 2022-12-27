use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use super::wallet::Key;

#[derive(Serialize, Deserialize)]
pub struct Block {
    prev_hash: Key,
    miner: Key,
    transaction: Transaction,
    pub hash: Key,
}

impl Block {
    pub fn new(
            prev_hash:Key,
            public_key: Key,
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
