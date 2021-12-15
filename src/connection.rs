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
    dissconnected: bool,
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
            dissconnected: false,
        }
    }
    pub async fn update(&mut self) {
        self.event_queue.clear();
        self.flush_queue().await;
        self.flush_ack().await;
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
        let buff = match encode::<ConnectionRequest>(request) {
            Ok(buff) => buff,
            Err(e) => {
                eprintln!("failed to decode connectionrequest {}", e);
                return;
            }
        };
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
    async fn flush_ack(&mut self) {
        let acks = self.ack_queue.get_send_able_and_clear();
        for ack in acks {
            self.send_ack(ack).await;
        }
    }
    fn send_ping(&mut self) {
        let connected_ping = ConnectedPing::new(self.last_ping as i64);
        let buff = match encode::<ConnectedPing>(connected_ping) {
            Ok(buff) => buff,
            Err(e) => {
                eprintln!("failed to decode connectedping {}", e);
                return;
            }
        };
        let frame = Frame::new(Reliability::Unreliable, &buff);
        self.send(frame);
    }
    async fn send_ack(&mut self, packet: (u32, u32)) {
        if self.dissconnected {
            return;
        }
        let ack = Ack::new(packet);
        let buff = match encode::<Ack>(ack) {
            Ok(buff) => buff,
            Err(e) => {
                eprintln!("failed to encode ack {}", e);
                return;
            }
        };
        match self.socket.send_to(&buff, self.address).await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
                self.disconnected();
            }
        }
    }
    fn handle_ack(&mut self, buff: &[u8]) {
        let ack = match decode::<Ack>(buff) {
            Ok(ack) => ack,
            Err(e) => {
                eprintln!("failed to decode ack {}", e);
                return;
            }
        };
        for sequence in ack.get_all() {
            self.packet_queue.recieved(sequence);
        }
    }

    fn handle_nack(&mut self, buff: &[u8]) {
        let nack = match decode::<Nack>(buff) {
            Ok(nack) => nack,
            Err(e) => {
                eprintln!("failed to decode nack {}", e);
                return;
            }
        };
        for sequence in nack.get_all() {
            self.packet_queue.resend(sequence);
        }
    }

    fn handle_datagram(&mut self, buff: &[u8]) {
        let frame_set = match FrameSet::decode(buff) {
            Ok(frameset) => frameset,
            Err(e) => {
                eprintln!("failed to decode frameset {}", e);
                return;
            }
        };
        self.ack_queue.add(frame_set.sequence_number);
        if self.ack_queue.get_missing_len() != 0 {
            todo!("send nack")
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
            NewIncomingConnection::ID => {}
            ConnectedPing::ID => {
                self.handle_connectedping(payload);
            }
            ConnectedPong::ID => {}
            Disconnected::ID => {
                self.disconnect();
                self.disconnected();
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
            self.message_index += 1;
            self.order_index += 1;
            frame.message_index = self.message_index;
            frame.order_index = self.order_index;
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
    async fn flush_queue(&mut self) {
        let mut error = false;
        for send_able in self.packet_queue.get_packet().clone() {
            let frame_set = match send_able.encode() {
                Ok(buff) => buff,
                Err(e) => {
                    eprintln!("failed to encode frameset {}", e);
                    return;
                }
            };
            match self.socket.send_to(&frame_set, self.address).await {
                Ok(_) => {}
                Err(e) => {
                    if !self.dissconnected {
                        eprintln!("{}", e);
                        error = true;
                    }
                }
            }
        }
        if error {
            if !self.dissconnected {
                self.disconnected();
            }
        }
    }
    fn handle_connectionrequest(&mut self, payload: &[u8]) {
        let p = match decode::<ConnectionRequest>(payload) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("failed to decode connectionrequest {}", e);
                return;
            }
        };

        let reply = ConnectionRequestAccepted::new(self.address, p.time, self.time_stamp());
        let buff = match encode::<ConnectionRequestAccepted>(reply) {
            Ok(buff) => buff,
            Err(e) => {
                eprintln!("failed to encode connectionrequestaccepted {}", e);
                return;
            }
        };
        let frame = Frame::new(Reliability::ReliableOrdered, &buff);
        self.send(frame);
        self.event_queue
            .push(RaknetEvent::Connected(self.address, self.guid))
    }
    fn handle_connectionrequest_accepted(&mut self, payload: &[u8]) {
        let accepted = match decode::<ConnectionRequestAccepted>(payload) {
            Ok(accepted) => accepted,
            Err(e) => {
                eprintln!("failed to decode connectionrequestaccepted {}", e);
                return;
            }
        };

        let newincoming = NewIncomingConnection {
            server_address: self.address,
            request_timestamp: accepted.request_timestamp,
            accepted_timestamp: accepted.accepted_timestamp,
        };
        let buff = match encode::<NewIncomingConnection>(newincoming) {
            Ok(buff) => buff,
            Err(e) => {
                eprintln!("failed to encode newincomingconnection {}", e);
                return;
            }
        };
        let frame = Frame::new(Reliability::ReliableOrdered, &buff);
        self.send(frame);
        self.event_queue
            .push(RaknetEvent::Connected(self.address, self.guid));
    }
    fn handle_connectedping(&mut self, payload: &[u8]) {
        let p = match decode::<ConnectedPing>(payload) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("failed to decode connectedping {}", e);
                return;
            }
        };

        let pong = ConnectedPong::new(p.client_timestamp, self.time_stamp());
        let buff = match encode::<ConnectedPong>(pong) {
            Ok(buff) => buff,
            Err(e) => {
                eprintln!("failed to encode connectedpong {}", e);
                return;
            }
        };

        let mut frame = Frame::new(Reliability::ReliableOrdered, &buff);
        frame.message_index = self.message_index;
        frame.order_index = self.order_index;
        self.message_index += 1;
        self.order_index += 1;
        self.send(frame);
    }

    pub fn disconnect(&mut self) {
        if !self.dissconnected {
            let mut frame = Frame::new(Reliability::ReliableOrdered, &[Disconnected::ID]);
            self.message_index += 1;
            self.order_index += 1;
            frame.message_index = self.message_index;
            frame.order_index = self.order_index;
            self.send(frame);
        }
    }

    fn disconnected(&mut self) {
        self.dissconnected = true;
        self.event_queue
            .push(RaknetEvent::Disconnected(self.address, self.guid));
    }

    pub fn time_stamp(&mut self) -> i64 {
        match self.timer.elapsed().as_millis().try_into() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{}", e);
                0
            }
        }
    }
}
