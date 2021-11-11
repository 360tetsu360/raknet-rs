use std::{
    net::SocketAddr
};
pub struct Connection {
    address : SocketAddr
}

impl Connection {
    pub fn handle_datagram(&self,buf : &mut [u8]) {

    }
}