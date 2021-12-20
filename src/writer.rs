use crate::reader::Endian;
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use std::{
    io::{Cursor, Result, Write},
    net::{IpAddr, SocketAddr},
    str,
};

#[derive(Clone)]
pub struct Writer {
    cursor: Cursor<Vec<u8>>,
}

impl Writer {
    pub fn new(buf: Vec<u8>) -> Self {
        Self {
            cursor: Cursor::new(buf),
        }
    }
    pub fn write(&mut self, v: &[u8]) -> Result<()> {
        self.cursor.write_all(v)
    }

    pub fn write_u8(&mut self, v: u8) -> Result<()> {
        self.cursor.write_u8(v)
    }

    pub fn write_u16(&mut self, v: u16, n: Endian) -> Result<()> {
        match n {
            Endian::Big => self.cursor.write_u16::<BigEndian>(v),
            Endian::Little => self.cursor.write_u16::<LittleEndian>(v),
        }
    }
    pub fn write_u32(&mut self, v: u32, n: Endian) -> Result<()> {
        match n {
            Endian::Big => self.cursor.write_u32::<BigEndian>(v),
            Endian::Little => self.cursor.write_u32::<LittleEndian>(v),
        }
    }
    pub fn write_u24(&mut self, v: u32, n: Endian) -> Result<()> {
        match n {
            Endian::Big => self.cursor.write_u24::<BigEndian>(v),
            Endian::Little => self.cursor.write_u24::<LittleEndian>(v),
        }
    }

    pub fn write_u64(&mut self, v: u64, n: Endian) -> Result<()> {
        match n {
            Endian::Big => self.cursor.write_u64::<BigEndian>(v),
            Endian::Little => self.cursor.write_u64::<LittleEndian>(v),
        }
    }

    pub fn write_i64(&mut self, v: i64, n: Endian) -> Result<()> {
        match n {
            Endian::Big => self.cursor.write_i64::<BigEndian>(v),
            Endian::Little => self.cursor.write_i64::<LittleEndian>(v),
        }
    }

    pub fn write_string(&mut self, body: &str) -> Result<()> {
        let raw = body.as_bytes();
        self.write_u16(raw.len() as u16, Endian::Big)?;
        self.write(raw)
    }
    pub fn write_magic(&mut self) -> Result<usize> {
        let magic = [
            0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34,
            0x56, 0x78,
        ];
        self.cursor.write(&magic)
    }
    pub fn write_address(&mut self, address: SocketAddr) -> Result<()> {
        if address.is_ipv4() {
            self.cursor.write_u8(0x4)?;
            let ip_bytes = match address.ip() {
                IpAddr::V4(ip) => ip.octets().to_vec(),
                _ => vec![0; 4],
            };

            self.write_u8(0xff - ip_bytes[0])?;
            self.write_u8(0xff - ip_bytes[1])?;
            self.write_u8(0xff - ip_bytes[2])?;
            self.write_u8(0xff - ip_bytes[3])?;
            self.cursor.write_u16::<BigEndian>(address.port())?;
            Ok(())
        } else {
            self.cursor.write_i16::<LittleEndian>(23)?;
            self.cursor.write_u16::<BigEndian>(address.port())?;
            self.cursor.write_i32::<BigEndian>(0)?;
            let ip_bytes = match address.ip() {
                IpAddr::V6(ip) => ip.octets().to_vec(),
                _ => vec![0; 16],
            };
            self.write(&ip_bytes)?;
            self.cursor.write_i32::<BigEndian>(0)?;
            Ok(())
        }
    }

    pub fn get_raw_payload(self) -> Vec<u8> {
        self.cursor.into_inner()
    }

    pub fn pos(&self) -> u64 {
        self.cursor.position()
    }
}
