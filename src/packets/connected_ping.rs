use crate::packets::Packet;
use crate::reader::{Endian, Reader};
use crate::writer::Writer;
use std::io::Result;

#[derive(Clone)]
pub struct ConnectedPing {
    pub client_timestamp: i64,
}

impl ConnectedPing {
    pub fn new(time: i64) -> Self {
        Self {
            client_timestamp: time,
        }
    }
}

use async_trait::async_trait;

#[async_trait]
impl Packet for ConnectedPing {
    const ID: u8 = 0x0;
    async fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self {
            client_timestamp: cursor.read_i64(Endian::Big).await?,
        })
    }
    async fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_i64(self.client_timestamp, Endian::Big).await?;

        Ok(cursor.get_raw_payload())
    }
}
