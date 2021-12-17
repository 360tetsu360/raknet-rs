pub mod ack;
pub mod already_connected;
pub mod connected_ping;
pub mod connected_pong;
pub mod connection_request;
pub mod connection_request_accepted;
pub mod disconnected;
pub mod frame;
pub mod frame_set;
pub mod incompatible_protocol_version;
pub mod nack;
pub mod new_incoming_connection;
pub mod open_connection_reply1;
pub mod open_connection_reply2;
pub mod open_connection_request1;
pub mod open_connection_request2;
pub mod unconnected_ping;
pub mod unconnected_pong;
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
    pub fn new(byte: u8) -> Result<Self> {
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
    pub fn to_byte(&self) -> u8 {
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

    pub fn reliable(&self) -> bool {
        matches!(
            self,
            Reliability::Reliable | Reliability::ReliableOrdered | Reliability::ReliableSequenced
        )
    }

    pub fn sequenced_or_ordered(&self) -> bool {
        matches!(
            self,
            Reliability::UnreliableSequenced
                | Reliability::ReliableOrdered
                | Reliability::ReliableSequenced
        )
    }

    pub fn sequenced(&self) -> bool {
        matches!(
            self,
            Reliability::UnreliableSequenced | Reliability::ReliableSequenced
        )
    }
}

#[cfg(test)]
mod test;

use std::io::Result;

pub(crate) trait Packet: Clone {
    const ID: u8;
    fn read(buf: &[u8]) -> Result<Self>
    where
        Self: Sized;
    fn write(&self) -> Result<Vec<u8>>;
}

pub(crate) fn encode<T: Packet>(packet: T) -> Result<Vec<u8>> {
    Ok([&[T::ID], &*packet.write().unwrap()].concat())
}

pub(crate) fn decode<T: Packet>(buf: &[u8]) -> Result<T> {
    T::read(&buf[1..])
}
