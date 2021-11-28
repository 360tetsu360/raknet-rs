use std::net::SocketAddr;
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
}

#[derive(Clone)]
pub struct RaknetPacket {
    pub address: SocketAddr,
    pub guid: u64,
    pub length: usize,
    pub data: Vec<u8>,
}
