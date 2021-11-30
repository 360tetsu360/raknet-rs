use raknet::raknet::Ping;
use std::net::ToSocketAddrs;
use tokio;

#[tokio::main]
async fn main() {
    let pinger = Ping::new().await;
    let remote = "hivebedrock.network:19132"
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();
    let pong = pinger.ping(remote).await.unwrap();
    println!("{}", pong);
}
