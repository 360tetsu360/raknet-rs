use crate::reader::{Endian, Reader};
use crate::writer::Writer;
use std::io::Result;
pub struct UnconnectedPing {
    pub time: u64,
    _magic: [u8; 16],
    pub guid: u64,
}

impl UnconnectedPing {
    pub fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self {
            time: cursor.read_u64(Endian::Big)?,
            _magic: cursor.read_magic()?,
            guid: cursor.read_u64(Endian::Big)?,
        })
    }
    pub fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_u64(self.time, Endian::Big)?;
        cursor.write_magic()?;
        cursor.write_u64(self.guid, Endian::Big)?;
        Ok(cursor.get_raw_payload())
    }
}
