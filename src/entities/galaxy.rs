pub use crate::generators::{generate_sector, generate_system};
use crate::prelude::*;

#[derive(Debug)]
pub struct Galaxy {
    seed: Seed,
}

impl Galaxy {
    pub fn new(seed: String) -> Self {
        Self {
            seed: Seed::new(seed),
        }
    }

    pub fn random() -> Self {
        Self {
            seed: Seed::random(),
        }
    }

    pub fn get_sector(&self, coordinate: Coordinate) -> Sector {
        generate_sector(self.seed.subseed(vec![coordinate]), 8, 10)
    }

    pub fn get_system(&self, sec_coordinate: Coordinate, hex_coordinate: Coordinate) -> System {
        let seed = self
            .seed
            .subseed(vec![sec_coordinate])
            .subseed(vec![hex_coordinate]);

        generate_system(seed, hex_coordinate)
    }
}
