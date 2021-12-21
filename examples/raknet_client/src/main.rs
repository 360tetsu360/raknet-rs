use raknet::{Client, RaknetEvent};
use std::net::ToSocketAddrs;

async fn client() {
    let mut remote = "127.0.0.1:19132".to_socket_addrs().unwrap();
    let mut client = Client::new(remote.next().unwrap(), true);
    client.listen().await;
    client.connect().await;
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        let events = client.recv().await.unwrap();
        let mut disconnected = false;
        for event in events {
            match event {
                RaknetEvent::Connected(addr, guid) => {
                    println!("connected {} {}", addr, &guid);
                }
                RaknetEvent::Disconnected(addr, guid, _reason) => {
                    println!("disconnected {} {}", addr, &guid);
                    disconnected = true;
                    break;
                }
                RaknetEvent::Packet(packet) => {
                    match packet.data[0] {
                        0xfe => {
                            println!("{:?}",&packet.data[1..])
                        },
                        _ => {}
                    }
                }
                RaknetEvent::Error(addr, error) => {
                    eprintln!("{} {}", addr, error);
                    disconnected = true;
                }
            }
        }
        if disconnected {
            break;
        }
    }
}

#[tokio::main]
async fn main() {
    client().await;
}