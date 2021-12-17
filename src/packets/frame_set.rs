use std::io::Result;

use super::frame::Frame;
use crate::{
    reader::{Endian, Reader},
    writer::Writer,
};

#[derive(Clone)]
pub struct FrameSet {
    pub header: u8,
    pub sequence_number: u32,
    pub datas: Vec<Frame>,
}

impl FrameSet {
    pub fn decode(payload: &[u8]) -> Result<Self> {
        let size = payload.len();
        let mut cursor = Reader::new(payload);
        let mut frame_set = Self {
            header: cursor.read_u8()?,
            sequence_number: cursor.read_u24(Endian::Little)?,
            datas: vec![],
        };
        while cursor.pos() < size as u64 {
            frame_set.datas.push(Frame::decode(&mut cursor)?)
        }

        Ok(frame_set)
    }
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut cursor = Writer::new(vec![]);
        cursor.write_u8(self.header)?;
        cursor.write_u24(self.sequence_number, Endian::Little)?;
        for frame in &self.datas {
            frame.encode(&mut cursor)?;
        }
        Ok(cursor.get_raw_payload())
    }
}
