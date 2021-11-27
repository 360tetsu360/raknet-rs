use crate::reader::Reader;
use crate::writer::Writer;
use std::{convert::TryInto, io::Result};

pub struct OpenConnectionRequest1 {
    _magic: bool,
    pub protocol_version: u8,
    pub mtu_size: u16, //[u8;mtusize]
}

impl OpenConnectionRequest1 {
    pub fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self {
            _magic: cursor.read_magic()?,
            protocol_version: cursor.read_u8()?,
            mtu_size: (payload.len() + 29).try_into().unwrap(),
        })
    }
    pub fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_magic()?;
        cursor.write_u8(self.protocol_version)?;
        cursor.write(
            vec![0; (self.mtu_size as usize) - (cursor.pos() as usize + 28) - 1].as_slice(),
        )?;

        Ok(cursor.get_raw_payload())
    }
}
