use crate::packets::Packet;
use crate::reader::{Endian, Reader};
use crate::writer::Writer;
use std::{io::Result, net::SocketAddr};

#[derive(Clone)]
pub struct ConnectionRequestAccepted {
    pub client_address: SocketAddr,
    pub system_index: u16,
    pub request_timestamp: i64,
    pub accepted_timestamp: i64,
}

impl ConnectionRequestAccepted {
    pub fn new(
        client_address: SocketAddr,
        request_timestamp: i64,
        accepted_timestamp: i64,
    ) -> Self {
        Self {
            client_address,
            system_index: 0,
            request_timestamp,
            accepted_timestamp,
        }
    }
}

impl Packet for ConnectionRequestAccepted {
    const ID: u8 = 0x10;
    fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self {
            client_address: cursor.read_address()?,
            system_index: cursor.read_u16(Endian::Big)?,
            request_timestamp: {
                cursor.next(((payload.len() - 16) - cursor.pos() as usize) as u64);
                cursor.read_i64(Endian::Big)?
            },
            accepted_timestamp: cursor.read_i64(Endian::Big)?,
        })
    }
    fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_address(self.client_address)?;
        cursor.write_u16(self.system_index, Endian::Big)?;
        for _ in 0..10 {
            cursor.write_u8(0x06)?;
        }
        cursor.write_i64(self.request_timestamp, Endian::Big)?;
        cursor.write_i64(self.accepted_timestamp, Endian::Big)?;
        Ok(cursor.get_raw_payload())
    }
}
