use rand::random;
use std::{
    collections::HashMap,
    io::{Error, Result},
    net::SocketAddr,
    sync::Arc,
    time::Instant,
};
use tokio::{net::UdpSocket, sync::Mutex};

use crate::{
    connection::Connection,
    packet::RaknetPacket,
    packets::{
        already_connected::AlreadyConnected, decode, encode,
        incompatible_protocol_version::IncompatibleProtocolVersion,
        open_connection_reply1::OpenConnectionReply1, open_connection_reply2::OpenConnectionReply2,
        open_connection_request1::OpenConnectionRequest1,
        open_connection_request2::OpenConnectionRequest2, unconnected_ping::UnconnectedPing,
        unconnected_pong::UnconnectedPong, Packet,
    },
};

const RAKNET_PROTOCOL_VERSION: u8 = 0xA;

#[derive(Clone, Copy)]
pub enum DisconnectReason {
    Timeout,
    Disconnect,
}
pub enum RaknetEvent {
    Packet(RaknetPacket),
    Connected(SocketAddr, u64),
    Disconnected(SocketAddr, u64, DisconnectReason),
    Error(SocketAddr, Error),
}

trait ServerHandler {
    fn on_connected(&mut self, address: SocketAddr);
    fn on_message(&mut self, address: SocketAddr, packet: RaknetPacket);
    fn on_disconnected(&mut self, address: SocketAddr);
    fn on_error(&mut self, e: Error);
}

impl Clone for RaknetEvent {
    fn clone(&self) -> Self {
        match self {
            RaknetEvent::Packet(p) => RaknetEvent::Packet(p.clone()),
            RaknetEvent::Connected(p, e) => RaknetEvent::Connected(*p, *e),
            RaknetEvent::Disconnected(p, e, r) => RaknetEvent::Disconnected(*p, *e, *r),
            RaknetEvent::Error(p, err) => {
                RaknetEvent::Error(*p, std::io::Error::new(err.kind(), err.to_string()))
            }
        }
    }
}

pub struct Server {
    pub socket: Option<Arc<UdpSocket>>,
    pub connection: Arc<Mutex<HashMap<SocketAddr, Connection>>>,
    pub id: u64,
    pub title: Arc<Mutex<String>>,
    time: Instant,
    local_addr: SocketAddr,
    connected_clients: Arc<Mutex<Vec<u64>>>,
}

impl Server {
    pub fn new(address: SocketAddr, title: String) -> Self {
        Self {
            socket: None,
            connection: Arc::new(Mutex::new(HashMap::new())),
            id: random::<u64>(),
            title: Arc::new(Mutex::new(title)),
            time: Instant::now(),
            local_addr: address,
            connected_clients: Arc::new(Mutex::new(vec![])),
        }
    }

    pub async fn listen(&mut self) {
        self.socket = Some(Arc::new(
            UdpSocket::bind(self.local_addr)
                .await
                .unwrap_or_else(|e| panic!("failed to bind socket {}", e)),
        ));
        let socket2 = self.socket.clone().expect("failed to clone Arc<UdpSocket>");
        let connections2 = self.connection.clone();
        let connected_client = self.connected_clients.clone();
        let id = self.id;
        let motd = self.title.clone();
        let time = self.time;
        tokio::spawn(async move {
            let mut v = [0u8; 1500];
            loop {
                let (size, source) = match socket2.recv_from(&mut v).await {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                };
                if !connections2.lock().await.contains_key(&source) {
                    //not connected
                    let buff = &v[..size];
                    match buff[0] {
                        UnconnectedPing::ID => {
                            let p = match decode::<UnconnectedPing>(buff) {
                                Ok(p) => p,
                                Err(e) => {
                                    eprintln!("failed to decode unconnectedping {}", e);
                                    continue;
                                }
                            };
                            let pong =
                                UnconnectedPong::new(p.time, id, motd.lock().await.to_string());
                            if let Ok(data) = encode::<UnconnectedPong>(pong) {
                                match socket2.send_to(&data, source).await {
                                    Ok(_) => {}
                                    Err(e) => {
                                        eprintln!("failed to encode unconnectedpong {}", e);
                                    }
                                };
                            };
                        }
                        OpenConnectionRequest1::ID => {
                            let p = match decode::<OpenConnectionRequest1>(buff) {
                                Ok(p) => p,
                                Err(e) => {
                                    eprintln!("failed to decode openconnectionrequest {}", e);
                                    continue;
                                }
                            };
                            if p.protocol_version == RAKNET_PROTOCOL_VERSION {
                                let ocreply1 = OpenConnectionReply1::new(id, false, p.mtu_size);
                                if let Ok(data) = encode::<OpenConnectionReply1>(ocreply1) {
                                    match socket2.send_to(&data, source).await {
                                        Ok(_) => {}
                                        Err(e) => {
                                            eprintln!("{}", e);
                                        }
                                    };
                                } else {
                                    eprintln!("failed to encode openconnectionreply");
                                };
                            } else {
                                let reply =
                                    IncompatibleProtocolVersion::new(RAKNET_PROTOCOL_VERSION, id);
                                if let Ok(data) = encode::<IncompatibleProtocolVersion>(reply) {
                                    match socket2.send_to(&data, source).await {
                                        Ok(_) => {}
                                        Err(e) => {
                                            eprintln!("{}", e);
                                        }
                                    };
                                } else {
                                    eprintln!("failed to encode incompatibleprotocolversion");
                                };
                            }
                        }
                        OpenConnectionRequest2::ID => {
                            let p = match decode::<OpenConnectionRequest2>(buff) {
                                Ok(p) => p,
                                Err(e) => {
                                    eprintln!("failed to decode openconnectionrequest2 {}", e);
                                    continue;
                                }
                            };
                            if connected_client.lock().await.contains(&p.guid) {
                                let already_connected = AlreadyConnected::new(id);
                                if let Ok(data) = encode::<AlreadyConnected>(already_connected) {
                                    match socket2.send_to(&data, source).await {
                                        Ok(_) => {}
                                        Err(e) => {
                                            eprintln!("{}", e);
                                            continue;
                                        }
                                    };
                                } else {
                                    eprintln!("failed to encode alreadyconnected");
                                };
                                continue;
                            }

                            let ocreply2 = OpenConnectionReply2::new(id, source, p.mtu, false);
                            if let Ok(data) = encode::<OpenConnectionReply2>(ocreply2) {
                                match socket2.send_to(&data, source).await {
                                    Ok(_) => {}
                                    Err(e) => {
                                        eprintln!("{}", e);
                                        continue;
                                    }
                                };
                                connections2.lock().await.insert(
                                    source,
                                    Connection::new(source, socket2.clone(), id, time, p.mtu),
                                );
                                connected_client.lock().await.push(p.guid);
                            } else {
                                eprintln!("failed to encode openconnectionreply2");
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
                        .handle(&v[..size])
                        .await;
                }
            }
        });
    }

    pub async fn recv(&self) -> Result<Vec<RaknetEvent>> {
        let mut events: Vec<RaknetEvent> = vec![];
        let mut disconnected_clients = vec![];
        for (_address, connection) in self.connection.lock().await.iter_mut() {
            for event in connection.event_queue.clone() {
                if let RaknetEvent::Disconnected(addr, _guid, _reason) = event {
                    disconnected_clients.push(addr);
                }
                events.push(event);
            }
            connection.update().await;
        }
        for addr in disconnected_clients.iter() {
            self.connection.lock().await.remove(addr);
        }
        disconnected_clients.clear();
        Ok(events)
    }

    pub async fn send_to(&mut self, addr: &SocketAddr, buff: &[u8]) -> Result<()> {
        if !self.connection.lock().await.contains_key(addr) {
            panic!("No connection found!!!!")
        }
        self.connection
            .lock()
            .await
            .get_mut(addr)
            .unwrap()
            .send_to(buff);
        Ok(())
    }

    pub async fn set_motd(&mut self, motd: String) -> Result<()> {
        let mut old = self.title.lock().await;
        *old = motd;
        Ok(())
    }

    pub async fn disconnect(&mut self, addr: SocketAddr) {
        if !self.connection.lock().await.contains_key(&addr) {
            return;
        }

        self.connection
            .lock()
            .await
            .get_mut(&addr)
            .unwrap()
            .disconnect();
    }
}

pub struct Client {
    pub socket: Option<Arc<UdpSocket>>,
    remote: SocketAddr,
    connection: Arc<Mutex<Option<Connection>>>,
    guid: u64,
    mtu: u16,
    time: Instant,
    local: SocketAddr,
}

impl Client {
    pub fn new(remote_address: SocketAddr, online: bool) -> Self {
        let local: SocketAddr = {
            if online {
                "0.0.0.0:0".parse().unwrap()
            } else {
                "127.0.0.1:0".parse().unwrap()
            }
        };
        Self {
            socket: None,
            remote: remote_address,
            connection: Arc::new(Mutex::new(None)),
            guid: random::<u64>(),
            mtu: 1492,
            time: Instant::now(),
            local,
        }
    }

    pub async fn listen(&mut self) {
        self.socket = Some(Arc::new(
            UdpSocket::bind(self.local)
                .await
                .unwrap_or_else(|e| panic!("failed to bind socket {}", e)),
        ));
        let socket2 = self.socket.clone().expect("failed to clone Arc<UdpSocket>");
        let connections2 = self.connection.clone();
        let guid = self.guid;
        let mtu = self.mtu;
        let timer = self.time;
        let remote = self.remote;
        tokio::spawn(async move {
            let mut v = [0u8; 1500];
            loop {
                let (size, source) = match socket2.recv_from(&mut v).await {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                };
                let buff = &v[..size];
                if !source.eq(&remote) {
                    println!("packet from unknown address {}", source);
                    continue;
                }
                if let Some(conn) = connections2.lock().await.as_mut() {
                    conn.handle(buff).await;
                    continue;
                }

                match buff[0] {
                    OpenConnectionReply1::ID => {
                        let reply1 = match decode::<OpenConnectionReply1>(buff) {
                            Ok(p) => p,
                            Err(e) => {
                                eprintln!("failed to decode openconnectionreply1 {}", e);
                                continue;
                            }
                        };
                        let request2 = OpenConnectionRequest2::new(source, reply1.mtu_size, guid);
                        if let Ok(payload) = encode::<OpenConnectionRequest2>(request2) {
                            match socket2.send_to(&payload, source).await {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("{}", &e);
                                }
                            };
                        }
                    }
                    OpenConnectionReply2::ID => {
                        match decode::<OpenConnectionReply2>(buff) {
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("failed to decode openconnectionreply2 {}", e);
                                continue;
                            }
                        };
                        let connection = Connection::new(source, socket2.clone(), guid, timer, mtu);
                        *connections2.lock().await = Some(connection);
                        connections2.lock().await.as_mut().unwrap().connect();
                    }
                    IncompatibleProtocolVersion::ID => {
                        let version = match decode::<IncompatibleProtocolVersion>(buff) {
                            Ok(p) => p,
                            Err(e) => {
                                eprintln!("failed to decode incompatibleprotocolversion {}", e);
                                continue;
                            }
                        };
                        panic!(
                            "different server : {}, client : {}",
                            version.server_protocol, RAKNET_PROTOCOL_VERSION
                        );
                    }
                    AlreadyConnected::ID => {
                        let _alredy_connected = match decode::<AlreadyConnected>(buff) {
                            Ok(p) => p,
                            Err(e) => {
                                eprintln!("failed to decode alreadyconnected {}", e);
                                continue;
                            }
                        };
                        panic!("already connected");
                    }
                    _ => {
                        println!("unknown packet ID {}", buff[0]);
                    }
                }
            }
        });
    }
    pub async fn connect(&self) {
        if let Some(socket) = self.socket.clone() {
            let request1 = OpenConnectionRequest1::new(RAKNET_PROTOCOL_VERSION, self.mtu);
            if let Ok(payload) = encode::<OpenConnectionRequest1>(request1) {
                match socket.send_to(&payload, self.remote).await {
                    Ok(_) => {}
                    Err(e) => eprintln!("{}", &e),
                };
            };
        }
    }
    pub async fn disconnect(&mut self) {
        if let Some(conn) = self.connection.lock().await.as_mut() {
            conn.disconnect();
        }
    }
    pub async fn send(&mut self, buff: &[u8]) -> Result<()> {
        if let Some(conn) = self.connection.lock().await.as_mut() {
            conn.send_to(buff);
        }
        Ok(())
    }
    pub async fn recv(&self) -> Result<Vec<RaknetEvent>> {
        let mut events: Vec<RaknetEvent> = vec![];
        if let Some(conn) = self.connection.lock().await.as_mut() {
            for event in conn.event_queue.clone() {
                events.push(event);
            }
            conn.update().await;
        }
        Ok(events)
    }

    pub fn address(&self) -> SocketAddr {
        self.local
    }
}

pub struct Ping {
    socket: UdpSocket,
}

impl Ping {
    pub async fn new() -> Self {
        Self {
            socket: UdpSocket::bind("0.0.0.0:0")
                .await
                .expect("Unable to bind to address"),
        }
    }
    pub async fn ping(&self, address: SocketAddr) -> Result<String> {
        let unconnected_ping = UnconnectedPing::new(0, 0);
        let payload = encode::<UnconnectedPing>(unconnected_ping)?;
        let mut ret = String::new();
        self.socket.send_to(&payload, address).await?;
        let mut v = [0u8; 1500];
        let (size, _source) = self.socket.recv_from(&mut v).await?;
        let buff = &v[..size];
        if buff[0] == UnconnectedPong::ID {
            let pong = decode::<UnconnectedPong>(buff)?;
            ret = pong.motd;
        }

        Ok(ret)
    }
}
