pub mod connection;
pub mod packet;
pub mod packets;
pub mod raknet;
pub mod reader;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::raknet::{RaknetEvent, Server};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[tokio::test]
    async fn it_works() {
        let server = Server::new(
            SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                19132,
            ),
        "MCPE;Dedicated Server;390;1.14.60;0;10;13253860892328930865;Bedrock level;Survival;1;19132;19133;".to_owned()
        ).await;
        server.listen().await;
        for _ in 0..100{
            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
            let events = server.recv().await.unwrap();
            for event in events {
                match event {
                    RaknetEvent::Connected(addr, guid) => {
                        println!("connected {} {}", addr, &guid)
                    }
                    RaknetEvent::Disconnected(addr, guid) => {
                        println!("disconnected {} {}", addr, &guid)
                    }
                    _ => {}
                }
            }
        }
    }
}
