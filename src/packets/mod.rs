pub(crate) mod ack;
pub(crate) mod already_connected;
pub(crate) mod connected_ping;
pub(crate) mod connected_pong;
pub(crate) mod connection_request;
pub(crate) mod connection_request_accepted;
pub(crate) mod disconnected;
pub(crate) mod frame;
pub(crate) mod frame_set;
pub(crate) mod incompatible_protocol_version;
pub(crate) mod nack;
pub(crate) mod new_incoming_connection;
pub(crate) mod open_connection_reply1;
pub(crate) mod open_connection_reply2;
pub(crate) mod open_connection_request1;
pub(crate) mod open_connection_request2;
pub(crate) mod unconnected_ping;
pub(crate) mod unconnected_pong;

pub use ack::*;
pub use already_connected::*;
pub use connected_ping::*;
pub use connected_pong::*;
pub use connection_request::*;
pub use connection_request_accepted::*;
pub use disconnected::*;
pub use frame::*;
pub use frame_set::*;
pub use incompatible_protocol_version::*;
pub use nack::*;
pub use new_incoming_connection::*;
pub use open_connection_reply1::*;
pub use open_connection_reply2::*;
pub use open_connection_request1::*;
pub use open_connection_request2::*;
pub use unconnected_ping::*;
pub use unconnected_pong::*;

use std::io::{Error, ErrorKind};
#[derive(Clone)]
pub enum Reliability {
    Unreliable,
    UnreliableSequenced,
    Reliable,
    ReliableOrdered,
    ReliableSequenced,
}

impl Reliability {
    pub(crate) fn new(byte: u8) -> Result<Self> {
        match byte {
            0x0 => Ok(Self::Unreliable),
            0x1 => Ok(Self::UnreliableSequenced),
            0x2 => Ok(Self::Reliable),
            0x3 => Ok(Self::ReliableOrdered),
            0x4 => Ok(Self::ReliableSequenced),
            _ => Err(Error::new(
                ErrorKind::Other,
                format!("unknown reliability byte {}", &byte),
            )),
        }
    }
    pub(crate) fn to_byte(&self) -> u8 {
        match self {
            Self::Unreliable => 0x0,
            Self::UnreliableSequenced => 0x1,
            Self::Reliable => 0x2,
            Self::ReliableOrdered => 0x3,
            Self::ReliableSequenced => 0x4,
        }
    }

    pub(crate) fn reliable(&self) -> bool {
        matches!(
            self,
            Reliability::Reliable | Reliability::ReliableOrdered | Reliability::ReliableSequenced
        )
    }

    pub(crate) fn sequenced_or_ordered(&self) -> bool {
        matches!(
            self,
            Reliability::UnreliableSequenced
                | Reliability::ReliableOrdered
                | Reliability::ReliableSequenced
        )
    }

    pub(crate) fn sequenced(&self) -> bool {
        matches!(
            self,
            Reliability::UnreliableSequenced | Reliability::ReliableSequenced
        )
    }
}

pub const MAGIC: [u8; 16] = [
    0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56, 0x78,
];

use async_trait::async_trait;
use std::io::Result;

#[async_trait]
pub trait Packet: Clone {
    const ID: u8;
    async fn read(buf: &[u8]) -> Result<Self>
    where
        Self: Sized;
    async fn write(&self) -> Result<Vec<u8>>;
}

pub async fn encode<T: Packet>(packet: T) -> Result<Vec<u8>> {
    Ok([&[T::ID], &*packet.write().await?].concat())
}

pub async fn decode<T: Packet>(buf: &[u8]) -> Result<T> {
    T::read(&buf[1..]).await
}

#[test]
fn reliability() {
    let reliables = vec![
        (Reliability::new(0x0).unwrap(), 0x0, false, false, false),
        (Reliability::new(0x1).unwrap(), 0x1, false, true, true),
        (Reliability::new(0x2).unwrap(), 0x2, true, false, false),
        (Reliability::new(0x3).unwrap(), 0x3, true, true, false),
        (Reliability::new(0x4).unwrap(), 0x4, true, true, true),
    ];
    for reliable in reliables {
        assert_eq!(reliable.0.to_byte(), reliable.1);
        assert_eq!(reliable.0.reliable(), reliable.2);
        assert_eq!(reliable.0.sequenced_or_ordered(), reliable.3);
        assert_eq!(reliable.0.sequenced(), reliable.4);
    }
}
