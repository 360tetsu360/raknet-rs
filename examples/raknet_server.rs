use raknet::{RaknetEvent, Server};
use std::net::SocketAddr;
#[tokio::main]
async fn main() {
    let local: SocketAddr = "127.0.0.1:19132".parse().expect("could not parse addr");
    let mut server = Server::new(
            local,
        "MCPE;§5raknet rs;390;1.17.42;0;10;13253860892328930865;Bedrock level;Survival;1;19132;19133;".to_owned()
        );
    server.listen().await;
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        let events = server.recv().await.unwrap();
        for event in events {
            match event {
                RaknetEvent::Connected(addr, guid) => {
                    println!("connected {} {}", addr, &guid)
                }
                RaknetEvent::Disconnected(addr, guid, _reason) => {
                    println!("disconnected {} {}", addr, &guid)
                }
                RaknetEvent::Packet(packet) => {
                    println!("{}", packet.data[0]);
                }
                _ => {}
            }
        }
    }
}
