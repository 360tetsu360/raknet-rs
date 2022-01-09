use crate::{
    macros::*,
    packet::{ACKQueue, RaknetPacket},
    packetqueue::PacketQueue,
    packets::*,
    receivedqueue::ReceivedQueue,
    time, DisconnectReason, RaknetEvent,
};
use std::{collections::VecDeque, convert::TryInto, net::SocketAddr, sync::Arc};
use tokio::{net::UdpSocket, sync::mpsc::Sender};
const DATAGRAM_FLAG: u8 = 0x80;

const ACK_FLAG: u8 = 0x40;

const NACK_FLAG: u8 = 0x20;

pub enum RaknetType {
    Client,
    Server,
}

pub struct Connection {
    pub address: SocketAddr,
    socket: Arc<UdpSocket>,
    pub event_sender: Sender<RaknetEvent>,
    pub guid: u64,
    pub opponent_guid: u64,
    pub mtu: u16,
    pub last_receive: u128,
    ack_queue: ACKQueue,
    packet_queue: PacketQueue,
    message_index: u32,
    order_index: u32,
    split_id: u16,
    received: ReceivedQueue,
    last_ping: u128,
    dissconnected: bool,
    recovery_queue: VecDeque<RaknetEvent>,
    rak_type: RaknetType,
}

impl Connection {
    pub fn new(
        address: SocketAddr,
        socket: Arc<UdpSocket>,
        guid: u64,
        opponent_guid: u64,
        mtu: u16,
        sender: Sender<RaknetEvent>,
        rak_type: RaknetType,
    ) -> Self {
        let time = time();
        Self {
            address,
            socket,
            event_sender: sender,
            guid,
            opponent_guid,
            mtu,
            last_receive: time,
            ack_queue: ACKQueue::new(),
            packet_queue: PacketQueue::new(mtu, time),
            message_index: 0,
            order_index: 0,
            split_id: 0,
            received: ReceivedQueue::new(),
            last_ping: time,
            dissconnected: false,
            recovery_queue: VecDeque::new(),
            rak_type,
        }
    }
    pub async fn update(&mut self) {
        self.flush_queue().await;
        self.flush_ack().await;
        self.recovery();
        let time = time();
        if (time - self.last_receive) > 10000 {
            self.disconnect();
            self.disconnected(DisconnectReason::Timeout).await;
        }
        if (time - self.last_ping) > 5 * 1000 {
            // 1/s
            self.last_ping = time;
            self.send_ping().await;
        }
    }
    fn recovery(&mut self) {
        if !self.recovery_queue.is_empty() {
            while let Some(event) = self.recovery_queue.pop_front() {
                if self.event_sender.try_send(event).is_err() {
                    break;
                }
            }
        }
    }
    pub async fn connect(&mut self) {
        let request = ConnectionRequest::new(self.guid, time() as i64, false);
        let buff = unwrap_or_dbg!(encode(request).await);
        let frame = Frame::new(Reliability::Reliable, &buff);
        self.send(frame);
    }
    pub async fn handle(&mut self, buff: &[u8]) {
        let header = buff[0];

        self.last_receive = time();

        if header & ACK_FLAG != 0 {
            self.handle_ack(buff).await;
        } else if header & NACK_FLAG != 0 {
            self.handle_nack(buff).await;
        } else if header & DATAGRAM_FLAG != 0 {
            self.handle_datagram(buff).await;
        }
    }
    async fn flush_ack(&mut self) {
        let acks = self.ack_queue.get_send_able_and_clear();
        for ack in acks {
            self.send_ack(ack).await;
        }
    }
    async fn send_ping(&mut self) {
        let connected_ping = ConnectedPing::new(self.last_ping as i64);
        let buff = unwrap_or_dbg!(encode(connected_ping).await);
        let frame = Frame::new(Reliability::Unreliable, &buff);
        self.send(frame);
    }
    async fn send_ack(&mut self, packet: (u32, u32)) {
        if self.dissconnected {
            return;
        }
        let ack = Ack::new(packet);
        let buff = unwrap_or_dbg!(encode(ack).await);
        unwrap_or_dbg!(self.socket.send_to(&buff, self.address).await);
    }
    async fn send_nack(&mut self, packet: u32) {
        if self.dissconnected {
            return;
        }
        let nack = Nack::new((packet, packet));
        let buff = unwrap_or_dbg!(encode(nack).await);
        unwrap_or_dbg!(self.socket.send_to(&buff, self.address).await);
    }
    async fn handle_ack(&mut self, buff: &[u8]) {
        let ack = unwrap_or_return!(decode::<Ack>(buff).await);
        for sequence in ack.get_all() {
            self.packet_queue.received(sequence);
        }
    }

    async fn handle_nack(&mut self, buff: &[u8]) {
        let nack = unwrap_or_return!(decode::<Nack>(buff).await);
        for sequence in nack.get_all() {
            self.packet_queue.resend(sequence);
        }
    }

    async fn handle_datagram(&mut self, buff: &[u8]) {
        let frame_set = unwrap_or_return!(FrameSet::decode(buff).await);
        self.ack_queue.add(frame_set.sequence_number);
        if self.ack_queue.get_missing_len() != 0 {
            for miss in self.ack_queue.get_missing() {
                self.send_nack(miss).await;
            }
        }
        for frame in frame_set.datas {
            self.receive_packet(frame).await;
        }
    }
    async fn receive_packet(&mut self, frame: Frame) {
        if !frame.reliability.sequenced_or_ordered() {
            self.handle_packet(&frame.data).await;
        } else {
            self.received.add(frame);
            for packet in self.received.get_all() {
                self.handle_packet(&packet.data).await;
            }
        }
    }

    async fn handle_packet(&mut self, payload: &[u8]) {
        match payload[0] {
            ConnectionRequest::ID => {
                self.handle_connectionrequest(payload).await;
            }
            ConnectionRequestAccepted::ID => {
                self.handle_connectionrequest_accepted(payload).await;
            }
            NewIncomingConnection::ID => {}
            ConnectedPing::ID => {
                self.handle_connectedping(payload).await;
            }
            ConnectedPong::ID => {}
            Disconnected::ID => {
                self.disconnect();
                self.disconnected(DisconnectReason::Disconnect).await;
            }
            _ => {
                let rak_packet =
                    RaknetPacket::new(self.address, self.opponent_guid, payload.to_vec());
                if self.put_event(RaknetEvent::Packet(rak_packet)) {
                    self.recovery_queue
                        .push_back(RaknetEvent::Packet(RaknetPacket::new(
                            self.address,
                            self.opponent_guid,
                            payload.to_vec(),
                        )));
                }
            }
        }
    }

    pub fn send_to(&mut self, buff: &[u8]) {
        if buff.len() < (self.mtu - 100 - 42).into() {
            let mut frame = Frame::new(Reliability::ReliableOrdered, buff);
            frame.message_index = self.message_index;
            frame.order_index = self.order_index;
            self.message_index += 1;
            self.order_index += 1;
            self.send(frame);
        } else {
            let max = self.mtu - 52 - 100;
            let mut split_len = buff.len() as u16 / max;
            if buff.len() as u16 % max != 0 {
                split_len += 1;
            }
            for i in 0..split_len {
                let range = (i * max) as usize..{
                    if ((i + 1) * max) as usize > buff.len() {
                        (i * max) as usize + (buff.len() as u16 % max) as usize
                    } else {
                        ((i + 1) * max) as usize
                    }
                };
                let mut frame = Frame::new(Reliability::ReliableOrdered, &buff[range]);
                frame.split = true;
                frame.message_index = self.message_index;
                frame.order_index = self.order_index;
                frame.split_count = split_len as u32;
                frame.split_id = self.split_id;
                frame.split_index = i as u32;
                self.send(frame);
                self.message_index += 1;
            }
            self.split_id += 1;
            self.order_index += 1;
        }
    }
    fn send(&mut self, packet: Frame) {
        self.packet_queue.add_frame(packet);
    }
    fn put_event(&mut self, event: RaknetEvent) -> bool {
        self.recovery();
        self.event_sender.try_send(event).is_err()
    }
    async fn flush_queue(&mut self) {
        let time = time();
        for send_able in self.packet_queue.get_packet(time).clone() {
            let frame_set = unwrap_or_dbg!(send_able.encode().await);
            unwrap_or_dbg!(self.socket.send_to(&frame_set, self.address).await);
        }
    }
    async fn handle_connectionrequest(&mut self, payload: &[u8]) {
        if let RaknetType::Client = self.rak_type {
            return;
        }

        let p = unwrap_or_return!(decode::<ConnectionRequest>(payload).await);

        let reply = ConnectionRequestAccepted::new(self.address, p.time, self.time_stamp());
        let buff = unwrap_or_dbg!(encode::<ConnectionRequestAccepted>(reply).await);
        let frame = Frame::new(Reliability::ReliableOrdered, &buff);
        self.send(frame);
        self.message_index += 1;
        self.order_index += 1;
        if self.put_event(RaknetEvent::Connected(self.address, self.opponent_guid)) {
            self.recovery_queue
                .push_back(RaknetEvent::Connected(self.address, self.opponent_guid));
        }
    }
    async fn handle_connectionrequest_accepted(&mut self, payload: &[u8]) {
        if let RaknetType::Server = self.rak_type {
            return;
        }

        let accepted = unwrap_or_return!(decode::<ConnectionRequestAccepted>(payload).await);

        let newincoming = NewIncomingConnection {
            server_address: self.address,
            request_timestamp: accepted.request_timestamp,
            accepted_timestamp: accepted.accepted_timestamp,
        };
        let buff = unwrap_or_dbg!(encode(newincoming).await);
        let frame = Frame::new(Reliability::ReliableOrdered, &buff);
        self.send(frame);
        self.message_index += 1;
        self.order_index += 1;
        if self.put_event(RaknetEvent::Connected(self.address, self.opponent_guid)) {
            self.recovery_queue
                .push_back(RaknetEvent::Connected(self.address, self.opponent_guid));
        }
        self.send_ping().await;
    }
    async fn handle_connectedping(&mut self, payload: &[u8]) {
        let p = unwrap_or_return!(decode::<ConnectedPing>(payload).await);

        let pong = ConnectedPong::new(p.client_timestamp, self.time_stamp());
        let buff = unwrap_or_dbg!(encode(pong).await);

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
            frame.message_index = self.message_index;
            frame.order_index = self.order_index;
            self.message_index += 1;
            self.order_index += 1;
            self.send(frame);
            self.dissconnected = true;
        }
    }

    async fn disconnected(&mut self, reason: DisconnectReason) {
        self.dissconnected = true;
        if self.put_event(RaknetEvent::Disconnected(
            self.address,
            self.opponent_guid,
            reason,
        )) {
            self.recovery_queue.push_back(RaknetEvent::Disconnected(
                self.address,
                self.guid,
                reason,
            ));
        }
    }

    pub fn time_stamp(&self) -> i64 {
        time().try_into().unwrap_or(0)
    }
}
