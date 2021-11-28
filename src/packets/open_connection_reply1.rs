use crate::packets::Packet;
use crate::reader::{Endian, Reader};
use crate::writer::Writer;
use std::io::Result;

#[derive(Clone)]
pub struct OpenConnectionReply1 {
    _magic: bool,
    pub guid: u64,
    pub use_encryption: u8,
    pub mtu_size: u16,
}

impl OpenConnectionReply1 {
    pub fn new(guid: u64, use_encryption: bool, mtu_size: u16) -> Self {
        Self {
            _magic: true,
            guid,
            use_encryption: use_encryption as u8,
            mtu_size,
        }
    }
}

impl Packet for OpenConnectionReply1 {
    const ID: u8 = 0x6;
    fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self {
            _magic: cursor.read_magic()?,
            guid: cursor.read_u64(Endian::Big)?,
            use_encryption: cursor.read_u8()?,
            mtu_size: cursor.read_u16(Endian::Big)?,
        })
    }
    fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_magic()?;
        cursor.write_u64(self.guid, Endian::Big)?;
        cursor.write_u8(self.use_encryption)?;
        cursor.write_u16(self.mtu_size, Endian::Big)?;

        Ok(cursor.get_raw_payload())
    }
}
