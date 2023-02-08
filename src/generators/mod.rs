use crate::prelude::*;

mod worldgen;
use worldgen::*;

pub fn generate_system<R: Rollable>(rng: &mut R) -> System {
    let mainworld = generate_mainworld(rng);

    let belts = rng.roll(1, 6, -3).max(0);
    let gas_giants = ((rng.roll(2, 6, 0) as f32 / 2.0) - 2.0).round().max(0.0) as i32;

    System {
        mainworld,
        belts,
        gas_giants,
    }
}
