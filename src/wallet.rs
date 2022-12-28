use std::str::FromStr;
use serde::{Deserialize, Serialize};
use std::hash::{Hash};

#[derive(Serialize, Deserialize,Hash,Clone,Copy,PartialEq, Eq)]
pub struct Key(pub [u8;32]);

impl FromStr for Key{
    type Err = std::string::ParseError;

    fn from_str(string:&str)->Result<Self,Self::Err>{
        let mut key = Key([0;32]);
        for (idx, i)  in string.chars().enumerate(){
            key.0[idx]=i.to_digit(10).expect("wrong input") as u8 ;
        }

        Ok(key)
    }
}