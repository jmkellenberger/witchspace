use rand::{Rng, RngCore, SeedableRng};
use rand_seeder::{Seeder, SipHasher, SipRng};
use std::hash::{Hash, Hasher};

pub type Dice = SipRng;

#[derive(Debug, Clone, Hash)]
pub struct Seed {
    pub top_level_seed: String,
    seed: u64,
}

impl Seed {
    pub fn new<H: Hash>(top_level_seed: String, inputs: Vec<H>) -> Self {
        let mut hasher = SipHasher::new();
        top_level_seed.hash(&mut hasher);
        for input in inputs {
            input.hash(&mut hasher)
        }
        let seed = hasher.finish();

        Self {
            top_level_seed,
            seed,
        }
    }

    pub fn random() -> Self {
        Self {
            top_level_seed: String::from(""),
            seed: Dice::from_entropy().next_u64(),
        }
    }

    pub fn subseed<H: Hash>(&self, inputs: Vec<H>) -> Self {
        let mut hasher = SipHasher::new();
        self.hash(&mut hasher);
        for input in inputs {
            input.hash(&mut hasher)
        }
        let seed = hasher.finish();
        Self {
            top_level_seed: self.top_level_seed.clone(),
            seed,
        }
    }

    pub fn to_rng(&self) -> Dice {
        Seeder::from(self.seed).make_rng()
    }
}

pub trait Rollable {
    fn roll_dice(&mut self, num: usize, sides: i32) -> Vec<i32>;
    fn roll(&mut self, num: usize, sides: i32, modifier: i32) -> i32;
    fn flux(&mut self, modifier: i32) -> i32;
}

impl Rollable for Dice {
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

    fn setup() -> Dice {
        Seed::new(String::from("test"), vec!["t", "e", "s", "t"]).to_rng()
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
