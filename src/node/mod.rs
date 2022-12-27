use std::net::IpAddr;
use super::wallet::Key;

struct Node{
    private_key: Key,
    public_key: Key,

    peers: Vec<IpAddr>,
}

impl Node{
    fn new(private_key:Key,public_key:Key)->Self{
        Self{private_key,public_key,peers:vec!["127.0.0.1".parse().expect("parse error")]}
    }
}