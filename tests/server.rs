use raknet::reader::{Endian, Reader};
use raknet::writer::Writer;
use raknet::{Client, Ping, RaknetEvent, Server};
use std::cmp::Ordering;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs};

#[tokio::test]
async fn ping() {
    let pinger = Ping::new().await;
    let remote = "mco.mineplex.com:19132"
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();
    let pong = pinger.ping(remote).await.unwrap();
    println!("{}", pong);
}

const BUFFER: [u8; 8] = [0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7];
const TEST_U8: u8 = 0xff;
const TEST_U16: u16 = 0xffff;
const TEST_U24: u32 = 0xffffff;
const TEST_U32: u32 = 0xffffffff;
const TEST_U64: u64 = 0xffffffffffffffff;
const TEST_I64: i64 = 0x7fffffffffffffff;

#[tokio::test]
async fn reader_writer() {
    let test_address: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 19132);
    let test_string = "Hello world";
    let mut cursor = Writer::new(vec![]);

    cursor.write(&BUFFER).await.unwrap();
    cursor.write_u8(TEST_U8).await.unwrap();
    cursor.write_u16(TEST_U16, Endian::Big).await.unwrap();
    cursor.write_u16(TEST_U16, Endian::Little).await.unwrap();
    cursor.write_u24(TEST_U24, Endian::Big).await.unwrap();
    cursor.write_u24(TEST_U24, Endian::Little).await.unwrap();
    cursor.write_u32(TEST_U32, Endian::Big).await.unwrap();
    cursor.write_u32(TEST_U32, Endian::Little).await.unwrap();
    cursor.write_u64(TEST_U64, Endian::Big).await.unwrap();
    cursor.write_u64(TEST_U64, Endian::Little).await.unwrap();
    cursor.write_i64(TEST_I64, Endian::Big).await.unwrap();
    cursor.write_i64(TEST_I64, Endian::Little).await.unwrap();

    cursor.write_address(test_address).await.unwrap();
    cursor.write_magic().await.unwrap();
    cursor.write_string(test_string).await.unwrap();

    let buff = cursor.get_raw_payload();

    let mut cursor = Reader::new(&buff);
    let mut buff = [0u8; 8];
    cursor.read(&mut buff).await.unwrap();
    assert_eq!(buff, BUFFER);
    assert_eq!(cursor.read_u8().await.unwrap(), TEST_U8);
    assert_eq!(cursor.read_u16(Endian::Big).await.unwrap(), TEST_U16);
    assert_eq!(cursor.read_u16(Endian::Little).await.unwrap(), TEST_U16);
    assert_eq!(cursor.read_u24(Endian::Big).await.unwrap(), TEST_U24);
    assert_eq!(cursor.read_u24(Endian::Little).await.unwrap(), TEST_U24);
    assert_eq!(cursor.read_u32(Endian::Big).await.unwrap(), TEST_U32);
    assert_eq!(cursor.read_u32(Endian::Little).await.unwrap(), TEST_U32);
    assert_eq!(cursor.read_u64(Endian::Big).await.unwrap(), TEST_U64);
    assert_eq!(cursor.read_u64(Endian::Little).await.unwrap(), TEST_U64);
    assert_eq!(cursor.read_i64(Endian::Big).await.unwrap(), TEST_I64);
    assert_eq!(cursor.read_i64(Endian::Little).await.unwrap(), TEST_I64);

    assert_eq!(cursor.read_address().await.unwrap(), test_address);
    assert_eq!(cursor.read_magic().await.unwrap(), true);
    assert_eq!(
        cursor.read_string().await.unwrap().cmp(test_string),
        Ordering::Equal
    );
}

#[tokio::test]
async fn server_test() {
    tokio::spawn(async move {
        let local: SocketAddr = "127.0.0.1:19132".parse().expect("could not parse addr");
        let mut server = Server::new(
                local,
            "MCPE;§5raknet rs;390;1.17.42;0;10;13253860892328930865;Bedrock level;Survival;1;19132;19133;".to_owned()
            );
        server.listen().await.unwrap();
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
                        if packet.data[0] == 0x48 {
                            let msg = String::from_utf8_lossy(&packet.data);
                            if msg == "Hello Server!!" {
                                server
                                    .send_to(&packet.address, b"Hello Client!!")
                                    .await
                                    .unwrap();
                            }
                        } else if packet.data[0] == 0xff {
                            server.disconnect(packet.address).await;
                        }
                    }
                    _ => {}
                }
            }
        }
    });
    let pinger = Ping::new().await;
    let remote = "127.0.0.1:19132".to_socket_addrs().unwrap().next().unwrap();
    let pong = pinger.ping(remote).await.unwrap();
    println!("{}", pong);

    let mut client = Client::new(remote, true).await.unwrap();
    client.connect().await.unwrap();
    client.listen().await;
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        let events = client.recv().await.unwrap();
        let mut dissconnected = false;
        for event in events {
            match event {
                RaknetEvent::Connected(addr, guid) => {
                    println!("connected {} {}", addr, &guid);
                    client.send(b"Hello Server!!").await.unwrap();
                }
                RaknetEvent::Disconnected(addr, guid, _reason) => {
                    println!("disconnected {} {}", addr, &guid);
                    dissconnected = true;
                    break;
                }
                RaknetEvent::Packet(packet) => {
                    if packet.data[0] == 0x48 {
                        let msg = String::from_utf8_lossy(&packet.data);
                        if msg == "Hello Client!!" {
                            client.send(&[0xffu8; 4896]).await.unwrap();
                        }
                    }
                }
                _ => {}
            }
        }
        if dissconnected {
            break;
        }
    }
}
