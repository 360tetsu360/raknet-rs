use std::{collections::HashMap, io::Result, net::SocketAddr};

use crate::packets::{frame::Frame, Reliability};
pub struct ACKQueue {
    pub packets: Vec<(u32, u32)>, //min max
    pub missing: Vec<u32>,
}

impl Default for ACKQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl ACKQueue {
    pub fn new() -> Self {
        Self {
            packets: vec![],
            missing: vec![],
        }
    }
    pub fn add(&mut self, sequence: u32) {
        let mut added = false;
        if self.missing.contains(&sequence) {
            let index = self.missing.iter().position(|x| *x == sequence).unwrap();
            self.missing.remove(index);
        }
        for (_lowest, highest) in self.packets.iter_mut() {
            if *highest + 1 == sequence {
                *highest += 1;
                added = true;
            }
        }
        if !added {
            self.packets.sort_unstable();
            if !self.packets.is_empty() && sequence > self.packets.last().unwrap().1 {
                for num in self.packets.last().unwrap().1 + 1..sequence {
                    self.missing.push(num);
                }
            }
            self.packets.push((sequence, sequence));
        }
        self.clean();
    }
    pub fn clean(&mut self) {
        self.packets.sort_unstable();
        let clone = self.packets.clone();
        for x in 0..clone.len() {
            if x + 1 != clone.len() && clone[x].1 + 1 == clone[x + 1].0 {
                self.packets[x + 1].0 = self.packets[x].0;
                self.packets.remove(x);
            }
        }
    }

    pub fn get_send_able_and_clear(&mut self) -> Vec<(u32, u32)> {
        let ret = self.packets.clone();
        self.packets.clear();
        ret
    }
    pub fn get_missing(&self) -> Vec<u32> {
        self.missing.clone()
    }
    pub fn get_missing_len(&self) -> usize {
        self.missing.len()
    }
}

#[derive(Clone)]
pub struct SplitPacket {
    pub split_size: u32,
    pub data: HashMap<u32, Vec<u8>>,
    pub reliability: Reliability,
    pub message_index: u32,
    pub order_index: u32,
    full: bool,
}
impl SplitPacket {
    pub fn new(
        split_size: u32,
        message_index: u32,
        order_index: u32,
        reliability: Reliability,
    ) -> Self {
        Self {
            split_size,
            data: HashMap::new(),
            reliability,
            message_index,
            order_index,
            full: false,
        }
    }
    pub fn add(&mut self, index: u32, payload: &[u8]) {
        if index < self.split_size {
            self.data.insert(index, payload.to_vec());
            if self.data.len() as u32 == self.split_size {
                self.full = true;
            }
        }
    }
    pub fn is_full(&self) -> bool {
        self.full
    }
    pub fn get_all(&mut self) -> Vec<u8> {
        let mut ret = vec![];
        for index in 0..self.split_size {
            ret.append(self.data.get_mut(&index).unwrap())
        }
        ret
    }
    pub fn get_frame(&mut self) -> Result<Frame> {
        let buff: Vec<u8> = self.get_all();
        let mut frame = Frame::new(self.reliability.clone(), &buff);
        frame.order_index = self.order_index;
        Ok(frame)
    }
}

pub struct SplitPacketQueue {
    pub pool: HashMap<u16, SplitPacket>,
    delete: Vec<u16>,
}
impl Default for SplitPacketQueue {
    fn default() -> Self {
        Self::new()
    }
}
impl SplitPacketQueue {
    pub fn new() -> Self {
        Self {
            pool: HashMap::new(),
            delete: vec![],
        }
    }
    pub fn add(&mut self, frame: &Frame) {
        if let std::collections::hash_map::Entry::Vacant(e) = self.pool.entry(frame.split_id) {
            let mut new_split = SplitPacket::new(
                frame.split_count,
                frame.message_index,
                frame.sequence_index,
                frame.reliability.clone(),
            );
            new_split.order_index = frame.order_index;
            e.insert(new_split);
        }
        self.pool
            .get_mut(&frame.split_id)
            .unwrap()
            .add(frame.split_index, &frame.data);
    }
    pub fn get_and_clear(&mut self) -> Vec<SplitPacket> {
        let mut ret = vec![];
        for water in self.pool.iter() {
            if water.1.is_full() {
                self.delete.push(*water.0);
                ret.push(water.1.clone());
            }
        }
        for delete in self.delete.iter() {
            self.pool.remove(delete);
        }
        ret
    }
}

#[derive(Clone)]
pub struct RaknetPacket {
    pub address: SocketAddr,
    pub guid: u64,
    pub length: usize,
    pub data: Vec<u8>,
}

impl RaknetPacket {
    pub fn new(address: SocketAddr, guid: u64, data: Vec<u8>) -> Self {
        Self {
            address,
            guid,
            length: data.len(),
            data,
        }
    }
}
