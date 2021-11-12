pub mod connection;
pub mod packet;
pub mod packets;
pub mod raknet;
pub mod reader;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::raknet::Server;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[tokio::test]
    async fn it_works() {
        Server::new(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            19132,
        ))
        .await;
    }
}
