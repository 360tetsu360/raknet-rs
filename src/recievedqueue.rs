use std::collections::HashMap;

use crate::packets::frame::Frame;

pub struct RecievdQueue {
    min: u32,
    max: u32,
    packet_queue: HashMap<u32, Vec<u8>>,
}

impl RecievdQueue {
    pub fn new() -> Self {
        Self {
            min: 0,
            max: 0,
            packet_queue: HashMap::new(),
        }
    }
    pub fn add(&mut self, frame: Frame) {
        if frame.order_index < self.min {
            return;
        }
        if self.packet_queue.contains_key(&frame.order_index) {
            return;
        }
        if frame.order_index >= self.max {
            self.max = frame.order_index + 1
        }
        self.packet_queue.insert(frame.order_index, frame.data);
    }
    pub fn get_all(&mut self) -> Vec<Vec<u8>> {
        let mut ret = vec![];
        let mut index = self.min;
        for o in self.min..self.max {
            if self.packet_queue.contains_key(&o) {
                ret.push(self.packet_queue.get(&o).unwrap().clone());
            } else {
                break;
            }
            self.packet_queue.remove(&o);
            index += 1;
        }
        self.min = index;
        ret
    }
}
