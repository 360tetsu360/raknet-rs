use crate::packets::Packet;
use crate::reader::{Endian, Reader};
use crate::writer::Writer;
use std::io::Result;

#[derive(Clone)]
pub struct UnconnectedPing {
    pub time: i64,
    _magic: bool,
    pub guid: u64,
}

impl UnconnectedPing {
    pub fn new(time: i64, guid: u64) -> Self {
        Self {
            time,
            _magic: true,
            guid,
        }
    }
}

use async_trait::async_trait;

#[async_trait]
impl Packet for UnconnectedPing {
    const ID: u8 = 0x01;
    async fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self {
            time: cursor.read_i64(Endian::Big).await?,
            _magic: cursor.read_magic().await?,
            guid: cursor.read_u64(Endian::Big).await?,
        })
    }
    async fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_i64(self.time, Endian::Big).await?;
        cursor.write_magic().await?;
        cursor.write_u64(self.guid, Endian::Big).await?;
        Ok(cursor.get_raw_payload())
    }
}
