use crate::packet::ACKQueue;
use crate::raknet::{Ping, RaknetEvent, Server};
use crate::reader::{Endian, Reader};
use crate::writer::Writer;
use std::cmp::Ordering;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs};
#[tokio::test]
async fn server() {
    let remote_addr: SocketAddr = "127.0.0.1:19132".parse().expect("could not parse addr");
    let mut server = Server::new(
            remote_addr,
        "MCPE;§5raknet rs;390;1.17.42;0;10;13253860892328930865;Bedrock level;Survival;1;19132;19133;".to_owned()
        );
    server.listen().await;
    for _ in 0..0 {
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        let events = server.recv().await.unwrap();
        for event in events {
            match event {
                RaknetEvent::Connected(addr, guid) => {
                    println!("connected {} {}", addr, &guid)
                }
                RaknetEvent::Disconnected(addr, guid, _reason) => {
                    println!("disconnected {} {}", addr, &guid)
                }
                RaknetEvent::Packet(_packet) => {}
                _ => {}
            }
        }
    }
}

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

const BUFFER: [u8; 8] = [0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7];
const TEST_U8: u8 = 0xff;
const TEST_U16: u16 = 0xffff;
const TEST_U24: u32 = 0xffffff;
const TEST_U32: u32 = 0xffffffff;
const TEST_U64: u64 = 0xffffffffffffffff;
const TEST_I64: i64 = 0x7fffffffffffffff;

#[test]
fn reader_writer() {
    let test_address: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 19132);
    let test_string = "Hello world";
    let mut cursor = Writer::new(vec![]);

    cursor.write(&BUFFER).unwrap();
    cursor.write_u8(TEST_U8).unwrap();
    cursor.write_u16(TEST_U16, Endian::Big).unwrap();
    cursor.write_u16(TEST_U16, Endian::Little).unwrap();
    cursor.write_u16(TEST_U16, Endian::Native).unwrap();
    cursor.write_u24(TEST_U24, Endian::Big).unwrap();
    cursor.write_u24(TEST_U24, Endian::Little).unwrap();
    cursor.write_u24(TEST_U24, Endian::Native).unwrap();
    cursor.write_u32(TEST_U32, Endian::Big).unwrap();
    cursor.write_u32(TEST_U32, Endian::Little).unwrap();
    cursor.write_u32(TEST_U32, Endian::Native).unwrap();
    cursor.write_u64(TEST_U64, Endian::Big).unwrap();
    cursor.write_u64(TEST_U64, Endian::Little).unwrap();
    cursor.write_u64(TEST_U64, Endian::Native).unwrap();
    cursor.write_i64(TEST_I64, Endian::Big).unwrap();
    cursor.write_i64(TEST_I64, Endian::Little).unwrap();
    cursor.write_i64(TEST_I64, Endian::Native).unwrap();

    cursor.write_address(test_address).unwrap();
    cursor.write_magic().unwrap();
    cursor.write_string(test_string).unwrap();

    let buff = cursor.get_raw_payload();

    let mut cursor = Reader::new(&buff);
    let mut buff = [0u8; 8];
    cursor.read(&mut buff).unwrap();
    assert_eq!(buff, BUFFER);
    assert_eq!(cursor.read_u8().unwrap(), TEST_U8);
    assert_eq!(cursor.read_u16(Endian::Big).unwrap(), TEST_U16);
    assert_eq!(cursor.read_u16(Endian::Little).unwrap(), TEST_U16);
    assert_eq!(cursor.read_u16(Endian::Native).unwrap(), TEST_U16);
    assert_eq!(cursor.read_u24(Endian::Big).unwrap(), TEST_U24);
    assert_eq!(cursor.read_u24(Endian::Little).unwrap(), TEST_U24);
    assert_eq!(cursor.read_u24(Endian::Native).unwrap(), TEST_U24);
    assert_eq!(cursor.read_u32(Endian::Big).unwrap(), TEST_U32);
    assert_eq!(cursor.read_u32(Endian::Little).unwrap(), TEST_U32);
    assert_eq!(cursor.read_u32(Endian::Native).unwrap(), TEST_U32);
    assert_eq!(cursor.read_u64(Endian::Big).unwrap(), TEST_U64);
    assert_eq!(cursor.read_u64(Endian::Little).unwrap(), TEST_U64);
    assert_eq!(cursor.read_u64(Endian::Native).unwrap(), TEST_U64);
    assert_eq!(cursor.read_i64(Endian::Big).unwrap(), TEST_I64);
    assert_eq!(cursor.read_i64(Endian::Little).unwrap(), TEST_I64);
    assert_eq!(cursor.read_i64(Endian::Native).unwrap(), TEST_I64);

    assert_eq!(cursor.read_address().unwrap(), test_address);
    assert_eq!(cursor.read_magic().unwrap(), true);
    assert_eq!(cursor.read_string().cmp(test_string), Ordering::Equal);
}
