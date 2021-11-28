use crate::reader::{Endian, Reader};
use crate::writer::Writer;
use std::io::Result;

use crate::packets::Packet;

#[derive(Clone)]
pub struct UnconnectedPong {
    pub time: u64,
    pub guid: u64,
    _magic: bool,
    pub motd: String,
}

impl UnconnectedPong {
    pub fn new(time: u64, guid: u64, motd: String) -> Self {
        Self {
            time,
            guid,
            _magic: true,
            motd,
        }
    }
}

impl Packet for UnconnectedPong {
    const ID: u8 = 0x1c;
    fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self {
            time: cursor.read_u64(Endian::Big)?,
            guid: cursor.read_u64(Endian::Big)?,
            _magic: cursor.read_magic()?,
            motd: cursor.read_string().to_owned(),
        })
    }
    fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_u64(self.time, Endian::Big)?;
        cursor.write_u64(self.guid, Endian::Big)?;
        cursor.write_magic()?;
        cursor.write_string(&self.motd)?;
        Ok(cursor.get_raw_payload())
    }
}
