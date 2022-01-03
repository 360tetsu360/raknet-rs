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
    UnreliableACKReceipt,
    ReliableACKReceipt,
    ReliableOrderedACKReceipt,
}

impl Reliability {
    pub(crate) fn new(byte: u8) -> Result<Self> {
        match byte {
            0x0 => Ok(Self::Unreliable),
            0x1 => Ok(Self::UnreliableSequenced),
            0x2 => Ok(Self::Reliable),
            0x3 => Ok(Self::ReliableOrdered),
            0x4 => Ok(Self::ReliableSequenced),
            0x5 => Ok(Self::UnreliableACKReceipt),
            0x6 => Ok(Self::ReliableACKReceipt),
            0x7 => Ok(Self::ReliableOrderedACKReceipt),
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
            Self::UnreliableACKReceipt => 0x5,
            Self::ReliableACKReceipt => 0x6,
            Self::ReliableOrderedACKReceipt => 0x7,
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
