use std::net::SocketAddr;

#[derive(Clone)]
pub struct Packet {
    pub address: SocketAddr,
    pub guid: u64,
    pub length: usize,
    pub data: Vec<u8>,
}
