use crate::packets::Packet;
use crate::reader::{Endian::Big, Reader};
use crate::writer::Writer;
use std::{io::Result, net::SocketAddr};

#[derive(Clone)]
pub struct ConnectionRequestAccepted {
    pub client_address: SocketAddr,
    pub system_index: u16,
    pub request_timestamp: i64,
    pub accepted_timestamp: i64,
}

impl ConnectionRequestAccepted {
    pub fn new(
        client_address: SocketAddr,
        request_timestamp: i64,
        accepted_timestamp: i64,
    ) -> Self {
        Self {
            client_address,
            system_index: 0,
            request_timestamp,
            accepted_timestamp,
        }
    }
}

use async_trait::async_trait;

#[async_trait]
impl Packet for ConnectionRequestAccepted {
    const ID: u8 = 0x10;
    async fn read(payload: &[u8]) -> Result<Self> {
        let mut cursor = Reader::new(payload);
        let client_address = cursor.read_address().await?;
        let system_index = cursor.read_u16(Big).await?;
        cursor.next(((payload.len() - 16) - cursor.pos() as usize) as u64);
        let request_timestamp = cursor.read_i64(Big).await?;
        let accepted_timestamp = cursor.read_i64(Big).await?;
        Ok(Self {
            client_address,
            system_index,
            request_timestamp,
            accepted_timestamp,
        })
    }
    async fn write(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_address(self.client_address).await?;
        cursor.write_u16(self.system_index, Big).await?;
        cursor.write(&[0x6u8; 10]).await?;
        cursor.write_i64(self.request_timestamp, Big).await?;
        cursor.write_i64(self.accepted_timestamp, Big).await?;
        Ok(cursor.get_raw_payload())
    }
}
