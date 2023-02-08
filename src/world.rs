use std::fmt::Display;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct World {
    pub port: String,
    pub size: i32,
    pub atmosphere: i32,
    pub hydrographics: i32,
    pub population: i32,
    pub population_digit: i32,
    pub government: i32,
    pub law: i32,
    pub tech: i32,
    pub naval_base: bool,
    pub scout_base: bool,
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bases = match (self.naval_base, self.scout_base) {
            (true, true) => "B",
            (true, false) => "N",
            (false, true) => "S",
            (false, false) => " ",
        };
        write!(
            f,
            "{}{}{}{}{}{}{}-{} {}",
            self.port,
            to_ehex(self.size),
            to_ehex(self.atmosphere),
            to_ehex(self.hydrographics),
            to_ehex(self.population),
            to_ehex(self.government),
            to_ehex(self.law),
            to_ehex(self.tech),
            bases,
        )
    }
}
