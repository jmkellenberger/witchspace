use crate::prelude::*;

mod worldgen;
use worldgen::*;
mod stargen;
use stargen::*;

pub fn generate_system(seed: Seed) -> System {
    let mut rng = seed.to_rng();
    let stars = generate_stars(&mut rng);
    let mainworld = generate_mainworld(&mut rng);

    let worlds = vec![];
    let belts = rng.roll(1, 6, -3).max(0);
    let gas_giants = ((rng.roll(2, 6, 0) as f32 / 2.0) - 2.0).round().max(0.0) as i32;

    System {
        stars,
        mainworld,
        worlds,
        belts,
        gas_giants,
    }
}
