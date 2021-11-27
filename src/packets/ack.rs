pub struct ACK {
    pub record_count: u16,
    pub max_equals_min: bool,
    pub sequences: Vec<u32>,
}

impl ACK {
    /*pub fn new(sequences: &[u32]) -> Self {
        sequences.sort();
        Self{
            record_count : 1,

        }
    }*/
}
