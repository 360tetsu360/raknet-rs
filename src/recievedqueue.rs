use std::collections::HashMap;

use crate::{packet::SplitPacketQueue, packets::frame::Frame};

pub struct RecievdQueue {
    min: u32,
    max: u32,
    packet_queue: HashMap<u32, Frame>,
    splits: SplitPacketQueue,
}
impl RecievdQueue {
    pub fn new() -> Self {
        Self {
            min: 0,
            max: 0,
            packet_queue: HashMap::new(),
            splits: SplitPacketQueue::new(),
        }
    }
    pub fn add(&mut self, frame: Frame) {
        if frame.split {
            self.splits.add(&frame);
            for mut packet in self.splits.get_and_clear() {
                let f = packet.get_frame().unwrap();
                if f.order_index >= self.max {
                    self.max = f.order_index + 1
                }
                self.packet_queue.insert(f.order_index, f);
            }
            return;
        }
        if frame.order_index < self.min {
            return;
        }
        if self.packet_queue.contains_key(&frame.order_index) {
            return;
        }
        if frame.order_index >= self.max {
            self.max = frame.order_index + 1
        }
        self.packet_queue.insert(frame.order_index, frame);
    }
    pub fn get_all(&mut self) -> Vec<Frame> {
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
