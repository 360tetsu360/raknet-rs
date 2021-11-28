use crate::packets::Packet;
use std::io::Result;

#[derive(Clone)]
pub struct Disconnected;

impl Packet for Disconnected {
    const ID: u8 = 0x15;
    fn read(_payload: &[u8]) -> Result<Self> {
        Ok(Self)
    }
    fn write(&self) -> Result<Vec<u8>> {
        Ok(vec![])
    }
}
