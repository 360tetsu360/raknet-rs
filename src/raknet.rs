use rand::random;
use std::{
    collections::HashMap,
    io::{Error, Result},
    net::SocketAddr,
    sync::Arc,
    time::Instant,
};
use tokio::{
    net::{ToSocketAddrs, UdpSocket},
    sync::Mutex,
};

use crate::{
    connection::Connection,
    packet::Packet,
    packets::{
        open_connection_reply1::OpenConnectionReply1, open_connection_reply2::OpenConnectionReply2,
        unconnected_pong::UnconnectedPong, Packets,
    },
};

pub enum RaknetEvent {
    Packet(Packet),
    Connected(SocketAddr, u64),
    Disconnected(SocketAddr, u64),
    Error(SocketAddr, Error),
}

impl Clone for RaknetEvent {
    fn clone(&self) -> Self {
        match &*self {
            RaknetEvent::Packet(p) => RaknetEvent::Packet(p.clone()),
            RaknetEvent::Connected(p, e) => RaknetEvent::Connected(*p, *e),
            RaknetEvent::Disconnected(p, e) => RaknetEvent::Disconnected(*p, *e),
            RaknetEvent::Error(p, err) => {
                RaknetEvent::Error(*p, std::io::Error::new(err.kind(), err.to_string()))
            }
        }
    }
}

pub struct Server {
    pub socket: Arc<UdpSocket>,
    pub connection: Arc<Mutex<HashMap<SocketAddr, Connection>>>,
    pub id: u64,
    pub title: Arc<String>,
    time: Instant,
}

impl Server {
    pub async fn new(address: impl ToSocketAddrs, title: String) -> Self {
        Self {
            socket: Arc::new(UdpSocket::bind(address).await.unwrap()),
            connection: Arc::new(Mutex::new(HashMap::new())),
            id: random::<u64>(),
            title: Arc::new(title),
            time: Instant::now(),
        }
    }

    pub async fn listen(&self) {
        let socket2 = self.socket.clone();
        let connections2 = self.connection.clone();
        let id = self.id;
        let motd = self.title.clone();
        let time = self.time;
        tokio::spawn(async move {
            let mut v = [0u8; 1500];
            loop {
                let (size, source) = socket2.recv_from(&mut v).await.unwrap();
                if !connections2.lock().await.contains_key(&source) {
                    //not connected
                    let packet = match Packets::decode(&v[..size]) {
                        Ok(s) => s,
                        Err(err) => {
                            println!("{}", &err);
                            Packets::Error(())
                        }
                    };
                    match packet {
                        Packets::UnconnectedPing(p) => {
                            let pong = UnconnectedPong::new(p.time, id, motd.to_string());
                            if let Ok(data) = Packets::UnconnectedPong(pong).encode() {
                                let _ = socket2.send_to(&data, source).await.unwrap();
                            };
                        }
                        Packets::OpenConnectionRequest1(p) => {
                            let ocreply1 = OpenConnectionReply1::new(id, false, p.mtu_size);
                            if let Ok(data) = Packets::OpenConnectionReply1(ocreply1).encode() {
                                let _ = socket2.send_to(&data, source).await.unwrap();
                            };
                        }
                        Packets::OpenConnectionRequest2(p) => {
                            let ocreply2 = OpenConnectionReply2::new(id, source, p.mtu, false);
                            if let Ok(data) = Packets::OpenConnectionReply2(ocreply2).encode() {
                                let _ = socket2.send_to(&data, source).await.unwrap();
                                connections2.lock().await.insert(
                                    source,
                                    Connection::new(source, socket2.clone(), id, time),
                                );
                            };
                            //connected!
                        }
                        _ => {}
                    }
                } else {
                    connections2
                        .lock()
                        .await
                        .get_mut(&source)
                        .unwrap()
                        .handle(&v[..size]); //todo : error handling
                }
            }
        });
    }

    pub async fn recv(&self) -> Result<Vec<RaknetEvent>> {
        let mut events: Vec<RaknetEvent> = vec![];
        let mut disconnected_clients = vec![];
        for (_address, connection) in self.connection.lock().await.iter_mut() {
            for event in connection.event_queue.clone() {
                if let RaknetEvent::Disconnected(addr, _guid) = event {
                    disconnected_clients.push(addr);
                }
                events.push(event);
            }
            connection.update();
        }
        for addr in disconnected_clients.iter() {
            self.connection.lock().await.remove(addr);
        }
        disconnected_clients.clear();
        Ok(events)
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
