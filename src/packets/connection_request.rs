use crate::packets::Packet;
use crate::reader::{Endian, Reader};
use crate::writer::Writer;
use std::io::Result;

#[derive(Clone)]
pub struct ConnectionRequest {
    pub guid: u64,
    pub time: i64,
    pub use_encryption: u8,
}

impl ConnectionRequest {
    pub fn new(guid: u64, time: i64, use_encryption: bool) -> Self {
        Self {
            guid,
            time,
            use_encryption: use_encryption as u8,
        }
    }
}

use async_trait::async_trait;

#[async_trait]
impl Packet for ConnectionRequest {
    const ID: u8 = 0x9;
    async fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self {
            guid: cursor.read_u64(Endian::Big).await?,
            time: cursor.read_i64(Endian::Big).await?,
            use_encryption: cursor.read_u8().await?,
        })
    }
    async fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_u64(self.guid, Endian::Big).await?;
        cursor.write_i64(self.time, Endian::Big).await?;
        cursor.write_u8(self.use_encryption).await?;

        Ok(cursor.get_raw_payload())
    }
}
