use blake3;

pub struct GokoPoh {
    pub hash: [u8; 32],
    pub count: u64,
}

impl GokoPoh {
    pub fn new() -> Self {
        Self {
            hash: [0u8; 32],
            count: 0,
        }
    }

    pub fn tick(&mut self) {
        self.hash = *blake3::hash(&self.hash).as_bytes();
        self.count += 1;
    }

    pub fn record(&mut self, data: &[u8]) {
        let mut input = Vec::new();
        input.extend_from_slice(&self.hash);
        input.extend_from_slice(data);
        self.hash = *blake3::hash(&input).as_bytes();
        self.count += 1;
    }

    pub fn verify(start: [u8; 32], ticks: u64) -> [u8; 32] {
        let mut hash = start;
        for _ in 0..ticks {
            hash = *blake3::hash(&hash).as_bytes();
        }
        hash
    }
}
