use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    sender: [u8; 32],
    reciever: [u8; 32],
    amount: u128,
}
