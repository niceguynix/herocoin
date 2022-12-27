use serde::{Deserialize, Serialize};
use super::wallet::Key;

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    sender: Key,
    reciever: Key,
    amount: u128,
}
