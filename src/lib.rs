pub mod packets;
pub mod raknet;
pub mod reader;
pub mod writer;
pub mod packet;
pub mod connection;

#[cfg(test)]
mod tests {
    use std::net::{SocketAddr,Ipv4Addr,IpAddr};
    use crate::raknet::Server;

    #[tokio::test]
    async fn it_works() {
        Server::new(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)),19132)).await;
    }
}