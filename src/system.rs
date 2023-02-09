use crate::prelude::*;

pub struct System {
    pub stars: Stars,
    pub mainworld: World,
    pub belts: i32,
    pub gas_giants: i32,
}

impl std::fmt::Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pbg = format!(
            "{}{}{}",
            self.mainworld.population_digit, self.belts, self.gas_giants
        );

        write!(f, "{} {} {}", self.mainworld, pbg, self.stars)
    }
}
