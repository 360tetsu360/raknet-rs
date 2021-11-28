use crate::{
    packet::ACKQueue,
    packets::{
        ack::ACK, connected_ping::ConnectedPing, connected_pong::ConnectedPong,
        connection_request::ConnectionRequest,
        connection_request_accepted::ConnectionRequestAccepted, decode, disconnected::Disconnected,
        encode, frame::Frame, frame_set::FrameSet, new_incoming_connection::NewIncomingConnection,
        Packet, Reliability,
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
    ack_queue: ACKQueue,
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
            ack_queue: ACKQueue::new(),
            send_queue: vec![],
            sequence_number: 0,
        }
    }
    pub fn update(&mut self) {
        self.event_queue.clear();
        self.flush_queue();
        self.flush_ack();
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
    fn flush_ack(&mut self) {
        let acks = self.ack_queue.get_send_able_and_clear();
        for ack in acks {
            self.send_ack(ack);
        }
    }
    fn send_ack(&mut self, packet: (u32, u32)) {
        let ack = ACK::new(packet);
        let buf = ack.encode().expect("failed to encode ACK");
        let socket = self.socket.clone();
        let address = self.address;
        tokio::spawn(async move {
            socket
                .send_to(&buf, address)
                .await
                .expect("failed to send ACK");
        });
    }
    fn handle_ack(&self, _buff: &[u8]) {}

    fn handle_nack(&self, _buff: &[u8]) {}

    fn handle_datagram(&mut self, buff: &[u8]) {
        let frame_set = FrameSet::decode(buff).expect("failed to read packet");
        self.ack_queue.add(frame_set.sequence_number);
        for frame in frame_set.datas {
            match frame.data[0] {
                ConnectionRequest::ID => {
                    let p = decode::<ConnectionRequest>(&frame.data).unwrap();
                    let reply = ConnectionRequestAccepted::new(
                        self.address,
                        p.time,
                        self.timer.elapsed().as_millis().try_into().unwrap(),
                    );
                    let buff = encode::<ConnectionRequestAccepted>(reply).unwrap();
                    let frame = Frame::new(Reliability::ReliableOrdered, &buff);
                    self.send(frame);
                }
                NewIncomingConnection::ID => {
                    let p = decode::<NewIncomingConnection>(&frame.data).unwrap();
                    println!("{}", p.server_address);
                }
                ConnectedPing::ID => {
                    let p = decode::<ConnectedPing>(&frame.data).unwrap();
                    let pong = ConnectedPong::new(
                        p.client_timestamp,
                        self.timer.elapsed().as_millis().try_into().unwrap(),
                    );
                    let buff = encode::<ConnectedPong>(pong).unwrap();
                    let mut frame = Frame::new(Reliability::ReliableOrdered, &buff);
                    frame.message_index = self.sequence_number;
                    frame.order_index = self.sequence_number;
                    self.send(frame);
                }
                Disconnected::ID => {
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
            self.send_queue.clear();
            self.sequence_number += 1;
            tokio::spawn(async move {
                socket
                    .send_to(&frame_set, address)
                    .await
                    .expect("failed to send packet");
            });
        }
    }
    pub fn disconnect(&mut self) {
        self.event_queue
            .push(RaknetEvent::Disconnected(self.address, self.guid));
    }
}
