use crate::packets::Packet;
use crate::reader::Reader;
use crate::writer::Writer;
use std::{convert::TryInto, io::Result};

#[derive(Clone)]
pub struct OpenConnectionRequest1 {
    _magic: bool,
    pub protocol_version: u8,
    pub mtu_size: u16, //[u8;mtusize]
}

impl OpenConnectionRequest1 {
    pub fn new(protocol_version: u8, mtu_size: u16) -> Self {
        Self {
            _magic: true,
            protocol_version,
            mtu_size,
        }
    }
}
use async_trait::async_trait;

#[async_trait]
impl Packet for OpenConnectionRequest1 {
    const ID: u8 = 0x5;
    async fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self {
            _magic: cursor.read_magic().await?,
            protocol_version: cursor.read_u8().await?,
            mtu_size: (payload.len() + 29).try_into().unwrap(),
        })
    }
    async fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_magic().await?;
        cursor.write_u8(self.protocol_version).await?;
        cursor
            .write(vec![0; (self.mtu_size as usize) - (cursor.pos() as usize + 28) - 1].as_slice())
            .await?;

        Ok(cursor.get_raw_payload())
    }
}
