use crate::reader::{Reader,Endian};
use crate::writer::Writer;
use std::{
    io::Result,
    net::{SocketAddr}
};

pub struct OpenConnectionReply2 {
    _magic : [u8;16],
    pub guid:u64,
    pub address: SocketAddr,
    pub mtu : u16,
    pub encryption_enabled : u8,
}

impl OpenConnectionReply2 {
    pub fn read(payload : &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self{
            _magic:cursor.read_magic()?,
            guid:cursor.read_u64(Endian::Big)?,
            address:cursor.read_address()?,
            mtu:cursor.read_u16(Endian::Big)?,
            encryption_enabled:cursor.read_u8()?
        })
    }
    pub fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_magic()?;
        cursor.write_u64(self.guid,Endian::Big)?;
        cursor.write_address(self.address)?;
        cursor.write_u16(self.mtu,Endian::Big)?;
        cursor.write_u8(self.encryption_enabled)?;

        Ok(cursor.get_raw_payload())
    }
}