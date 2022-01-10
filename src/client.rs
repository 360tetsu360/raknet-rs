use rand::random;
use std::{io::Result, net::SocketAddr, panic, sync::Arc};
use tokio::{
    net::UdpSocket,
    sync::{mpsc::Receiver, Mutex},
};

use crate::rak::{RaknetError, RaknetEvent};
use crate::{connection::Connection, packets::*};

use crate::macros::*;

const RAKNET_PROTOCOL_VERSION: u8 = 0xA;

pub struct Client {
    socket: Arc<UdpSocket>,
    connection: Arc<Mutex<Option<Connection>>>,
    event: Arc<Mutex<Vec<RaknetEvent>>>,
    reveiver: Arc<Mutex<Option<Receiver<RaknetEvent>>>>,

    pub guid: u64,
    pub mtu: u16,
    pub remote: SocketAddr,
    pub local: SocketAddr,
}

impl Client {
    pub async fn new(remote_address: SocketAddr, online: bool) -> std::io::Result<Self> {
        let local: SocketAddr = {
            if online {
                "0.0.0.0:0".parse().unwrap()
            } else {
                "127.0.0.1:0".parse().unwrap()
            }
        };
        let socket = Arc::new(UdpSocket::bind(local).await?);
        Ok(Self {
            socket,
            remote: remote_address,
            connection: Arc::new(Mutex::new(None)),
            event: Arc::new(Mutex::new(vec![])),
            guid: random::<u64>(),
            mtu: 1492,
            local,
            reveiver: Arc::new(Mutex::new(None)),
        })
    }

    pub async fn listen(&mut self) {
        if self.connection.lock().await.as_ref().is_none() {
            panic!("You must connect before listen")
        }

        let socket = self.socket.clone();
        let connection = self.connection.clone();
        let event = self.event.clone();
        let remote = self.remote;
        tokio::spawn(async move {
            let mut v = [0u8; 1500];
            loop {
                let (size, source) = match socket.recv_from(&mut v).await {
                    Ok(p) => p,
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::ConnectionReset {
                            event.lock().await.push(RaknetEvent::Error(
                                remote,
                                RaknetError::RemoteClosed(remote),
                            ))
                        }
                        continue;
                    }
                };

                if source != remote || size == 0 {
                    continue;
                }

                let buff = &v[..size];
                connection.lock().await.as_mut().unwrap().handle(buff).await;
            }
        });

        let connections = self.connection.clone();

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                connections.lock().await.as_mut().unwrap().update().await;
            }
        });
    }

    pub async fn connect(&mut self) -> std::result::Result<(), RaknetError> {
        let request1 = OpenConnectionRequest1::new(RAKNET_PROTOCOL_VERSION, self.mtu);
        let payload = match encode(request1).await {
            Ok(p) => p,
            Err(_) => {
                return Err(RaknetError::Other(
                    "Failed to encode OpenconnectionRequest1".to_owned(),
                ))
            }
        };
        match self.socket.send_to(&payload, self.remote).await {
            Ok(p) => p,
            Err(e) => return Err(RaknetError::Other(format!("{}", e))),
        };
        let timeout =
            tokio::time::timeout(std::time::Duration::from_secs(10), self.connect_to_server());

        match timeout.await {
            Ok(p) => p?,
            Err(_) => return Err(RaknetError::RemoteClosed(self.remote)),
        };

        Ok(())
    }

    async fn connect_to_server(&mut self) -> std::result::Result<(), RaknetError> {
        let socket = self.socket.clone();
        let connection2 = self.connection.clone();
        let guid = self.guid;
        let mtu = self.mtu;
        let remote = self.remote;
        let receiver2 = self.reveiver.clone();
        let mut v = [0u8; 1500];
        loop {
            let (size, source) = match socket.recv_from(&mut v).await {
                Ok(p) => p,
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::ConnectionReset {
                        return Err(RaknetError::RemoteClosed(remote));
                    }
                    continue;
                }
            };

            if size == 0 {
                continue;
            }

            let buff = &v[..size];

            if !source.eq(&remote) {
                continue;
            }

            match buff[0] {
                OpenConnectionReply1::ID => {
                    let reply1 = unwrap_or_continue!(decode::<OpenConnectionReply1>(buff).await);
                    let request2 = OpenConnectionRequest2::new(source, reply1.mtu_size, guid);
                    let payload = unwrap_or_continue!(encode(request2).await);
                    unwrap_or_continue!(socket.send_to(&payload, source).await);
                }
                OpenConnectionReply2::ID => {
                    let reply2 = unwrap_or_continue!(decode::<OpenConnectionReply2>(buff).await);
                    let (s, r) = tokio::sync::mpsc::channel::<RaknetEvent>(10);
                    *receiver2.lock().await = Some(r);
                    let connection = Connection::new(
                        source,
                        socket.clone(),
                        guid,
                        reply2.guid,
                        mtu,
                        s,
                        crate::connection::RaknetType::Client,
                    );
                    *connection2.lock().await = Some(connection);
                    connection2.lock().await.as_mut().unwrap().connect().await;
                    return Ok(());
                }
                IncompatibleProtocolVersion::ID => {
                    let version =
                        unwrap_or_continue!(decode::<IncompatibleProtocolVersion>(buff).await);
                    return Err(RaknetError::IncompatibleProtocolVersion(
                        version.server_protocol,
                        RAKNET_PROTOCOL_VERSION,
                    ));
                }
                AlreadyConnected::ID => {
                    let _alredy_connected =
                        unwrap_or_continue!(decode::<AlreadyConnected>(buff).await);
                    return Err(RaknetError::AlreadyConnected(remote));
                }
                _ => {}
            }
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
}
