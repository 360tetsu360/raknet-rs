# About
Pure [Rust](https://www.rust-lang.org/) implementation of [Raknet](http://www.jenkinssoftware.com/) Protocol.

# Example
Client
```rs
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
                            //do something here
                        }
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
```

Server 
```rs
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
                    match packet.data[0] {
                        0xfe => {
                            //do something here
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
```
Ping 
```rs
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
```

