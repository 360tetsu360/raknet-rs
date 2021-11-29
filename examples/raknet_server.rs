use raknet::raknet::{RaknetEvent, Server};
use std::net::SocketAddr;
use tokio;

#[tokio::main]
async fn main() {
    let remote_addr: SocketAddr = "127.0.0.1:19132".parse().expect("could not parse addr");
    let server = Server::new(
            remote_addr,
        "MCPE;§5raknet rs;390;1.17.42;0;10;13253860892328930865;Bedrock level;Survival;1;19132;19133;".to_owned()
        ).await;
    server.listen();
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        let events = server.recv().await.unwrap();
        for event in events {
            match event {
                RaknetEvent::Connected(addr, guid) => {
                    println!("connected {} {}", addr, &guid)
                }
                RaknetEvent::Disconnected(addr, guid) => {
                    println!("disconnected {} {}", addr, &guid)
                }
                RaknetEvent::Packet(_packet) => {
                    //do something
                }
                _ => {}
            }
        }
    }
}
