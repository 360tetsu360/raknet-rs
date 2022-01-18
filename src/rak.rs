use std::{fmt::Display, net::SocketAddr};

use crate::packet::RaknetPacket;

#[derive(Clone, Copy)]
pub enum DisconnectReason {
    Timeout,
    Disconnect,
}

#[derive(Clone)]
pub enum RaknetEvent {
    Packet(RaknetPacket),
    Connected(SocketAddr, u64),
    Disconnected(SocketAddr, u64, DisconnectReason),
    Error(SocketAddr, RaknetError),
}

#[derive(Debug, Clone)]
pub enum RaknetError {
    IncompatibleProtocolVersion(u8, u8), //Server,Client
    AlreadyConnected(SocketAddr),
    RemoteClosed(SocketAddr),
    Other(String),
}

impl Display for RaknetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncompatibleProtocolVersion(server, client) => {
                write!(f, "Different Protocol Version: {} {}", server, client)
            }
            Self::AlreadyConnected(s) => write!(f, "AlreadyConnected: {}", s),
            Self::RemoteClosed(s) => write!(f, "RemoteClosed : {}", s),
            Self::Other(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for RaknetError {}

pub trait RaknetHandler: std::marker::Send {
    fn on_connect(&mut self, addr: SocketAddr, guid: u64);
    fn on_disconnect(&mut self, addr: SocketAddr, guid: u64, reason: DisconnectReason);
    fn on_message(&mut self, packet: RaknetPacket);
    fn raknet_error(&mut self, addr: SocketAddr, e: RaknetError);
}
