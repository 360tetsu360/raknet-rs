use crate::packets::Packet;
use crate::reader::{Endian, Reader};
use crate::writer::Writer;
use std::io::Result;

#[derive(Clone)]
pub struct ConnectionRequest {
    pub guid: u64,
    pub time: u64,
    pub use_encryption: u8,
}

impl Packet for ConnectionRequest {
    const ID: u8 = 0x9;
    fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self {
            guid: cursor.read_u64(Endian::Big)?,
            time: cursor.read_u64(Endian::Big)?,
            use_encryption: cursor.read_u8()?,
        })
    }
    fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_u64(self.guid, Endian::Big)?;
        cursor.write_u64(self.time, Endian::Big)?;
        cursor.write_u8(self.use_encryption)?;

        Ok(cursor.get_raw_payload())
    }
}
