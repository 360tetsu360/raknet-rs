use raknet::raknet::{Client, RaknetEvent};
use std::net::SocketAddr;
use tokio;

#[tokio::main]
async fn main() {
    let remote: SocketAddr = "127.0.0.1:19132".parse().expect("could not parse addr");
    let client = Client::new(remote, false).await;
    client.listen();
    client.connect().await;
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        let events = client.recv().await.unwrap();
        for event in events {
            match event {
                RaknetEvent::Connected(addr, guid) => {
                    println!("connected {} {}", addr, &guid)
                }
                RaknetEvent::Disconnected(addr, guid) => {
                    println!("disconnected {} {}", addr, &guid)
                }
                RaknetEvent::Packet(_packet) => {}
                _ => {}
            }
        }
    }
}
