//! Simple Bloom filter for fast "definitely not present" checks on log content.

use crate::parser::LogLine;

#[allow(dead_code)]
pub struct BloomFilter {
    bits: Vec<u64>,
    size: usize,
    hashes: u32,
}

impl BloomFilter {
    #[allow(dead_code)]
    pub fn new(size_bits: usize, hash_count: u32) -> Self {
        let words = size_bits.div_ceil(64);
        Self {
            bits: vec![0u64; words],
            size: size_bits,
            hashes: hash_count,
        }
    }

    fn hash(&self, item: &str, seed: u32) -> usize {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        seed.hash(&mut h);
        item.hash(&mut h);
        (h.finish() as usize) % self.size
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, item: &str) {
        for i in 0..self.hashes {
            let bit = self.hash(item, i);
            self.bits[bit / 64] |= 1u64 << (bit % 64);
        }
    }

    #[allow(dead_code)]
    pub fn contains(&self, item: &str) -> bool {
        (0..self.hashes).all(|i| {
            let bit = self.hash(item, i);
            (self.bits[bit / 64] >> (bit % 64)) & 1 == 1
        })
    }
}

#[allow(dead_code)]
pub fn build_from_lines(lines: &[LogLine]) -> BloomFilter {
    let mut bf = BloomFilter::new(lines.len().max(64) * 10, 4);
    for line in lines {
        bf.insert(&line.raw);
    }
    bf
}
