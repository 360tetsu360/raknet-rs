pub mod ack;
pub mod connected_ping;
pub mod connected_pong;
pub mod connection_request;
pub mod connection_request_accepted;
pub mod frame;
pub mod frame_set;
pub mod incompatible_protocol_version;
pub mod new_incoming_connection;
pub mod open_connection_reply1;
pub mod open_connection_reply2;
pub mod open_connection_request1;
pub mod open_connection_request2;
pub mod unconnected_ping;
pub mod unconnected_pong;
use std::io::{Error, ErrorKind, Result};

use connected_ping::ConnectedPing as CPiPayload;
use connected_pong::ConnectedPong as CPoPayload;
use connection_request::ConnectionRequest as CRPayload;
use connection_request_accepted::ConnectionRequestAccepted as CRAcceptedPayload;
use incompatible_protocol_version::IncompatibleProtocolVersion as IPVPayload;
use new_incoming_connection::NewIncomingConnection as NICPayload;
use open_connection_reply1::OpenConnectionReply1 as OCReply1Payload;
use open_connection_reply2::OpenConnectionReply2 as OCReply2Payload;
use open_connection_request1::OpenConnectionRequest1 as OCRequest1Payload;
use open_connection_request2::OpenConnectionRequest2 as OCRequest2Payload;
use unconnected_ping::UnconnectedPing as UPiPayload;
use unconnected_pong::UnconnectedPong as UPoPayload;

pub enum Packets {
    UnconnectedPing(UPiPayload),
    UnconnectedPong(UPoPayload),
    OpenConnectionRequest1(OCRequest1Payload),
    OpenConnectionRequest2(OCRequest2Payload),
    OpenConnectionReply1(OCReply1Payload),
    OpenConnectionReply2(OCReply2Payload),
    ConnectedPing(CPiPayload),
    ConnectedPong(CPoPayload),
    ConnectionRequest(CRPayload),
    ConnectionRequestAccepted(CRAcceptedPayload),
    NewIncomingConnection(NICPayload),
    IncompatibleProtocolVersion(IPVPayload),
    Disconnect(()),
    Unknown(Vec<u8>),
    Error(()),
}

impl Packets {
    pub fn decode(buf: &[u8]) -> Result<Self> {
        match buf[0] {
            0x0 => Ok(Self::ConnectedPing(CPiPayload::read(&buf[1..])?)),
            0x01 | 0x02 => Ok(Self::UnconnectedPing(UPiPayload::read(&buf[1..])?)),
            0x03 => Ok(Self::ConnectedPong(CPoPayload::read(&buf[1..])?)),
            0x05 => Ok(Self::OpenConnectionRequest1(OCRequest1Payload::read(
                &buf[1..],
            )?)),
            0x06 => Ok(Self::OpenConnectionReply1(OCReply1Payload::read(
                &buf[1..],
            )?)),
            0x07 => Ok(Self::OpenConnectionRequest2(OCRequest2Payload::read(
                &buf[1..],
            )?)),
            0x08 => Ok(Self::OpenConnectionReply2(OCReply2Payload::read(
                &buf[1..],
            )?)),
            0x09 => Ok(Self::ConnectionRequest(CRPayload::read(&buf[1..])?)),
            0x10 => Ok(Self::ConnectionRequestAccepted(CRAcceptedPayload::read(
                &buf[1..],
            )?)),
            0x13 => Ok(Self::NewIncomingConnection(NICPayload::read(&buf[1..])?)),
            0x15 => Ok(Self::Disconnect(())),
            0x19 => Ok(Self::IncompatibleProtocolVersion(IPVPayload::read(
                &buf[1..],
            )?)),
            0x1c => Ok(Self::UnconnectedPong(UPoPayload::read(&buf[1..])?)),
            _ => Ok(Self::Unknown(buf.to_vec())),
        }
    }
    pub fn encode(self) -> Result<Vec<u8>> {
        match self {
            Packets::ConnectedPing(payload) => {
                let mut buf = payload.write()?;
                buf.insert(0, 0x0);
                Ok(buf)
            }
            Packets::UnconnectedPing(payload) => {
                let mut buf = payload.write()?;
                buf.insert(0, 0x1);
                Ok(buf)
            }
            Packets::ConnectedPong(payload) => {
                let mut buf = payload.write()?;
                buf.insert(0, 0x3);
                Ok(buf)
            }
            Packets::OpenConnectionRequest1(payload) => {
                let mut buf = payload.write()?;
                buf.insert(0, 0x5);
                Ok(buf)
            }
            Packets::OpenConnectionReply1(payload) => {
                let mut buf = payload.write()?;
                buf.insert(0, 0x6);
                Ok(buf)
            }
            Packets::OpenConnectionRequest2(payload) => {
                let mut buf = payload.write()?;
                buf.insert(0, 0x7);
                Ok(buf)
            }
            Packets::OpenConnectionReply2(payload) => {
                let mut buf = payload.write()?;
                buf.insert(0, 0x8);
                Ok(buf)
            }
            Packets::ConnectionRequest(payload) => {
                let mut buf = payload.write()?;
                buf.insert(0, 0x9);
                Ok(buf)
            }
            Packets::ConnectionRequestAccepted(payload) => {
                let mut buf = payload.write()?;
                buf.insert(0, 0x10);
                Ok(buf)
            }
            Packets::NewIncomingConnection(payload) => {
                let mut buf = payload.write()?;
                buf.insert(0, 0x13);
                Ok(buf)
            }
            Packets::Disconnect(_payload) => Ok(vec![0x15, 1]),
            Packets::IncompatibleProtocolVersion(payload) => {
                let mut buf = payload.write()?;
                buf.insert(0, 0x19);
                Ok(buf)
            }
            Packets::UnconnectedPong(payload) => {
                let mut buf = payload.write()?;
                buf.insert(0, 0x1c);
                Ok(buf)
            }
            Packets::Unknown(p) => Ok(p),
            Packets::Error(_p) => Ok(vec![]), //_ => {unimplemented!()}
        }
    }
}

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
