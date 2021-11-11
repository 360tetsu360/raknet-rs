pub mod unconnected_ping;
pub mod unconnected_pong;
pub mod open_connection_request1;
pub mod open_connection_request2;
pub mod open_connection_reply1;
pub mod open_connection_reply2;
pub mod connected_ping;
pub mod connected_pong;
pub mod connection_request;
pub mod connection_request_accepted;
pub mod incompatible_protocol_version;
pub mod new_incoming_connection;

use unconnected_ping::UnconnectedPing as UPiPayload;
use unconnected_pong::UnconnectedPong as UPoPayload;
use open_connection_request1::OpenConnectionRequest1 as OCRequest1Payload;
use open_connection_request2::OpenConnectionRequest2 as OCRequest2Payload;
use open_connection_reply1::OpenConnectionReply1 as OCReply1Payload;
use open_connection_reply2::OpenConnectionReply2 as OCReply2Payload;
use connected_ping::ConnectedPing as CPiPayload;
use connected_pong::ConnectedPong as CPoPayload;
use connection_request::ConnectionRequest as CRPayload;
use connection_request_accepted::ConnectionRequestAccepted as CRAcceptedPayload;
use incompatible_protocol_version::IncompatibleProtocolVersion as IPVPayload;
use new_incoming_connection::NewIncomingConnection as NICPayload;

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
    Disconnect(())
}

impl From<&mut [u8]> for Packets {
    fn from(buf: &mut [u8]) -> Self {
        match buf[0] {
            0x0 => {
                Self::ConnectedPing(CPiPayload::read(&mut buf[1..]).unwrap())
            },
            0x01 | 0x02 => {
                Self::UnconnectedPing(UPiPayload::read(&mut buf[1..]).unwrap())
            },
            0x03 => {
                Self::ConnectedPong(CPoPayload::read(&mut buf[1..]).unwrap())
            },
            0x05 => {
                Self::OpenConnectionRequest1(OCRequest1Payload::read(&mut buf[1..]).unwrap())
            },
            0x06 => {
                Self::OpenConnectionReply1(OCReply1Payload::read(&mut buf[1..]).unwrap())
            },
            0x07 => {
                Self::OpenConnectionRequest2(OCRequest2Payload::read(&mut buf[1..]).unwrap())
            },
            0x08 => {
                Self::OpenConnectionReply2(OCReply2Payload::read(&mut buf[1..]).unwrap())
            },
            0x09 => {
                Self::ConnectionRequest(CRPayload::read(&mut buf[1..]).unwrap())
            },
            0x10 => {
                Self::ConnectionRequestAccepted(CRAcceptedPayload::read(&mut buf[1..]).unwrap())
            },
            0x13 => {
                Self::NewIncomingConnection(NICPayload::read(&mut buf[1..]).unwrap())
            },
            0x15 => {
                Self::Disconnect(())
            }
            0x19 => {
                Self::IncompatibleProtocolVersion(IPVPayload::read(&mut buf[1..]).unwrap())
            }
            0x1c => {
                Self::UnconnectedPong(UPoPayload::read(&mut buf[1..]).unwrap())
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

impl From<Packets> for Vec<u8> {
    fn from(packet: Packets) -> Vec<u8> {
        match packet {
            Packets::ConnectedPing(payload) => {
                let mut buf = payload.write().unwrap();
                buf.insert(0, 0x0);
                buf
            },
            Packets::UnconnectedPing(payload) => {
                let mut buf = payload.write().unwrap();
                buf.insert(0, 0x1);
                buf
            },
            Packets::ConnectedPong(payload) => {
                let mut buf = payload.write().unwrap();
                buf.insert(0, 0x3);
                buf
            },
            Packets::OpenConnectionRequest1(payload) => {
                let mut buf = payload.write().unwrap();
                buf.insert(0, 0x5);
                buf
            },
            Packets::OpenConnectionReply1(payload) => {
                let mut buf = payload.write().unwrap();
                buf.insert(0, 0x6);
                buf
            },
            Packets::OpenConnectionRequest2(payload) => {
                let mut buf = payload.write().unwrap();
                buf.insert(0, 0x7);
                buf
            },
            Packets::OpenConnectionReply2(payload) => {
                let mut buf = payload.write().unwrap();
                buf.insert(0, 0x8);
                buf
            },
            Packets::ConnectionRequest(payload) => {
                let mut buf = payload.write().unwrap();
                buf.insert(0, 0x9);
                buf
            },
            Packets::ConnectionRequestAccepted(payload) => {
                let mut buf = payload.write().unwrap();
                buf.insert(0, 0x10);
                buf
            },
            Packets::NewIncomingConnection(payload) => {
                let mut buf = payload.write().unwrap();
                buf.insert(0, 0x13);
                buf
            },
            Packets::Disconnect(_payload) => {
                vec![0x15,1]
            },
            Packets::IncompatibleProtocolVersion(payload) => {
                let mut buf = payload.write().unwrap();
                buf.insert(0, 0x19);
                buf
            },
            Packets::UnconnectedPong(payload) => {
                let mut buf = payload.write().unwrap();
                buf.insert(0, 0x1c);
                buf
            },
            //_ => {unimplemented!()}
        }
    }
}
