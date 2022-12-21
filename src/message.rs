use serde::{Serialize,Deserialize};
use crate::block::Block;


#[derive(Serialize,Deserialize)]
pub enum Messages{
    GetBlock(),
    AddBlock(),
    AddTransaction()
}
