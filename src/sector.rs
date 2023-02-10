use std::fmt::Display;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Sector {
    pub seed: Seed,
    pub systems: Vec<System>,
}

impl Display for Sector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:4} {:16} {:10}{:24} {:17} {:8} {:4} {} {} {:2} {:2} {}\n{}",
            "Hex",
            "World Name",
            "UWP",
            "Trade Codes",
            "Extensions",
            "N",
            "B",
            "Z",
            "PBG",
            "W",
            "A",
            "Stellar",
            self.systems
                .iter()
                .map(|sys| sys.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
