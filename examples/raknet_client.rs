use raknet::raknet::{Client, RaknetEvent};
use std::net::ToSocketAddrs;
use tokio;

#[tokio::main]
async fn main() {
    let mut remote = "127.0.0.1:19132".to_socket_addrs().unwrap();
    //let remote: SocketAddr = "hivebedrock.network".parse().expect("could not parse addr");
    let mut client = Client::new(remote.next().unwrap(), true);
    client.listen().await;
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
