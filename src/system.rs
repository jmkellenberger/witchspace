use crate::prelude::*;

pub struct System {
    mainworld: Uwp,
    bases: String,
    population_modifier: i32,
    belts: i32,
    gas_giants: i32,
}

impl std::fmt::Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}{}{}",
            self.mainworld, self.bases, self.population_modifier, self.belts, self.gas_giants
        )
    }
}

impl System {
    pub fn generate<R: Rollable>(rng: &mut R) -> System {
        let mainworld = Uwp::generate_mainworld(rng);
        let bases = Uwp::check_bases(&mainworld, rng);

        let population_modifier = Uwp::get_population_mod(&mainworld, rng);

        let belts = rng.roll(1, 6, -3).max(0);
        let gas_giants = ((rng.roll(2, 6, 0) as f32 / 2.0) - 2.0).round().max(0.0) as i32;

        Self {
            mainworld,
            bases: String::from(bases),
            population_modifier,
            belts,
            gas_giants,
        }
    }
}
