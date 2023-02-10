use crate::prelude::*;

pub struct System {
    pub location: Coordinate,
    pub stars: Stars,
    pub mainworld: World,
    pub extensions: Extensions,
    pub worlds: i32,
    pub belts: i32,
    pub gas_giants: i32,
}

impl std::fmt::Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pbg = format!(
            "{}{}{}",
            self.mainworld.population_digit, self.belts, self.gas_giants
        );

        write!(
            f,
            "{} {} {} {} {} {} {}",
            self.location,
            self.mainworld,
            self.mainworld.trade_codes(),
            self.extensions,
            pbg,
            self.mainworld.travel_zone,
            self.stars
        )
    }
}
