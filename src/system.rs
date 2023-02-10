use crate::prelude::*;

pub struct System {
    pub location: Coordinate,
    pub name: String,
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
            "{} {:16} {:10} {:24} {} {:4} {} {} {}",
            self.location,
            self.name,
            self.mainworld,
            self.mainworld.trade_codes(),
            self.extensions,
            self.mainworld.bases_to_string(),
            pbg,
            self.mainworld.travel_zone,
            self.stars
        )
    }
}
