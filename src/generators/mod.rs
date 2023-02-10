use crate::prelude::*;

mod worldgen;
use worldgen::*;
mod stargen;
use stargen::*;

fn habitable_zone_variance(flux: i32) -> i32 {
    match flux {
        -6 => -2,
        -5..=-3 => 1,
        3..=5 => 1,
        6 => 2,
        _ => 0,
    }
}

pub fn generate_system(seed: Seed, location: Coordinate) -> System {
    let mut rng = seed.to_rng();
    let stars = generate_stars(&mut rng);
    let hz_variance = habitable_zone_variance(rng.flux(0));
    let orbit = stars.primary.habitable_zone_orbit(hz_variance);
    let mainworld = generate_mainworld(&mut rng, hz_variance, orbit);

    let belts = rng.roll(1, 6, -3).max(0);
    let gas_giants = ((rng.roll(2, 6, 0) as f32 / 2.0) - 2.0).round().max(0.0) as i32;
    let worlds = rng.roll(2, 6, 1 + gas_giants + belts);

    let extensions = Extensions::new(&mut rng, &mainworld, gas_giants + belts);

    System {
        location,
        stars,
        extensions,
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