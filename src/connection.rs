use crate::{packets::frame_set::FrameSet, raknet::RaknetEvent};
use std::{net::SocketAddr, sync::Arc, time::Instant};
use tokio::net::UdpSocket;

const DATAGRAM_FLAG: u8 = 0x80;
// bitFlagACK is set for every ACK packet.
const ACK_FLAG: u8 = 0x40;
// bitFlagNACK is set for every NACK packet.
const NACK_FLAG: u8 = 0x20;
pub struct Connection {
    pub address: SocketAddr,
    _socket: Arc<UdpSocket>,
    pub event_queue: Vec<RaknetEvent>,
    pub guid: u64,
    timer: Instant,
    last_recieve: u128,
    received_packet: Vec<u32>,
}

impl Connection {
    pub fn new(address: SocketAddr, socket: Arc<UdpSocket>, guid: u64, timer: Instant) -> Self {
        Self {
            address,
            _socket: socket,
            event_queue: vec![RaknetEvent::Connected(address, guid)],
            guid,
            timer,
            last_recieve: timer.elapsed().as_millis(),
            received_packet: vec![],
        }
    }
    pub fn update(&mut self) {
        self.event_queue.clear();
        self.ack_receipt();
        let time = self.timer.elapsed().as_millis();
        if (time - self.last_recieve) > 10000 {
            self.disconnect();
        }
    }
    pub fn handle(&mut self, buff: &[u8]) {
        let header = buff[0];

        self.last_recieve = self.timer.elapsed().as_millis();

        if header & ACK_FLAG != 0 {
            self.handle_ack(&buff[1..]);
        } else if header & NACK_FLAG != 0 {
            self.handle_nack(&buff[1..]);
        } else if header & DATAGRAM_FLAG != 0 {
            self.handle_datagram(&buff[1..]);
        }
    }
    fn ack_receipt(&mut self) {
        if !self.received_packet.is_empty() {
            self.send_ack(self.received_packet.clone().as_slice());
        }
    }
    fn send_ack(&mut self, _packets: &[u32]) {}
    fn handle_ack(&self, _buff: &[u8]) {}

    fn handle_nack(&self, _buff: &[u8]) {}

    fn handle_datagram(&mut self, buff: &[u8]) {
        let frame_set = FrameSet::decode(buff).expect("failed to read packet");
        self.received_packet.push(frame_set.sequence_number);
    }
    pub fn disconnect(&mut self) {
        self.event_queue
            .push(RaknetEvent::Disconnected(self.address, self.guid));
    }
}
