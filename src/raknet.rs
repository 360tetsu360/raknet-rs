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
    packet::RaknetPacket,
    packets::{
        decode, encode, open_connection_reply1::OpenConnectionReply1,
        open_connection_reply2::OpenConnectionReply2,
        open_connection_request1::OpenConnectionRequest1,
        open_connection_request2::OpenConnectionRequest2, unconnected_ping::UnconnectedPing,
        unconnected_pong::UnconnectedPong, Packet,
    },
};

const RAKNET_PROTOCOL_VERSION : u8 = 0xa;
pub enum RaknetEvent {
    Packet(RaknetPacket),
    Connected(SocketAddr, u64),
    Disconnected(SocketAddr, u64),
    Error(SocketAddr, Error),
}

impl Clone for RaknetEvent {
    fn clone(&self) -> Self {
        match self {
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
    pub title: Arc<Mutex<String>>,
    time: Instant,
}

impl Server {
    pub async fn new(address: impl ToSocketAddrs, title: String) -> Self {
        Self {
            socket: Arc::new(UdpSocket::bind(address).await.unwrap()),
            connection: Arc::new(Mutex::new(HashMap::new())),
            id: random::<u64>(),
            title: Arc::new(Mutex::new(title)),
            time: Instant::now(),
        }
    }

    pub fn listen(&self) {
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
                    let buff = &v[..size];
                    match buff[0] {
                        UnconnectedPing::ID => {
                            let p = decode::<UnconnectedPing>(buff).unwrap();
                            let pong =
                                UnconnectedPong::new(p.time, id, motd.lock().await.to_string());
                            if let Ok(data) = encode::<UnconnectedPong>(pong) {
                                let _ = socket2.send_to(&data, source).await.unwrap();
                            };
                        }
                        OpenConnectionRequest1::ID => {
                            let p = decode::<OpenConnectionRequest1>(buff).unwrap();
                            let ocreply1 = OpenConnectionReply1::new(id, false, p.mtu_size);
                            if let Ok(data) = encode::<OpenConnectionReply1>(ocreply1) {
                                let _ = socket2.send_to(&data, source).await.unwrap();
                            };
                        }
                        OpenConnectionRequest2::ID => {
                            let p = decode::<OpenConnectionRequest2>(buff).unwrap();
                            let ocreply2 = OpenConnectionReply2::new(id, source, p.mtu, false);
                            if let Ok(data) = encode::<OpenConnectionReply2>(ocreply2) {
                                let _ = socket2.send_to(&data, source).await.unwrap();
                                connections2.lock().await.insert(
                                    source,
                                    Connection::new(
                                        source,
                                        socket2.clone(),
                                        id,
                                        time,
                                        p.mtu,
                                    ),
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

    pub async fn set_motd(&mut self, motd: String) -> Result<()> {
        let mut old = self.title.lock().await;
        *old = motd;
        Ok(())
    }
}

pub struct Client {
    pub socket: Arc<UdpSocket>,
    remote : SocketAddr,
    connection: Arc<Mutex<Option<Connection>>>,
    guid: u64,
    mtu : u16,
    time: Instant,
}

impl Client {
    pub async fn new(remote_address: SocketAddr,online : bool) -> Self {
        let local : SocketAddr = {
            if online {
                "0.0.0.0:0".parse().unwrap()
            }else{
                "127.0.0.1:0".parse().unwrap()
            }
        };
        Self {
            socket: Arc::new(UdpSocket::bind(local).await.unwrap()),
            remote : remote_address,
            connection: Arc::new(Mutex::new(None)),
            guid : random::<u64>(),
            mtu : 1492,
            time: Instant::now(),
        }
    }

    pub fn listen(&self) {
        let socket2 = self.socket.clone();
        let connections2 = self.connection.clone();
        let guid = self.guid;
        let mtu = self.mtu;
        let timer = self.time;
        let remote = self.remote;
        tokio::spawn(async move {
            let mut v = [0u8; 1500];
            loop {
                let (size, source) = socket2.recv_from(&mut v).await.unwrap();
                let buff = &v[..size];
                if !source.eq(&remote){
                    println!("packet from unknown address {}",source);
                    continue;
                }
                if let Some(conn) = connections2.lock().await.as_mut() {
                    conn.handle(buff);
                    continue;
                }

                match buff[0] {
                    OpenConnectionReply1::ID => {
                        let reply1 = decode::<OpenConnectionReply1>(buff).unwrap();
                        let request2 = OpenConnectionRequest2::new(source, reply1.mtu_size, guid);
                        let payload = encode::<OpenConnectionRequest2>(request2).unwrap();
                        socket2.send_to(&payload, source).await.unwrap();
                    },
                    OpenConnectionReply2::ID => {
                        let connection = Connection::new(
                            source,
                            socket2.clone(),
                            guid,
                            timer,
                            mtu
                        );
                        *connections2.lock().await = Some(connection);
                        connections2.lock().await.as_mut().unwrap().connect();
                    },
                    _=>{
                        println!("unknown packet ID {}",buff[0]);
                    }
                }
            }
        });
    }
    pub async fn connect(&self) {
        let request1 = OpenConnectionRequest1::new(RAKNET_PROTOCOL_VERSION,self.mtu);
        let payload = encode::<OpenConnectionRequest1>(request1).unwrap();
        self.socket.send_to(&payload, self.remote).await.unwrap();
    }

    pub async fn recv(&self) -> Result<Vec<RaknetEvent>> {
        let mut events: Vec<RaknetEvent> = vec![];
        let mut disconnected_clients = vec![];
        if let Some(conn) = self.connection.lock().await.as_mut() {
            for event in conn.event_queue.clone() {
                if let RaknetEvent::Disconnected(addr, _guid) = event {
                    disconnected_clients.push(addr);
                }
                events.push(event);
            }
            conn.update();
        }

        for _addr in disconnected_clients.iter() {
            //TODO!!!!
            //self.connection.lock().await.remove(addr);
        }
        disconnected_clients.clear();
        Ok(events)
    }
    pub fn address(&self) -> SocketAddr {
        self.socket.local_addr().unwrap()
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
        let (size, _source) = self.socket.recv_from(&mut v).await.unwrap();
        let buff = &v[..size];
        if buff[0] == UnconnectedPong::ID {
            let pong = decode::<UnconnectedPong>(buff)?;
            ret = pong.motd;
        }

        Ok(ret)
    }
}
