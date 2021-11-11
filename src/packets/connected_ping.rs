use crate::reader::{Reader,Endian};
use crate::writer::Writer;
use std::{
    io::Result,
};

pub struct ConnectedPing{
    pub client_timestamp : u64,
}

impl ConnectedPing {
    pub fn read(payload : &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self{
            client_timestamp : cursor.read_u64(Endian::Big)?
        })
    }
    pub fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_u64(self.client_timestamp,Endian::Big)?;

        Ok(cursor.get_raw_payload())
    }
}