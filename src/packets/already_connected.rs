use crate::packets::Packet;
use crate::reader::{Endian, Reader};
use crate::writer::Writer;
use std::io::Result;

#[derive(Clone)]
pub struct AlreadyConnected {
    _magic: bool,
    pub guid: u64,
}

impl AlreadyConnected {
    pub fn new(guid: u64) -> Self {
        Self { _magic: true, guid }
    }
}

impl Packet for AlreadyConnected {
    const ID: u8 = 0x12;
    fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self {
            _magic: cursor.read_magic()?,
            guid: cursor.read_u64(Endian::Big)?,
        })
    }
    fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_magic()?;
        cursor.write_u64(self.guid, Endian::Big)?;
        Ok(cursor.get_raw_payload())
    }
}
