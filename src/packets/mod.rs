pub mod connected_ping;
pub mod connected_pong;
pub mod connection_request;
pub mod connection_request_accepted;
pub mod incompatible_protocol_version;
pub mod new_incoming_connection;
pub mod open_connection_reply1;
pub mod open_connection_reply2;
pub mod open_connection_request1;
pub mod open_connection_request2;
pub mod unconnected_ping;
pub mod unconnected_pong;

use std::io::Result;

pub(crate) trait Packet: Clone {
    const ID: u8;
    fn read(buf: &[u8]) -> Result<Self> where Self: Sized;
    fn write(&self) -> Result<Vec<u8>>;
}

pub(crate) fn encode<T: Packet>(packet: T) -> Result<Vec<u8>> {
    Ok([&[T::ID], &*packet.write().unwrap()].concat())
}

pub(crate) fn decode<T: Packet>(buf: &mut [u8]) -> Result<T>{
    T::read(&buf[1..])
}