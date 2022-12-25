use super::Miner;

impl Miner {
    pub fn compute_hash(&self) -> [u8; 32] {
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]
    }

    pub fn verify_block() -> bool {
        true
    }
}
