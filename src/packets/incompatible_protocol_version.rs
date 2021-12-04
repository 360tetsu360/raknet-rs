use crate::packets::Packet;
use crate::reader::{Endian, Reader};
use crate::writer::Writer;
use std::io::Result;

#[derive(Clone)]
pub struct IncompatibleProtocolVersion {
    _magic: bool,
    pub server_protocol: u8,
    pub server_guid: u64,
}

impl IncompatibleProtocolVersion {
    pub fn new(protocol_v: u8, guid: u64) -> Self {
        Self {
            _magic: true,
            server_protocol: protocol_v,
            server_guid: guid,
        }
    }
}

impl Packet for IncompatibleProtocolVersion {
    const ID: u8 = 0x19;
    fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self {
            _magic: cursor.read_magic()?,
            server_protocol: cursor.read_u8()?,
            server_guid: cursor.read_u64(Endian::Big)?,
        })
    }
    fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_magic()?;
        cursor.write_u8(self.server_protocol)?;
        cursor.write_u64(self.server_guid, Endian::Big)?;

        Ok(cursor.get_raw_payload())
    }
}
