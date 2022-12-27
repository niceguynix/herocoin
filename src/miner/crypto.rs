use super::Miner;
use crate::wallet::Key;

impl Miner {
    pub fn compute_hash(&self) -> Key {
        Key([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
            ])
    }

    pub fn verify_block() -> bool {
        true
    }
}
