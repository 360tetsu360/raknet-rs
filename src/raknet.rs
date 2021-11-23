use rand::random;
use std::{collections::HashMap, net::SocketAddr};
use tokio::net::{ToSocketAddrs, UdpSocket};

use crate::{
    connection::Connection,
    packets::{
        open_connection_reply1::OpenConnectionReply1, open_connection_reply2::OpenConnectionReply2,
        unconnected_pong::UnconnectedPong, Packets,
    },
};
pub struct Server {
    pub socket: UdpSocket,
    pub connection: HashMap<SocketAddr, Connection>,
    pub id: u64,
    pub title: String,
}

impl Server {
    pub async fn new(address: impl ToSocketAddrs, title: String) -> Self {
        println!("packet!");
        Self {
            socket: UdpSocket::bind(address).await.unwrap(),
            connection: HashMap::new(),
            id: random::<u64>(),
            title,
        }
    }

    pub async fn listen(&self) {
        loop {
            let mut v = [0; 2048];
            let (_size, source) = self
                .socket
                .recv_from(&mut v)
                .await
                .expect("Failed to receive datagram");
            self.handle(v, source).await;
            // これsourceに元アドレス入る
        }
    }

    pub async fn handle<T: AsMut<[u8]>>(&self, mut buff: T, source: SocketAddr) {
        if !self.connection.contains_key(&source) {
            //not connected
            let packet = match Packets::decode(buff.as_mut()) {
                Ok(s) => s,
                Err(err) => {
                    println!("{}", &err);
                    Packets::Error(())
                }
            };
            match packet {
                Packets::UnconnectedPing(p) => {
                    let pong = UnconnectedPong::new(p.time, self.id, self.title.clone());
                    if let Ok(data) = Packets::UnconnectedPong(pong).encode() {
                        let _ = self.socket.send_to(&data, source).await.unwrap();
                    };
                }
                Packets::OpenConnectionRequest1(p) => {
                    let ocreply1 = OpenConnectionReply1::new(self.id, false, p.mtu_size);
                    if let Ok(data) = Packets::OpenConnectionReply1(ocreply1).encode() {
                        let _ = self.socket.send_to(&data, source).await.unwrap();
                    };
                }
                Packets::OpenConnectionRequest2(p) => {
                    let ocreply2 = OpenConnectionReply2::new(self.id, source, p.mtu, false);
                    if let Ok(data) = Packets::OpenConnectionReply2(ocreply2).encode() {
                        let _ = self.socket.send_to(&data, source).await.unwrap();
                    };
                    //connected!
                }
                _ => {}
            }
        } else {
            //self.connection[&source].handle_datagram(buff.as_mut());
        }
    }
}

pub struct Client {
    pub socket: UdpSocket,
}

impl Client {
    pub async fn new(_remote_address: Option<impl ToSocketAddrs>) -> Self {
        Self {
            socket: UdpSocket::bind("0.0.0.0:0")
                .await
                .expect("Unable to bind to address"),
        }
    }
    pub fn address(&self) -> SocketAddr {
        self.socket.local_addr().unwrap()
    }
}
