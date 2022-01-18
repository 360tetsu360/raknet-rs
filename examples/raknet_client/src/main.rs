use raknet::{Client, RaknetHandler};
use std::net::{ToSocketAddrs, SocketAddr};

struct S;
impl RaknetHandler for S {
    fn on_connect(&mut self, addr: SocketAddr, _guid: u64) {
        dbg!(addr);
    }

    fn on_disconnect(&mut self, addr: SocketAddr, _guid: u64, _reason: raknet::DisconnectReason) {
        dbg!(addr);
    }

    fn on_message(&mut self, packet: raknet::packet::RaknetPacket) {
        dbg!(packet.address);
    }

    fn raknet_error(&mut self, addr: SocketAddr, _e: raknet::RaknetError) {
        dbg!(addr);
    }
}



async fn client() {
    let handler = S{};
    let mut remote = "127.0.0.1:19132".to_socket_addrs().unwrap();
    let mut client = Client::new(remote.next().unwrap(), true, handler).await.unwrap();
    client.connect().await.unwrap();
    client.listen().await;
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}

#[tokio::main]
async fn main() {
    client().await;
}
