use crate::reader::{Endian, Reader};
use crate::writer::Writer;
use std::io::Result;

pub struct UnconnectedPong {
    pub time: u64,
    pub guid: u64,
    pub magic: [u8; 16],
    pub motd: String,
}

impl UnconnectedPong {
    pub fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self {
            time: cursor.read_u64(Endian::Big)?,
            guid: cursor.read_u64(Endian::Big)?,
            magic: cursor.read_magic()?,
            motd: cursor.read_string().to_owned(),
        })
    }
    pub fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_u64(self.time, Endian::Big)?;
        cursor.write_u64(self.guid, Endian::Big)?;
        cursor.write_magic()?;
        cursor.write_string(&self.motd)?;
        Ok(cursor.get_raw_payload())
    }
}
