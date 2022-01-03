use rand::random;
use std::{io::Result, net::SocketAddr, sync::Arc, time::Instant};
use tokio::{
    net::UdpSocket,
    sync::{mpsc::Receiver, Mutex},
};

use crate::server::{RaknetError, RaknetEvent};

use crate::{connection::Connection, packets::*};

const RAKNET_PROTOCOL_VERSION: u8 = 0xA;

pub struct Client {
    pub socket: Option<Arc<UdpSocket>>,
    remote: SocketAddr,
    connection: Arc<Mutex<Option<Connection>>>,
    event: Arc<Mutex<Vec<RaknetEvent>>>,
    guid: u64,
    mtu: u16,
    time: Instant,
    local: SocketAddr,
    reveiver: Arc<Mutex<Option<Receiver<RaknetEvent>>>>,
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
            event: Arc::new(Mutex::new(vec![])),
            guid: random::<u64>(),
            mtu: 1492,
            time: Instant::now(),
            local,
            reveiver: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn listen(&mut self) {
        self.socket = Some(Arc::new(
            UdpSocket::bind(self.local)
                .await
                .unwrap_or_else(|e| panic!("failed to bind socket {}", e)),
        ));
        let socket2 = self.socket.clone().expect("failed to clone Arc<UdpSocket>");
        let connection2 = self.connection.clone();
        let event = self.event.clone();
        let guid = self.guid;
        let mtu = self.mtu;
        let timer = self.time;
        let remote = self.remote;
        let receiver2 = self.reveiver.clone();
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

                let socket3 = socket2.clone();
                let event2 = event.clone();
                let connection3 = connection2.clone();
                let receiver3 = receiver2.clone();
                tokio::spawn(async move {
                    let buff = &v[..size];
                    if !source.eq(&remote) {
                        println!("packet from unknown address {}", source);
                        return;
                    }
                    if let Some(conn) = connection3.lock().await.as_mut() {
                        conn.handle(buff).await;
                        return;
                    }

                    match buff[0] {
                        OpenConnectionReply1::ID => {
                            let reply1 = match decode::<OpenConnectionReply1>(buff).await {
                                Ok(p) => p,
                                Err(e) => {
                                    eprintln!("failed to decode openconnectionreply1 {}", e);
                                    return;
                                }
                            };
                            let request2 =
                                OpenConnectionRequest2::new(source, reply1.mtu_size, guid);
                            if let Ok(payload) = encode(request2).await {
                                match socket3.send_to(&payload, source).await {
                                    Ok(_) => {}
                                    Err(e) => {
                                        eprintln!("{}", &e);
                                    }
                                };
                            }
                        }
                        OpenConnectionReply2::ID => {
                            match decode::<OpenConnectionReply2>(buff).await {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("failed to decode openconnectionreply2 {}", e);
                                    return;
                                }
                            };
                            let (s, r) = tokio::sync::mpsc::channel::<RaknetEvent>(1024);
                            *receiver3.lock().await = Some(r);
                            let connection =
                                Connection::new(source, socket3.clone(), guid, timer, mtu, s);
                            *connection3.lock().await = Some(connection);

                            connection3.lock().await.as_mut().unwrap().connect().await;
                        }
                        IncompatibleProtocolVersion::ID => {
                            let version = match decode::<IncompatibleProtocolVersion>(buff).await {
                                Ok(p) => p,
                                Err(e) => {
                                    eprintln!("failed to decode incompatibleprotocolversion {}", e);
                                    return;
                                }
                            };
                            event2.lock().await.push(RaknetEvent::Error(
                                remote,
                                RaknetError::IncompatibleProtocolVersion(
                                    version.server_protocol,
                                    RAKNET_PROTOCOL_VERSION,
                                ),
                            ));
                        }
                        AlreadyConnected::ID => {
                            let _alredy_connected = match decode::<AlreadyConnected>(buff).await {
                                Ok(p) => p,
                                Err(e) => {
                                    eprintln!("failed to decode alreadyconnected {}", e);
                                    return;
                                }
                            };
                            event2.lock().await.push(RaknetEvent::Error(
                                remote,
                                RaknetError::AlreadyConnected(remote),
                            ));
                        }
                        _ => {
                            println!("unknown packet ID {:x}", buff[0]);
                        }
                    }
                });
            }
        });

        let connection = self.connection.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                if let Some(conn) = connection.lock().await.as_mut() {
                    conn.update().await;
                }
            }
        });
    }
    pub async fn connect(&self) {
        if let Some(socket) = self.socket.clone() {
            let request1 = OpenConnectionRequest1::new(RAKNET_PROTOCOL_VERSION, self.mtu);
            if let Ok(payload) = encode(request1).await {
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
        let mut events: Vec<RaknetEvent> = self.event.lock().await.clone();
        self.event.lock().await.clear();
        if self.connection.lock().await.as_mut().is_some() {
            while let Ok(event) = (*self.reveiver.lock().await).as_mut().unwrap().try_recv() {
                events.push(event);
            }
        }
        Ok(events)
    }

    pub fn address(&self) -> SocketAddr {
        self.local
    }
}
