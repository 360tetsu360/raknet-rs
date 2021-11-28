use std::io::Result;

use super::frame::Frame;
use crate::{
    reader::{Endian, Reader},
    writer::Writer,
};

pub struct FrameSet {
    pub sequence_number: u32,
    pub datas: Vec<Frame>,
}

impl FrameSet {
    pub fn new(sequence_number: u32, datas: &[Frame]) -> Self {
        Self {
            sequence_number,
            datas: datas.to_vec(),
        }
    }
    pub fn decode(payload: &[u8]) -> Result<Self> {
        let size = payload.len();
        let mut cursor = Reader::new(payload);
        let sequence_number = cursor.read_u24le(Endian::Little)?;
        let mut frame_set = Self {
            sequence_number,
            datas: vec![],
        };
        while cursor.pos() < size as u64 {
            frame_set.datas.push(Frame::decode(&mut cursor)?)
        }

        Ok(frame_set)
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_u24le(self.sequence_number, Endian::Little)?;
        for frame in &self.datas {
            frame.encode(&mut cursor)?;
        }
        Ok(cursor.get_raw_payload())
    }
}
