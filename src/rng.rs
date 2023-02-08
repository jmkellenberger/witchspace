use rand::Rng;
use rand_seeder::{Seeder, SipRng};

pub type Dice = SipRng;

pub trait Rollable {
    fn seed<T: std::hash::Hash>(seed: T) -> Dice;
    fn roll_dice(&mut self, num: usize, sides: i32) -> Vec<i32>;
    fn roll(&mut self, num: usize, sides: i32, modifier: i32) -> i32;
    fn flux(&mut self, modifier: i32) -> i32;
}

impl Rollable for Dice {
    fn seed<T: std::hash::Hash>(seed: T) -> Dice {
        Seeder::from(seed).make_rng()
    }

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
        Dice::seed("test")
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
