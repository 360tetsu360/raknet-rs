use byteorder::{BigEndian, LittleEndian, NativeEndian, ReadBytesExt};
use std::{
    convert::TryInto,
    io::{Cursor, ErrorKind, Read, Result},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    str,
};

pub enum Endian {
    Big,
    Little,
    Native,
}

#[derive(Clone)]
pub struct Reader<'a> {
    cursor: Cursor<&'a [u8]>,
    strbuf: Vec<u8>,
}

impl<'a> Reader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(buf),
            strbuf: Vec::with_capacity(0),
        }
    }
    pub fn read_u8(&mut self) -> Result<u8> {
        self.cursor.read_u8()
    }

    pub fn read_u16(&mut self, n: Endian) -> Result<u16> {
        match n {
            Endian::Big => self.cursor.read_u16::<BigEndian>(),
            Endian::Little => self.cursor.read_u16::<LittleEndian>(),
            Endian::Native => self.cursor.read_u16::<NativeEndian>(),
        }
    }

    pub fn read_u32(&mut self, n: Endian) -> Result<u32> {
        match n {
            Endian::Big => self.cursor.read_u32::<BigEndian>(),
            Endian::Little => self.cursor.read_u32::<LittleEndian>(),
            Endian::Native => self.cursor.read_u32::<NativeEndian>(),
        }
    }

    pub fn read_u64(&mut self, n: Endian) -> Result<u64> {
        match n {
            Endian::Big => self.cursor.read_u64::<BigEndian>(),
            Endian::Little => self.cursor.read_u64::<LittleEndian>(),
            Endian::Native => self.cursor.read_u64::<NativeEndian>(),
        }
    }

    pub fn read_u24le(&mut self, n: Endian) -> Result<u32> {
        match n {
            Endian::Big => self.cursor.read_u24::<BigEndian>(),
            Endian::Little => self.cursor.read_u24::<LittleEndian>(),
            Endian::Native => self.cursor.read_u24::<NativeEndian>(),
        }
    }

    pub fn read_string(&'a mut self) -> &'a str {
        let size = self.clone().read_u16(Endian::Big).unwrap();
        self.strbuf.resize(size.into(), 0);
        assert!(self.cursor.read(&mut self.strbuf).unwrap() == size.into());
        str::from_utf8(&self.strbuf).unwrap()
    }
    pub fn read_magic(&mut self) -> Result<[u8; 16]> {
        let mut magic = [0; 16];
        self.cursor
            .read(&mut magic)
            .expect("Unable to read magic bytes");
        let offline_magic = [
            0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34,
            0x56, 0x78,
        ];
        if magic == offline_magic {
            Ok(magic)
        } else {
            Err(std::io::Error::new(
                ErrorKind::Other,
                "Unable to read magic bytes",
            ))
        }
    }
    pub fn read_address(&mut self) -> Result<SocketAddr> {
        let ip_ver = self.read_u8()?;

        if ip_ver == 4 {
            let ip = Ipv4Addr::new(
                0xff - self.read_u8()?,
                0xff - self.read_u8()?,
                0xff - self.read_u8()?,
                0xff - self.read_u8()?,
            );
            let port = self.cursor.read_u16::<BigEndian>()?;
            Ok(SocketAddr::new(IpAddr::V4(ip), port))
        } else {
            self.next(2);
            let port = self.cursor.read_u16::<LittleEndian>()?;
            self.next(4);
            let mut addr_buf = [0; 16];

            self.cursor
                .read(&mut addr_buf)
                .expect("Unable to read ipv6 address bytes");
            self.next(4);
            Ok(SocketAddr::new(
                IpAddr::V6(Ipv6Addr::new(
                    u16::from_be_bytes((&addr_buf[..1]).try_into().unwrap()),
                    u16::from_be_bytes((&addr_buf[2..3]).try_into().unwrap()),
                    u16::from_be_bytes((&addr_buf[4..5]).try_into().unwrap()),
                    u16::from_be_bytes((&addr_buf[6..7]).try_into().unwrap()),
                    u16::from_be_bytes((&addr_buf[8..9]).try_into().unwrap()),
                    u16::from_be_bytes((&addr_buf[10..11]).try_into().unwrap()),
                    u16::from_be_bytes((&addr_buf[12..13]).try_into().unwrap()),
                    u16::from_be_bytes((&addr_buf[14..15]).try_into().unwrap()),
                )),
                port,
            ))
        } //IPv6 address = 128bit = u8 * 16
    }
    pub fn next(&mut self, n: u64) {
        self.cursor.set_position(self.cursor.position() + n);
    }

    pub fn pos(&self) -> u64 {
        self.cursor.position()
    }
}
