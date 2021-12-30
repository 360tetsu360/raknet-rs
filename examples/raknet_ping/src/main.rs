use raknet::Ping;
use std::net::{SocketAddr, ToSocketAddrs};

#[tokio::main]
async fn main() {
    let pinger = Ping::new().await;
    let remote = "hivebedrock.network:19132"
        .to_socket_addrs()
        .unwrap()
        .collect::<Vec<SocketAddr>>();

    let mut result = String::new();
    for addr in remote {
        if let Ok(p) = pinger.ping(addr).await {
            result = p;
            break;
        }
    }
    println!("{:02X?}", result);
}
