use std::collections::HashMap;

use crate::packets::{frame::Frame, frame_set::FrameSet};

const NEEDS_B_AND_AS_FLAG: u8 = 0x4;

const CONTINUOUS_SEND_FLAG: u8 = 0x8;

pub struct PacketQueue {
    pub queue: HashMap<u32, FrameSet>,
    pub time_passed: HashMap<u32, (u128, bool)>,
    pub max: u32,
    send_min: u32,
    resend: Vec<u32>,
    set_size: usize,
    set_queue: Vec<Frame>,
    mtu: u16,
    last_tick: u128,
}

impl PacketQueue {
    pub fn new(mtu: u16, last_tick: u128) -> Self {
        Self {
            queue: HashMap::new(),
            time_passed: HashMap::new(),
            max: 0,
            send_min: 0,
            resend: vec![],
            set_size: 0,
            set_queue: vec![],
            mtu,
            last_tick,
        }
    }
    pub fn add_frame(&mut self, frame: Frame) {
        if self.set_size + frame.length() < (self.mtu - 42) as usize && !frame.split {
            self.set_size += frame.length();
            self.set_queue.push(frame);
        } else {
            let set = FrameSet {
                header: {
                    if frame.split {
                        0x80 | NEEDS_B_AND_AS_FLAG | CONTINUOUS_SEND_FLAG
                    } else {
                        0x80 | NEEDS_B_AND_AS_FLAG
                    }
                },
                sequence_number: self.max,
                datas: vec![frame],
            };
            self.add(set);
        }
    }
    pub fn add(&mut self, frame_set: FrameSet) {
        if frame_set.sequence_number == self.max {
            self.max += 1;
            self.time_passed
                .insert(frame_set.sequence_number, (0, false));
            self.queue.insert(frame_set.sequence_number, frame_set);
        }
    }
    pub fn received(&mut self, sequence: u32) {
        if self.queue.contains_key(&sequence) {
            self.queue.remove(&sequence);
            self.time_passed.remove(&sequence);
        }
    }
    pub fn tick(&mut self, time: u128) {
        if !self.set_queue.is_empty() {
            let set = FrameSet {
                header: 0x80 | NEEDS_B_AND_AS_FLAG,
                sequence_number: self.max,
                datas: self.set_queue.clone(),
            };
            self.add(set);
            self.set_queue.clear();
            self.set_size = 0;
        }
        let time_passed = time - self.last_tick;
        for mut elem in self.time_passed.iter_mut() {
            if elem.1 .1 {
                elem.1 .0 += time_passed;
                if elem.1 .0 > 1000 {
                    //about 1000ms?
                    self.resend.push(*elem.0)
                }
                elem.1 .0 = time;
            }
        }
        self.last_tick = time;
    }
    pub fn readd(&mut self) {
        for resend in self.resend.iter() {
            if self.queue.contains_key(resend) {
                let mut added = self.queue.get_mut(resend).unwrap().clone();
                added.sequence_number = self.max;
                self.queue.insert(self.max, added);
                self.queue.remove(resend);
                self.time_passed.remove(resend);
                self.time_passed.insert(self.max, (0, false));
                self.max += 1;
            }
        }
        self.resend.clear();
    }
    pub fn resend(&mut self, index: u32) {
        if self.time_passed.contains_key(&index) {
            let mut added = self.queue.get_mut(&index).unwrap().clone();
            added.sequence_number = self.max;
            self.queue.insert(self.max, added);
            self.queue.remove(&index);
            self.time_passed.remove(&index);
            self.time_passed.insert(self.max, (0, false));
            self.max += 1;
        }
    }
    pub fn get_packet(&mut self, time: u128) -> Vec<&FrameSet> {
        //get send able packets and start timer
        self.tick(time);
        self.readd();
        let mut ret = vec![];
        for i in self.send_min..self.max {
            ret.push(self.queue.get(&i).unwrap());
            self.time_passed.get_mut(&i).unwrap().1 = true;
        }
        self.send_min = self.max;
        ret
    }
}

#[cfg(test)]
mod packet_q_test {
    use crate::packets::{Frame, Reliability};

    use super::PacketQueue;

    #[test]
    fn packet_q() {
        let time = std::time::Instant::now();
        let mut packetq = PacketQueue::new(1500, time.elapsed().as_millis());
        let frame = Frame::new(Reliability::Reliable, &[0u8; 100]);
        packetq.add_frame(frame);
        packetq.get_packet(time.elapsed().as_millis());
    }
}
