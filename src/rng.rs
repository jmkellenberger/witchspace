use rand::Rng;
pub use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn rng_from_seed<T: Hash>(seed: T) -> Pcg64 {
    Seeder::from(seed).make_rng()
}

#[allow(dead_code)]
pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub trait Rollable {
    fn roll(&mut self, n: u32) -> u32;
    fn flux(&mut self) -> i32;
}

impl Rollable for Pcg64 {
    fn roll(&mut self, n: u32) -> u32 {
        if n < 1 {
            return 0;
        } else {
            (0..n).fold(0, |acc, _| self.gen_range(1..7) + acc)
        }
    }

    fn flux(&mut self) -> i32 {
        self.roll(1) as i32 - self.roll(1) as i32
    }
}
