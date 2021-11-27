use std::io::Result;

use crate::{
    reader::{Endian, Reader},
    writer::Writer,
};

use super::Reliability;
const SPLIT_FLAG: u8 = 0x10;

#[derive(Clone)]
pub struct Frame {
    pub reliability: Reliability,

    pub message_index: u32,
    pub sequence_index: u32,
    pub order_index: u32,

    pub split: bool,
    pub split_count: u32,
    pub split_index: u32,
    pub split_id: u16,
    pub data: Vec<u8>,
}

impl Frame {
    pub fn new(reliability: Reliability, data: &[u8]) -> Self {
        Self {
            reliability,
            message_index: 0,
            sequence_index: 0,
            order_index: 0,
            split: false,
            split_count: 0,
            split_index: 0,
            split_id: 0,
            data: data.to_vec(),
        }
    }
    pub fn decode(cursor: &mut Reader) -> Result<Self> {
        let mut packet = Self {
            reliability: Reliability::new(0)?,

            message_index: 0,
            sequence_index: 0,
            order_index: 0,

            split: false,
            split_count: 0,
            split_index: 0,
            split_id: 0,
            data: vec![],
        };

        let header = cursor.read_u8()?;
        packet.split = (header & SPLIT_FLAG) != 0;
        packet.reliability = Reliability::new((header & 224) >> 5)?;
        let mut packet_length = cursor.read_u16(Endian::Big)?;
        packet_length >>= 3;

        if packet.reliability.reliable() {
            packet.message_index = cursor.read_u24le(Endian::Little)?;
        }

        if packet.reliability.sequenced() {
            packet.sequence_index = cursor.read_u24le(Endian::Little)?;
        }

        if packet.reliability.sequenced_or_ordered() {
            packet.order_index = cursor.read_u24le(Endian::Little)?;
            cursor.next(1);
        }

        if packet.split {
            packet.split_count = cursor.read_u32(Endian::Big)?;
            packet.split_id = cursor.read_u16(Endian::Big)?;
            packet.split_index = cursor.read_u32(Endian::Big)?;
        }
        packet.data = vec![0u8; packet_length as usize];
        cursor
            .read(&mut packet.data)
            .expect("error while reading data");
        Ok(packet)
    }

    pub fn encode(&self, cursor: &mut Writer) -> Result<()> {
        let mut header = self.reliability.to_byte() << 5;
        if self.split {
            header |= SPLIT_FLAG;
        }
        cursor.write_u8(header)?;
        cursor.write_u16((self.data.len() * 8) as u16, Endian::Big)?;
        if self.reliability.reliable() {
            cursor.write_u24le(self.message_index, Endian::Little)?;
        }
        if self.reliability.sequenced() {
            cursor.write_u24le(self.sequence_index, Endian::Little)?;
        }
        if self.reliability.sequenced_or_ordered() {
            cursor.write_u24le(self.order_index, Endian::Little)?;
            cursor.write_u8(0)?;
        }
        if self.split {
            cursor.write_u32(self.split_count, Endian::Big)?;
            cursor.write_u16(self.split_id, Endian::Big)?;
            cursor.write_u32(self.split_index, Endian::Big)?;
        }
        cursor.write(&self.data)?;
        Ok(())
    }
}
