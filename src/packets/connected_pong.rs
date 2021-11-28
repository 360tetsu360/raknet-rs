use crate::packets::Packet;
use crate::reader::{Endian, Reader};
use crate::writer::Writer;
use std::io::Result;

#[derive(Clone)]
pub struct ConnectedPong {
    pub client_timestamp: u64,
    pub server_timestamp: u64,
}

impl ConnectedPong {
    pub fn new(client_timestamp: u64, server_timestamp: u64) -> Self {
        Self {
            client_timestamp,
            server_timestamp,
        }
    }
}

impl Packet for ConnectedPong {
    const ID: u8 = 0x3;
    fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self {
            client_timestamp: cursor.read_u64(Endian::Big)?,
            server_timestamp: cursor.read_u64(Endian::Big)?,
        })
    }
    fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_u64(self.client_timestamp, Endian::Big)?;
        cursor.write_u64(self.server_timestamp, Endian::Big)?;
        Ok(cursor.get_raw_payload())
    }
}
