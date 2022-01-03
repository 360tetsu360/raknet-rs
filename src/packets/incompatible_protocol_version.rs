use crate::packets::Packet;
use crate::reader::{Endian, Reader};
use crate::writer::Writer;
use std::io::Result;

#[derive(Clone)]
pub struct IncompatibleProtocolVersion {
    pub server_protocol: u8,
    _magic: bool,
    pub server_guid: u64,
}

impl IncompatibleProtocolVersion {
    pub fn new(protocol_v: u8, guid: u64) -> Self {
        Self {
            server_protocol: protocol_v,
            _magic: true,
            server_guid: guid,
        }
    }
}

use async_trait::async_trait;

#[async_trait]
impl Packet for IncompatibleProtocolVersion {
    const ID: u8 = 0x19;
    async fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self {
            server_protocol: cursor.read_u8().await?,
            _magic: cursor.read_magic().await?,
            server_guid: cursor.read_u64(Endian::Big).await?,
        })
    }
    async fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_u8(self.server_protocol).await?;
        cursor.write_magic().await?;
        cursor.write_u64(self.server_guid, Endian::Big).await?;

        Ok(cursor.get_raw_payload())
    }
}
