use raknet::{Client, RaknetEvent, Server};
use std::io::stdin;
use std::net::{SocketAddr, ToSocketAddrs};

async fn server() {
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
                    server.send_to(&packet.address, &packet.data).await.unwrap();
                }
                RaknetEvent::Error(addr, error) => {
                    eprintln!("{} {}", addr, error);
                }
            }
        }
    }
}

async fn client() {
    let mut remote = "127.0.0.1:19132".to_socket_addrs().unwrap();
    let mut client = Client::new(remote.next().unwrap(), true).await;
    client.connect().await.unwrap();
    client.listen().await;
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        let events = client.recv().await.unwrap();
        let mut disconnected = false;
        for event in events {
            match event {
                RaknetEvent::Connected(addr, guid) => {
                    println!("connected {} {}", addr, &guid);
                    //client.send(&[0u8;4096]).await.unwrap();
                    client.send(b"Hello Server!!").await.unwrap();
                }
                RaknetEvent::Disconnected(addr, guid, _reason) => {
                    println!("disconnected {} {}", addr, &guid);
                    disconnected = true;
                    break;
                }
                RaknetEvent::Packet(packet) => {
                    let msg = String::from_utf8_lossy(&packet.data);
                    println!("{}", &msg);
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
    let stdin = stdin();

    println!("Please type in `server` or `client`.");

    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();

    if s.starts_with('s') {
        println!("Starting server..");
        server().await;
    } else {
        println!("Starting client..");
        client().await;
    }
}
