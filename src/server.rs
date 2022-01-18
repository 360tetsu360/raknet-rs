use rand::random;
use std::{collections::HashMap, io::Result, net::SocketAddr, sync::Arc};
use tokio::{
    net::UdpSocket,
    sync::{mpsc::Receiver, Mutex},
};

use crate::RaknetEvent;
use crate::{connection::Connection, packets::*};
use crate::{macros::*, RaknetHandler};

const RAKNET_PROTOCOL_VERSION: u8 = 0xA;

pub struct Server<T: RaknetHandler + 'static> {
    socket: Option<Arc<UdpSocket>>,
    connection: Arc<Mutex<HashMap<SocketAddr, Arc<Mutex<Connection>>>>>,
    title: Arc<Mutex<String>>,
    connected_clients: Arc<Mutex<Vec<u64>>>,
    receivers: Arc<Mutex<Vec<Receiver<RaknetEvent>>>>,
    pub local_addr: SocketAddr,
    pub id: u64,
    handler: Arc<Mutex<T>>,
}

impl<T: RaknetHandler + 'static> Server<T> {
    pub fn new(address: SocketAddr, title: String, handler: T) -> Self {
        Self {
            socket: None,
            connection: Arc::new(Mutex::new(HashMap::new())),
            id: random::<u64>(),
            title: Arc::new(Mutex::new(title)),
            local_addr: address,
            connected_clients: Arc::new(Mutex::new(vec![])),
            receivers: Arc::new(Mutex::new(vec![])),
            handler: Arc::new(Mutex::new(handler)),
        }
    }

    pub async fn listen(&mut self) -> std::io::Result<()> {
        self.socket = Some(Arc::new(UdpSocket::bind(self.local_addr).await?));
        let socket2 = self.socket.clone().unwrap();
        let connections2 = self.connection.clone();
        let connected_client = self.connected_clients.clone();
        let id = self.id;
        let motd = self.title.clone();
        let receiver = self.receivers.clone();
        tokio::spawn(async move {
            loop {
                let mut v = [0u8; 1500];

                let (size, source) = unwrap_or_return!(socket2.recv_from(&mut v).await);

                if size == 0 {
                    continue;
                }

                let connections3 = connections2.clone();
                let socket3 = socket2.clone();
                let connected_client2 = connected_client.clone();
                let motd2 = motd.clone();
                let receiver2 = receiver.clone();

                tokio::spawn(async move {
                    if !connections3.lock().await.contains_key(&source) {
                        //not connected
                        let buff = &v[..size];
                        match buff[0] {
                            UnconnectedPing::ID => {
                                let p = unwrap_or_return!(decode::<UnconnectedPing>(buff).await);
                                let pong = UnconnectedPong::new(
                                    p.time,
                                    id,
                                    motd2.lock().await.to_string(),
                                );
                                let data = unwrap_or_dbg!(encode(pong).await);
                                unwrap_or_dbg!(socket3.send_to(&data, source).await);
                            }
                            OpenConnectionRequest1::ID => {
                                let p =
                                    unwrap_or_return!(decode::<OpenConnectionRequest1>(buff).await);
                                if p.protocol_version == RAKNET_PROTOCOL_VERSION {
                                    let ocreply1 = OpenConnectionReply1::new(id, false, p.mtu_size);
                                    let data = unwrap_or_dbg!(encode(ocreply1).await);
                                    unwrap_or_dbg!(socket3.send_to(&data, source).await);
                                } else {
                                    let reply = IncompatibleProtocolVersion::new(
                                        RAKNET_PROTOCOL_VERSION,
                                        id,
                                    );
                                    let data = unwrap_or_dbg!(encode(reply).await);
                                    unwrap_or_dbg!(socket3.send_to(&data, source).await);
                                }
                            }
                            OpenConnectionRequest2::ID => {
                                let p =
                                    unwrap_or_return!(decode::<OpenConnectionRequest2>(buff).await);
                                if connected_client2.lock().await.contains(&p.guid) {
                                    let already_connected = AlreadyConnected::new(id);
                                    let data = unwrap_or_dbg!(encode(already_connected).await);
                                    unwrap_or_dbg!(socket3.send_to(&data, source).await);
                                    return;
                                }

                                let ocreply2 = OpenConnectionReply2::new(id, source, p.mtu, false);
                                let data = unwrap_or_dbg!(encode(ocreply2).await);
                                unwrap_or_dbg!(socket3.send_to(&data, source).await);
                                let (s, r) = tokio::sync::mpsc::channel::<RaknetEvent>(10);
                                connections3.lock().await.insert(
                                    source,
                                    Arc::new(Mutex::new(Connection::new(
                                        source,
                                        socket3.clone(),
                                        id,
                                        p.guid,
                                        p.mtu,
                                        s,
                                        crate::connection::RaknetType::Server,
                                    ))),
                                );
                                connected_client2.lock().await.push(p.guid);
                                receiver2.lock().await.push(r);
                                //connected!
                            }
                            _ => {}
                        }
                    } else {
                        connections3
                            .lock()
                            .await
                            .get_mut(&source)
                            .unwrap()
                            .lock()
                            .await
                            .handle(&v[..size])
                            .await;
                    }
                });
            }
        });

        let connections = self.connection.clone();
        let receivers = self.receivers.clone();
        let connected = self.connected_clients.clone();
        let handler = self.handler.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                for conn in connections.lock().await.values() {
                    let conn2 = conn.clone();
                    tokio::spawn(async move {
                        conn2.lock().await.update().await;
                    });
                }

                let mut disconnected_clients = vec![];

                for (index, receiver) in receivers.lock().await.iter_mut().enumerate() {
                    while let Ok(event) = receiver.try_recv() {
                        match event {
                            RaknetEvent::Packet(packet) => {
                                handler.lock().await.on_message(packet);
                            }
                            RaknetEvent::Connected(addr, guid) => {
                                handler.lock().await.on_connect(addr, guid);
                            }
                            RaknetEvent::Disconnected(addr, guid, reason) => {
                                disconnected_clients.push((addr, index));
                                handler.lock().await.on_disconnect(addr, guid, reason);
                            }
                            RaknetEvent::Error(addr, e) => {
                                handler.lock().await.raknet_error(addr, e);
                            }
                        }
                    }
                }
                for addr in disconnected_clients.iter() {
                    connections.lock().await.remove(&addr.0);
                    connected.lock().await.remove(addr.1);
                    receivers.lock().await.remove(addr.1);
                }
                disconnected_clients.clear();
            }
        });

        Ok(())
    }

    pub async fn send_to(&mut self, addr: &SocketAddr, buff: &[u8]) -> Result<()> {
        if !self.connection.lock().await.contains_key(addr) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Not connected",
            ));
        }
        self.connection
            .lock()
            .await
            .get_mut(addr)
            .unwrap()
            .lock()
            .await
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
            .lock()
            .await
            .disconnect();
    }
}
