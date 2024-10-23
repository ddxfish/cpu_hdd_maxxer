use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

pub struct PatternGenerator {
    rng: StdRng,
}

impl PatternGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn generate_chunk(&mut self, size: usize) -> Vec<u8> {
        let mut data = vec![0u8; size];
        self.rng.fill(&mut data[..]);
        data
    }
}