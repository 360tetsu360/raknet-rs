use std::io::{Result};

use crate::{reader::{Reader,Endian}};
const SPLIT_FLAG : u8 = 0x10;

//const RELIABILITY_UNRELIABLE : u8 = 0x0;
const UNRELIABLE_SEQUENCED : u8 = 0x1;
const RELIABLE : u8 = 0x2;
const RELIABLE_ORDERED : u8 = 0x3;
const RELIABLE_SEQUENCED : u8 = 0x4;

pub struct Frame {
	pub reliability : u8,

	pub message_index : u32,
	pub sequence_index : u32,
	pub order_index : u32,

	pub split : bool,
	pub split_count : u32,
	pub split_index : u32,
	pub split_id : u16,
    pub data: Vec<u8>,
}

impl Frame {
    pub fn decode(cursor : &mut Reader) -> Result<Self> {
        let mut packet = Self{
            reliability : 0,

            message_index : 0,
            sequence_index : 0,
            order_index : 0,
        
            split : false,
            split_count : 0,
            split_index : 0,
            split_id : 0,
            data: vec![],
        };

        let header = cursor.read_u8()?;
        packet.split = (header&SPLIT_FLAG) != 0;
        packet.reliability = (header & 224) >> 5;
        let mut packet_length = cursor.read_u16(Endian::Big)?;
        packet_length >>= 3;

        if packet.reliable() {
            packet.message_index = cursor.read_u24le(Endian::Little)?;
        }
    
        if packet.sequenced() {
            packet.sequence_index = cursor.read_u24le(Endian::Little)?;
        }
    
        if packet.sequenced_or_ordered() {
            packet.order_index = cursor.read_u24le(Endian::Little)?;
            cursor.next(1);
        }
    
        if packet.split {
            packet.split_count = cursor.read_u32(Endian::Big)?;
            packet.split_id = cursor.read_u16(Endian::Big)?;
            packet.split_index = cursor.read_u32(Endian::Big)?;
        }
        packet.data = vec![0u8;packet_length as usize];
        cursor.read(&mut packet.data).expect("error while reading data");
        Ok(packet)
    }

    fn reliable(&self) -> bool{
        matches!(self.reliability, RELIABLE|
            RELIABLE_ORDERED| 
            RELIABLE_SEQUENCED)
    }
    
    fn sequenced_or_ordered(&self) -> bool {
        matches!(self.reliability, UNRELIABLE_SEQUENCED|
            RELIABLE_ORDERED|
            RELIABLE_SEQUENCED)
    }
    
    fn sequenced(&self) -> bool {
        matches!(self.reliability, UNRELIABLE_SEQUENCED|
            RELIABLE_SEQUENCED)
    }
    
}