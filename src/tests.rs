use crate::packet::ACKQueue;
use crate::raknet::{RaknetEvent, Server};
use std::net::SocketAddr;
#[tokio::test]
async fn it_works() {
    let remote_addr: SocketAddr = "127.0.0.1:19132".parse().expect("could not parse addr");
    let server = Server::new(
            remote_addr,
        "MCPE;§5raknet rs;390;1.17.42;0;10;13253860892328930865;Bedrock level;Survival;1;19132;19133;".to_owned()
        ).await;
    server.listen();
    for _ in 0..0 {
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        let events = server.recv().await.unwrap();
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

#[test]
fn ack_queue() {
    let mut y = ACKQueue::new();
    for x in 0..10 {
        y.add(x);
    }
    for x in 11..20 {
        y.add(x);
    }
    //y.add(10);
    let z = y.get_send_able_and_clear();
    println!("{{");
    for a in z {
        println!("  ({},{}),", a.0, a.1);
    }
    println!("}}");
    y.add(10);
    let z = y.get_send_able_and_clear();
    println!("{{");
    for a in z {
        println!("  ({},{}),", a.0, a.1);
    }
    println!("}}");
}
