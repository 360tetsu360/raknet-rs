use std::{
    net::{SocketAddr, UdpSocket},
    rc::Rc,
};
pub struct Connection {
    pub address: SocketAddr,
    pub socket: Rc<UdpSocket>,
    //send_query : Vec<Frame>,
}

impl Connection {
    //pub fn new(addr : SocketAddr){}
    //pub fn handle_datagram(&self, buf: &mut [u8]) {}
    //pub fn send(&mut self,packet : Frame) {}
}
