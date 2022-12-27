use std::net::IpAddr;

struct Node{
    private_key: [u8; 32],
    public_key: [u8; 32],

    peers: Vec<IpAddr>,
}

impl Node{
    fn new(private_key:[u8;32],public_key:[u8;32])->Self{
        Self{private_key,public_key,peers:vec!["127.0.0.1".parse().expect("parse error")]}
    }
}