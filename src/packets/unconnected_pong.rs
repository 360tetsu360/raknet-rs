use crate::reader::{Endian, Reader};
use crate::writer::Writer;
use std::io::Result;

use crate::packets::Packet;

#[derive(Clone)]
pub struct UnconnectedPong {
    pub time: i64,
    pub guid: u64,
    _magic: bool,
    pub motd: String,
}

impl UnconnectedPong {
    pub fn new(time: i64, guid: u64, motd: String) -> Self {
        Self {
            time,
            guid,
            _magic: true,
            motd,
        }
    }
}

use async_trait::async_trait;

#[async_trait]
impl Packet for UnconnectedPong {
    const ID: u8 = 0x1c;
    async fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self {
            time: cursor.read_i64(Endian::Big).await?,
            guid: cursor.read_u64(Endian::Big).await?,
            _magic: cursor.read_magic().await?,
            motd: cursor.read_string().await?.to_owned(),
        })
    }
    async fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_i64(self.time, Endian::Big).await?;
        cursor.write_u64(self.guid, Endian::Big).await?;
        cursor.write_magic().await?;
        cursor.write_string(&self.motd).await?;
        Ok(cursor.get_raw_payload())
    }
}
