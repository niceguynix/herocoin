use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Key(pub [u8;32]);

impl FromStr for Key{
    type Err = std::string::ParseError;

    fn from_str(string:&str)->Result<Self,Self::Err>{
        let key:Key;
        for (idx, i)  in string.chars().enumerate(){
            key.0[idx]=i.to_digit(10).expect("wrong input") as u8 ;
        }

        Ok(key)
    }
}