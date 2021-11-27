use crate::{
    packets::{
        connection_request_accepted::ConnectionRequestAccepted, frame::Frame, frame_set::FrameSet,
        Packets, Reliability,
    },
    raknet::RaknetEvent,
};
use std::{convert::TryInto, net::SocketAddr, sync::Arc, time::Instant};
use tokio::net::UdpSocket;

const DATAGRAM_FLAG: u8 = 0x80;

const ACK_FLAG: u8 = 0x40;

const NACK_FLAG: u8 = 0x20;

pub struct Connection {
    pub address: SocketAddr,
    socket: Arc<UdpSocket>,
    pub event_queue: Vec<RaknetEvent>,
    pub guid: u64,
    timer: Instant,
    last_recieve: u128,
    received_packet: Vec<u32>,
    send_queue: Vec<Frame>,
    sequence_number: u32,
}

impl Connection {
    pub fn new(address: SocketAddr, socket: Arc<UdpSocket>, guid: u64, timer: Instant) -> Self {
        Self {
            address,
            socket,
            event_queue: vec![RaknetEvent::Connected(address, guid)],
            guid,
            timer,
            last_recieve: timer.elapsed().as_millis(),
            received_packet: vec![],
            send_queue: vec![],
            sequence_number: 0,
        }
    }
    pub fn update(&mut self) {
        self.event_queue.clear();
        self.ack_receipt();
        self.flush_queue();
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
        for frame in frame_set.datas {
            let packet = Packets::decode(&frame.data).expect("failed to read packet");
            match packet {
                Packets::ConnectionRequest(p) => {
                    let reply = ConnectionRequestAccepted::new(
                        self.address,
                        p.time,
                        self.timer.elapsed().as_millis().try_into().unwrap(),
                    );
                    let buff = Packets::ConnectionRequestAccepted(reply).encode().unwrap();
                    let frame = Frame::new(Reliability::ReliableOrdered, &buff);
                    self.send(frame);
                }
                Packets::NewIncomingConnection(p) => {
                    println!("{}", p.server_address);
                }
                Packets::Disconnect(_) => {
                    self.disconnect();
                }
                _ => {}
            }
        }
    }

    fn send(&mut self, packet: Frame) {
        self.send_queue.push(packet);
    }
    fn flush_queue(&mut self) {
        if !self.send_queue.is_empty() {
            let mut frame_set = FrameSet::new(self.sequence_number, &self.send_queue)
                .encode()
                .expect("error while encoding packet");
            frame_set.insert(0, 0x80);
            let socket = self.socket.clone();
            let address = self.address;
            tokio::spawn(async move {
                socket
                    .send_to(&frame_set, address)
                    .await
                    .expect("failed to send packet");
            });
            self.send_queue.clear();
            self.sequence_number += 1;
        }
    }
    pub fn disconnect(&mut self) {
        self.event_queue
            .push(RaknetEvent::Disconnected(self.address, self.guid));
    }
}
