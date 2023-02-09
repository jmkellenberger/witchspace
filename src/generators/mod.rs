use crate::prelude::*;

mod worldgen;
use worldgen::*;
mod stargen;
use stargen::*;

pub fn generate_system(seed: Seed, location: Coordinate) -> System {
    let mut rng = seed.to_rng();
    let stars = generate_stars(&mut rng);
    let mainworld = generate_mainworld(&mut rng);

    let worlds = vec![];
    let belts = rng.roll(1, 6, -3).max(0);
    let gas_giants = ((rng.roll(2, 6, 0) as f32 / 2.0) - 2.0).round().max(0.0) as i32;

    System {
        location,
        stars,
        mainworld,
        worlds,
        belts,
        gas_giants,
    }
}

pub fn generate_sector(seed: Seed, row_count: u32, col_count: u32) -> Vec<System> {
    let mut rng = seed.to_rng();
    let mut systems = vec![];

    for row in 1..row_count + 1 {
        for col in 1..col_count + 1 {
            if rng.roll(1, 6, 0) > 4 {
                let coordinate = Coordinate::new(row as i32, col as i32);
                let subseed = seed.subseed(vec![coordinate]);
                systems.push(generate_system(subseed, coordinate))
            } else {
            }
        }
    }
    systems
}
