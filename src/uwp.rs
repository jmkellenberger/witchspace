use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Uwp {
    port: String,
    size: i32,
    atmosphere: i32,
    hydrographics: i32,
    population: i32,
    government: i32,
    law: i32,
    tech: i32,
}

impl Uwp {
    pub fn generate_mainworld<R: Rollable>(rng: &mut R) -> Self {
        let port = String::from(
            ["A", "A", "A", "B", "B", "C", "C", "D", "E", "E", "X"][rng.roll(2, 6, -2) as usize],
        );

        let size = match rng.roll(2, 6, -2) {
            10 => rng.roll(1, 6, 9),
            roll => roll,
        };

        let atmosphere = match size {
            0 => 0,
            _ => rng.flux(size).clamp(0, 15),
        };

        let hydrographics = match (size, atmosphere) {
            (0 | 1, _) => 0,
            (_, 0..=2 | 10..=15) => rng.flux(atmosphere - 4).clamp(0, 10),
            (_, _) => rng.flux(atmosphere).clamp(0, 10),
        };

        let population = match rng.roll(2, 6, -2) {
            10 => rng.roll(2, 6, 3),
            roll => roll,
        };

        let government = match population {
            0 => 0,
            _ => rng.flux(population).clamp(0, 15),
        };
        let law = match population {
            0 => 0,
            _ => rng.flux(government).clamp(0, 18),
        };

        let tech = rng
            .roll(
                1,
                6,
                tech_mod(
                    &port,
                    size,
                    atmosphere,
                    hydrographics,
                    population,
                    government,
                ),
            )
            .clamp(0, 33);

        Self {
            port,
            size,
            atmosphere,
            hydrographics,
            population,
            government,
            law,
            tech,
        }
    }
}

fn tech_mod(
    port: &str,
    size: i32,
    atmosphere: i32,
    hydrographics: i32,
    population: i32,
    government: i32,
) -> i32 {
    port_tech(port)
        + size_tech(size)
        + atmmosphere_tech(atmosphere)
        + hydrographics_tech(hydrographics)
        + population_tech(population)
        + government_tech(government)
}

fn port_tech(port: &str) -> i32 {
    match port {
        "A" => 6,
        "B" => 4,
        "C" => 2,
        "F" => 1,
        "X" => -4,
        _ => 0,
    }
}

fn size_tech(size: i32) -> i32 {
    match size {
        0..=1 => 2,
        2..=4 => 1,
        _ => 0,
    }
}

fn atmmosphere_tech(atmosphere: i32) -> i32 {
    match atmosphere {
        0..=3 | 10..=15 => 1,
        _ => 0,
    }
}

fn hydrographics_tech(hydrographics: i32) -> i32 {
    match hydrographics {
        9 => 1,
        10 => 2,
        _ => 0,
    }
}

fn population_tech(population: i32) -> i32 {
    match population {
        1..=5 => 1,
        9 => 2,
        10..=15 => 4,
        _ => 0,
    }
}

fn government_tech(government: i32) -> i32 {
    match government {
        0 | 5 => 1,
        14 => -2,
        _ => 0,
    }
}
