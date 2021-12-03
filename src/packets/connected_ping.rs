use crate::packets::Packet;
use crate::reader::{Endian, Reader};
use crate::writer::Writer;
use std::io::Result;

#[derive(Clone)]
pub struct ConnectedPing {
    pub client_timestamp: u64,
}

impl ConnectedPing {
    pub fn new(time: u64) -> Self {
        Self {
            client_timestamp: time,
        }
    }
}

impl Packet for ConnectedPing {
    const ID: u8 = 0x0;
    fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self {
            client_timestamp: cursor.read_u64(Endian::Big)?,
        })
    }
    fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_u64(self.client_timestamp, Endian::Big)?;

        Ok(cursor.get_raw_payload())
    }
}
