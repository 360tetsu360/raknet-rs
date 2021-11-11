use rand::random;
use tokio::net::{ToSocketAddrs, UdpSocket};
use std::{collections::HashMap, fmt::Result,io, net::SocketAddr};

use crate::{connection::Connection, packets::{Packets, unconnected_ping::UnconnectedPing, unconnected_pong::UnconnectedPong}};

pub struct Server {
    pub socket: UdpSocket,
    pub connection: HashMap<SocketAddr, Connection>,
    pub id: u64,
}

impl Server {
    pub async fn new(address: impl ToSocketAddrs) -> Self {
        println!("packet!");
        let listener = Self {
            socket: UdpSocket::bind(address).await.unwrap(),
            connection: HashMap::new(),
            id: random::<u64>(),
        };
        listener.listen().await;
        listener
    }

    pub async fn listen(&self) {
        loop {
            let mut v = [0; 2048];
            let (_size, source) = self
                .socket
                .recv_from(&mut v).await
                .expect("Failed to receive datagram");
            self.handle(v, source).await;
            // これsourceに元アドレス入る
        }
    }

    pub async fn handle<T: AsMut<[u8]>>(&self,mut buff: T, source: SocketAddr) {
        if !self.connection.contains_key(&source) {
        }
        else {
            //self.connection[&source].handle_datagram(buff.as_mut());
        }
    }
}


pub struct Client{
    pub socket : UdpSocket
}

impl Client {
    pub async fn new(_remote_address : Option<impl ToSocketAddrs>) -> Self {
        Self {
            socket: UdpSocket::bind("0.0.0.0:0").await.expect("Unable to bind to address")
        }
    }
    pub fn address(&self) -> SocketAddr {
        self.socket.local_addr().unwrap()
    }

}