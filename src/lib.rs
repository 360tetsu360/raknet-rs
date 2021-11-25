pub mod connection;
pub mod packet;
pub mod packets;
pub mod raknet;
pub mod reader;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::raknet::{RaknetEvent, Server};
    use std::net::SocketAddr;

    #[tokio::test]
    async fn it_works() {
        let remote_addr: SocketAddr = "127.0.0.1:19132".parse().expect("could not parse addr");
        let server = Server::new(
            remote_addr,
        "MCPE;Dedicated Server;390;1.17.42;0;10;13253860892328930865;Bedrock level;Survival;1;19132;19133;".to_owned()
        ).await;
        server.listen().await;
        for _ in 0..0{
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
