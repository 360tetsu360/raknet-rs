use raknet::raknet::{Client, RaknetEvent};
use std::net::ToSocketAddrs;

async fn client() {
    let mut remote = "127.0.0.1:19132".to_socket_addrs().unwrap();
    //let remote: SocketAddr = "hivebedrock.network".parse().expect("could not parse addr");
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
                    client.send(&[0xffu8; 4096]).await.unwrap();
                }
                RaknetEvent::Disconnected(addr, guid, _reason) => {
                    println!("disconnected {} {}", addr, &guid);
                    disconnected = true;
                    break;
                }
                RaknetEvent::Packet(_packet) => {
                    client.disconnect().await;
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
    tokio::spawn(async move {
        client().await;
    });
    client().await;
}
