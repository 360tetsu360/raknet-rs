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

use std::io::{ErrorKind, Result};
pub trait Packet {
    const ID: u8;
    
}