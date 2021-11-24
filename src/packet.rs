use std::net::SocketAddr;
pub struct Frame {
    //
    //
    //
    pub data: Vec<u8>,
}

pub struct FrameSet {
    pub sequence_number: u32,
}

#[derive(Clone)]
pub struct Packet {
    pub address: SocketAddr,
    pub guid: u64,
    pub length: usize,
    pub data: Vec<u8>,
}
