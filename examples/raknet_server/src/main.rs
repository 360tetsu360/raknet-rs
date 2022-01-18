use raknet::{RaknetHandler, Server};
use std::net::SocketAddr;

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

#[tokio::main]
async fn main() {
    let handler = S {};
    let local: SocketAddr = "127.0.0.1:19132".parse().expect("could not parse addr");
    let mut server = Server::new(
            local,
        "MCPE;§5raknet rs;390;1.17.42;0;10;13253860892328930865;Bedrock level;Survival;1;19132;19133;".to_owned(),
        handler
        );
    server.listen().await.unwrap();
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}
