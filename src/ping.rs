use std::{io::Result, net::SocketAddr};

use tokio::net::UdpSocket;

use crate::packets::*;

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
        let payload = encode(unconnected_ping).await?;
        let mut ret = String::new();
        self.socket.send_to(&payload, address).await?;
        let mut v = [0u8; 1500];

        let timeout = tokio::time::timeout(
            std::time::Duration::from_secs(1),
            self.socket.recv_from(&mut v),
        );
        let (size, _source) = timeout.await??;
        let buff = &v[..size];
        if buff[0] == UnconnectedPong::ID {
            let pong = decode::<UnconnectedPong>(buff).await?;
            ret = pong.motd;
        }

        Ok(ret)
    }
}
