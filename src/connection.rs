use crate::{packets::frame_set::FrameSet, raknet::RaknetEvent};
use std::{net::SocketAddr, sync::Arc, time::Instant};
use tokio::net::UdpSocket;

pub struct Connection {
    pub address: SocketAddr,
    _socket: Arc<UdpSocket>,
    pub event_queue: Vec<RaknetEvent>,
    pub guid: u64,
    timer: Instant,
    last_recieve: u128,
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
        }
    }
    pub fn update(&mut self) {
        self.event_queue.clear();
        let time = self.timer.elapsed().as_millis();
        if (time - self.last_recieve) > 10000 {
            self.disconnect();
        }
    }
    pub fn handle_datagram(&self, buff: &[u8]) {
        let frame_set = FrameSet::decode(buff).expect("failed to read packet");
        println!("{} {}",frame_set.sequence_number,frame_set.datas.len());
    }
    pub fn disconnect(&mut self) {
        self.event_queue
            .push(RaknetEvent::Disconnected(self.address, self.guid));
    }
}
