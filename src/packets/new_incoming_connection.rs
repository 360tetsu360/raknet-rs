use crate::packets::Packet;
use crate::reader::{Endian, Reader};
use crate::writer::Writer;
use std::{io::Result, net::SocketAddr};

#[derive(Clone)]
pub struct NewIncomingConnection {
    pub server_address: SocketAddr,
    pub request_timestamp: u64,
    pub accepted_timestamp: u64,
}

impl Packet for NewIncomingConnection {
    const ID: u8 = 0x13;
    fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self {
            server_address: cursor.read_address()?,
            request_timestamp: {
                cursor.next(((payload.len() - 16) - cursor.pos() as usize) as u64);
                cursor.read_u64(Endian::Big)?
            },
            accepted_timestamp: cursor.read_u64(Endian::Big)?,
        })
    }
    fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_address(self.server_address)?;
        for _ in 0..10 {
            cursor.write_u8(0x06)?;
        }
        cursor.write_u64(self.request_timestamp, Endian::Big)?;
        cursor.write_u64(self.accepted_timestamp, Endian::Big)?;
        Ok(cursor.get_raw_payload())
    }
}
