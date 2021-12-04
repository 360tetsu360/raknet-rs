use crate::{
    packet::{ACKQueue, RaknetPacket},
    packetqueue::PacketQueue,
    packets::{
        ack::Ack, connected_ping::ConnectedPing, connected_pong::ConnectedPong,
        connection_request::ConnectionRequest,
        connection_request_accepted::ConnectionRequestAccepted, decode, disconnected::Disconnected,
        encode, frame::Frame, frame_set::FrameSet, nack::Nack,
        new_incoming_connection::NewIncomingConnection, Packet, Reliability,
    },
    raknet::RaknetEvent,
    recievedqueue::RecievdQueue,
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
    pub mtu: u16,
    pub timer: Instant,
    pub last_recieve: u128,
    ack_queue: ACKQueue,
    packet_queue: PacketQueue,
    message_index: u32,
    order_index: u32,
    recieved: RecievdQueue,
    last_ping: u128,
}

impl Connection {
    pub fn new(
        address: SocketAddr,
        socket: Arc<UdpSocket>,
        guid: u64,
        timer: Instant,
        mtu: u16,
    ) -> Self {
        Self {
            address,
            socket,
            event_queue: vec![],
            guid,
            mtu,
            timer,
            last_recieve: timer.elapsed().as_millis(),
            ack_queue: ACKQueue::new(),
            packet_queue: PacketQueue::new(mtu),
            message_index: 0,
            order_index: 0,
            recieved: RecievdQueue::new(),
            last_ping: timer.elapsed().as_millis(),
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
        if time - self.last_ping > 5 * 1000 {
            // 1/s
            self.last_ping = time;
            self.send_ping();
        }
    }
    pub fn connect(&mut self) {
        let request =
            ConnectionRequest::new(self.guid, self.timer.elapsed().as_millis() as i64, false);
        let buff = encode::<ConnectionRequest>(request).unwrap();
        let frame = Frame::new(Reliability::Reliable, &buff);
        self.send(frame);
    }
    pub fn handle(&mut self, buff: &[u8]) {
        let header = buff[0];

        self.last_recieve = self.timer.elapsed().as_millis();

        if header & ACK_FLAG != 0 {
            self.handle_ack(buff);
        } else if header & NACK_FLAG != 0 {
            self.handle_nack(buff);
        } else if header & DATAGRAM_FLAG != 0 {
            self.handle_datagram(buff);
        }
    }
    fn flush_ack(&mut self) {
        let acks = self.ack_queue.get_send_able_and_clear();
        for ack in acks {
            self.send_ack(ack);
        }
    }
    fn send_ping(&mut self) {
        let connected_ping = ConnectedPing::new(self.last_ping as i64);
        let frame = Frame::new(
            Reliability::Unreliable,
            &encode::<ConnectedPing>(connected_ping).unwrap(),
        );
        self.send(frame);
    }
    fn send_ack(&mut self, packet: (u32, u32)) {
        let ack = Ack::new(packet);
        let buf = encode::<Ack>(ack).expect("failed to encode ACK");
        let socket = self.socket.clone();
        let address = self.address;
        tokio::spawn(async move {
            socket
                .send_to(&buf, address)
                .await
                .expect("failed to send ACK");
        });
    }
    fn handle_ack(&mut self, buff: &[u8]) {
        let ack = decode::<Ack>(buff).unwrap();
        for sequence in ack.get_all() {
            self.packet_queue.recieved(sequence);
        }
    }

    fn handle_nack(&mut self, buff: &[u8]) {
        let nack = decode::<Nack>(buff).unwrap();
        for sequence in nack.get_all() {
            self.packet_queue.resend(sequence);
        }
    }

    fn handle_datagram(&mut self, buff: &[u8]) {
        let frame_set = FrameSet::decode(buff).expect("failed to read packet");
        self.ack_queue.add(frame_set.sequence_number);
        if self.ack_queue.get_missing_len() != 0 {
            
        }
        for frame in frame_set.datas {
            self.recieve_packet(frame);
        }
    }
    fn recieve_packet(&mut self, frame: Frame) {
        if !frame.reliability.sequenced_or_ordered() {
            self.handle_packet(&frame.data);
        } else {
            self.recieved.add(frame);
            for packet in self.recieved.get_all() {
                self.handle_packet(&packet.data);
            }
        }
    }

    fn handle_packet(&mut self, payload: &[u8]) {
        match payload[0] {
            ConnectionRequest::ID => {
                self.handle_connectionrequest(payload);
            }
            ConnectionRequestAccepted::ID => {
                self.handle_connectionrequest_accepted(payload);
            }
            NewIncomingConnection::ID => {
                //let p = decode::<NewIncomingConnection>(payload).unwrap();
            }
            ConnectedPing::ID => {
                self.handle_connectedping(payload);
            }
            ConnectedPong::ID => {}
            Disconnected::ID => {
                self.disconnect();
            }
            _ => {
                let rak_packet = RaknetPacket::new(self.address, self.guid, payload.to_vec());
                self.event_queue.push(RaknetEvent::Packet(rak_packet));
            }
        }
    }

    pub fn send_to(&mut self, buff: &[u8]) {
        if buff.len() < (self.mtu - 42).into() {
            let mut frame = Frame::new(Reliability::ReliableOrdered, buff);
            frame.message_index = self.message_index;
            frame.order_index = self.order_index;
            self.message_index += 1;
            self.order_index += 1;
            self.send(frame);
        } else {
            let max = self.mtu - 52;
            let mut split_len = buff.len() as u16 / max;
            if buff.len() as u16 % max != 0 {
                split_len += 1;
            }
            for i in 0..split_len {
                let range = (i * max) as usize..((i + 1) * max) as usize;
                let mut frame = Frame::new(Reliability::ReliableOrdered, &buff[range]);
                frame.split = true;
                frame.message_index = self.message_index;
                frame.order_index = self.order_index;
                self.send(frame);
                self.message_index += 1;
            }
            self.order_index += 1;
        }
    }
    fn send(&mut self, packet: Frame) {
        self.packet_queue.add_frame(packet);
    }
    fn flush_queue(&mut self) {
        for send_able in self.packet_queue.get_packet() {
            let frame_set = send_able.encode().expect("failed to encode frame_set");
            let socket = self.socket.clone();
            let address = self.address;
            tokio::spawn(async move {
                socket
                    .send_to(&frame_set, address)
                    .await
                    .expect("failed to send packet");
            });
        }
    }
    fn handle_connectionrequest(&mut self, payload: &[u8]) {
        let p = decode::<ConnectionRequest>(payload).unwrap();
        let reply = ConnectionRequestAccepted::new(
            self.address,
            p.time,
            self.timer.elapsed().as_millis().try_into().unwrap(),
        );
        let buff = encode::<ConnectionRequestAccepted>(reply).unwrap();
        let frame = Frame::new(Reliability::ReliableOrdered, &buff);
        self.send(frame);
        self.event_queue
            .push(RaknetEvent::Connected(self.address, self.guid))
    }
    fn handle_connectionrequest_accepted(&mut self, payload: &[u8]) {
        let accepted = decode::<ConnectionRequestAccepted>(payload).unwrap();
        let newincoming = NewIncomingConnection {
            server_address: self.address,
            request_timestamp: accepted.request_timestamp,
            accepted_timestamp: accepted.accepted_timestamp,
        };
        let buff = encode::<NewIncomingConnection>(newincoming).unwrap();
        let frame = Frame::new(Reliability::ReliableOrdered, &buff);
        self.send(frame);
    }
    fn handle_connectedping(&mut self, payload: &[u8]) {
        let p = decode::<ConnectedPing>(payload).unwrap();
        let pong = ConnectedPong::new(
            p.client_timestamp,
            self.timer.elapsed().as_millis().try_into().unwrap(),
        );
        let buff = encode::<ConnectedPong>(pong).unwrap();
        let mut frame = Frame::new(Reliability::ReliableOrdered, &buff);
        frame.message_index = self.message_index;
        frame.order_index = self.order_index;
        self.message_index += 1;
        self.order_index += 1;
        self.send(frame);
    }

    pub fn disconnect(&mut self) {
        self.event_queue
            .push(RaknetEvent::Disconnected(self.address, self.guid));
    }
}
