use crate::packets::Packet;
use std::io::Result;

#[derive(Clone)]
pub struct Disconnected;

use async_trait::async_trait;

#[async_trait]
impl Packet for Disconnected {
    const ID: u8 = 0x15;
    async fn read(_payload: &[u8]) -> Result<Self> {
        Ok(Self)
    }
    async fn write(&self) -> Result<Vec<u8>> {
        Ok(vec![])
    }
}
