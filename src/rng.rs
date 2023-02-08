use rand::Rng;
use rand::SeedableRng;
pub use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn rng_from_seed<T: Hash>(seed: T) -> Pcg64 {
    Seeder::from(seed).make_rng()
}

pub fn rng() -> Pcg64 {
    Pcg64::from_entropy()
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub trait Rollable {
    fn roll_dice(&mut self, num: usize, sides: i32) -> Vec<i32>;
    fn roll(&mut self, num: usize, sides: i32, modifier: i32) -> i32;
    fn flux(&mut self, modifier: i32) -> i32;
}

impl Rollable for Pcg64 {
    fn roll_dice(&mut self, num: usize, sides: i32) -> Vec<i32> {
        if sides < 1 || num < 1 {
            return vec![0];
        } else {
            return (0..num).map(|_| self.gen_range(1..=sides)).collect();
        }
    }

    fn roll(&mut self, num: usize, sides: i32, modifier: i32) -> i32 {
        if sides < 1 || num < 1 {
            return 0;
        } else {
            self.roll_dice(num, sides).into_iter().sum::<i32>() + modifier
        }
    }

    fn flux(&mut self, modifier: i32) -> i32 {
        self.roll_dice(2, 6)
            .into_iter()
            .reduce(|acc, die| acc - die)
            .unwrap_or(0)
            + modifier
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Pcg64 {
        rng_from_seed("test")
    }

    #[test]
    fn test_roll_range() {
        let mut rng = setup();
        let rolls = rng.roll_dice(100, 6);
        let min = rolls.iter().min().unwrap();
        let max = rolls.iter().max().unwrap();

        assert_eq!((*min, *max), (1, 6))
    }

    #[test]
    fn test_batch_rolls() {
        let mut rng = setup();
        let rolls = rng.roll_dice(100, 6);
        let mut rng2 = setup();
        let rolls2 = rng2.roll_dice(100, 6);
        assert_eq!(rolls, rolls2)
    }

    #[test]
    fn test_roll() {
        let mut rng = setup();
        let rolls = rng.roll(100, 6, 0);
        let mut rng2 = setup();
        let rolls2 = rng2.roll(100, 6, 0);
        assert_eq!(rolls, rolls2)
    }

    #[test]
    fn test_flux() {
        let mut rng = setup();
        let rolls = rng.flux(0);
        let mut rng2 = setup();
        let rolls2 = rng2.flux(0);
        assert_eq!(rolls, rolls2)
    }

    #[test]
    fn test_flux_range() {
        let mut rng = setup();
        let rolls: Vec<i32> = (0..100).map(|_| rng.flux(0)).collect();
        let min = rolls.iter().min().unwrap();
        let max = rolls.iter().max().unwrap();
        assert_eq!((*min, *max), (-5, 5))
    }
}
