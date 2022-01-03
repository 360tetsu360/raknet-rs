use crate::packets::Packet;
use crate::reader::{Endian, Reader};
use crate::writer::Writer;
use std::{io::Result, net::SocketAddr};

#[derive(Clone)]
pub struct OpenConnectionReply2 {
    _magic: bool,
    pub guid: u64,
    pub address: SocketAddr,
    pub mtu: u16,
    pub encryption_enabled: u8,
}

impl OpenConnectionReply2 {
    pub fn new(guid: u64, address: SocketAddr, mtu: u16, encryption_enabled: bool) -> Self {
        Self {
            _magic: true,
            guid,
            address,
            mtu,
            encryption_enabled: encryption_enabled as u8,
        }
    }
}

use async_trait::async_trait;

#[async_trait]
impl Packet for OpenConnectionReply2 {
    const ID: u8 = 0x8;
    async fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        Ok(Self {
            _magic: cursor.read_magic().await?,
            guid: cursor.read_u64(Endian::Big).await?,
            address: cursor.read_address().await?,
            mtu: cursor.read_u16(Endian::Big).await?,
            encryption_enabled: cursor.read_u8().await?,
        })
    }
    async fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_magic().await?;
        cursor.write_u64(self.guid, Endian::Big).await?;
        cursor.write_address(self.address).await?;
        cursor.write_u16(self.mtu, Endian::Big).await?;
        cursor.write_u8(self.encryption_enabled).await?;

        Ok(cursor.get_raw_payload())
    }
}
