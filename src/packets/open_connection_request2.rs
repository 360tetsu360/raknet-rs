use crate::packets::Packet;
use crate::reader::{Endian, Reader};
use crate::writer::Writer;
use std::{io::Result, net::SocketAddr};
#[derive(Clone)]
pub struct OpenConnectionRequest2 {
    _magic: bool,
    pub address: SocketAddr,
    pub mtu: u16,
    pub guid: u64,
}

impl OpenConnectionRequest2 {
    pub fn new(address: SocketAddr, mtu: u16, guid: u64) -> Self {
        Self {
            _magic: true,
            address,
            mtu,
            guid,
        }
    }
}
impl Packet for OpenConnectionRequest2 {
    const ID: u8 = 0x7;
    fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self {
            _magic: cursor.read_magic()?,
            address: cursor.read_address()?,
            mtu: cursor.read_u16(Endian::Big)?,
            guid: cursor.read_u64(Endian::Big)?,
        })
    }

    fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_magic()?;
        cursor.write_address(self.address)?;
        cursor.write_u16(self.mtu, Endian::Big)?;
        cursor.write_u64(self.guid, Endian::Big)?;

        Ok(cursor.get_raw_payload())
    }
}
