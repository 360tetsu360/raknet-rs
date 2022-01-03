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

use async_trait::async_trait;

#[async_trait]
impl Packet for OpenConnectionReply1 {
    const ID: u8 = 0x6;
    async fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self {
            _magic: cursor.read_magic().await?,
            guid: cursor.read_u64(Endian::Big).await?,
            use_encryption: cursor.read_u8().await?,
            mtu_size: cursor.read_u16(Endian::Big).await?,
        })
    }
    async fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_magic().await?;
        cursor.write_u64(self.guid, Endian::Big).await?;
        cursor.write_u8(self.use_encryption).await?;
        cursor.write_u16(self.mtu_size, Endian::Big).await?;

        Ok(cursor.get_raw_payload())
    }
}
