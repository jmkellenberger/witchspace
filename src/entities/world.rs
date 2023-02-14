use std::fmt::Display;

use super::to_ehex;

#[derive(Debug, Clone, PartialEq)]
pub enum Base {
    Naval,
    Scout,
    WayStation,
    Depot,
}

impl Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            Base::Naval => "N",
            Base::Scout => "S",
            Base::WayStation => "W",
            Base::Depot => "D",
        };
        write!(f, "{code:}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TravelZone {
    Green,
    Amber,
    Red,
}

impl Display for TravelZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            TravelZone::Green => " ",
            TravelZone::Amber => "A",
            TravelZone::Red => "R",
        };
        write!(f, "{code:}")
    }
}

pub type Orbit = u8;

#[derive(Debug, Clone, PartialEq)]
pub enum MainWorldType {
    Planet,
    CloseSatellite(Orbit),
    FarSatellite(Orbit),
}

#[derive(Debug, Clone, PartialEq)]
pub struct World {
    pub mainworld_type: MainWorldType,
    pub hz_variance: i32,
    pub orbit: i32,
    pub port: String,
    pub bases: Vec<Base>,
    pub size: i32,
    pub atmosphere: i32,
    pub hydrographics: i32,
    pub population: i32,
    pub population_digit: i32,
    pub government: i32,
    pub law: i32,
    pub tech: i32,
    pub travel_zone: TravelZone,
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}-{}",
            self.port,
            to_ehex(self.size),
            to_ehex(self.atmosphere),
            to_ehex(self.hydrographics),
            to_ehex(self.population),
            to_ehex(self.government),
            to_ehex(self.law),
            to_ehex(self.tech),
        )
    }
}

impl World {
    pub fn bases_to_string(&self) -> String {
        self.bases
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn has_naval_base(&self) -> bool {
        self.bases.contains(&Base::Naval)
    }

    pub fn has_scout_base(&self) -> bool {
        self.bases.contains(&Base::Scout)
    }

    pub fn has_way_station(&self) -> bool {
        self.bases.contains(&Base::WayStation)
    }

    pub fn has_depot(&self) -> bool {
        self.bases.contains(&Base::Depot)
    }

    // Planetary Trade Codes
    pub fn is_asteroid_belt(&self) -> bool {
        self.size == 0 && self.atmosphere == 0 && self.hydrographics == 0
    }

    pub fn is_vacuum(&self) -> bool {
        self.atmosphere == 0
    }

    pub fn is_desert(&self) -> bool {
        self.hydrographics == 0 && (2..=9).contains(&self.atmosphere)
    }

    pub fn is_fluid_oceans(&self) -> bool {
        self.hydrographics != 0 && (10..=12).contains(&self.atmosphere)
    }

    pub fn is_garden(&self) -> bool {
        [6, 7, 8].contains(&self.size)
            && [5, 6, 8].contains(&self.atmosphere)
            && [5, 6, 7].contains(&self.hydrographics)
    }

    pub fn is_hellworld(&self) -> bool {
        (3..=12).contains(&self.size)
            && [2, 4, 7, 9, 10, 11, 12].contains(&self.atmosphere)
            && self.hydrographics < 3
    }

    pub fn is_ice_capped(&self) -> bool {
        self.atmosphere < 2 && self.hydrographics != 0
    }

    pub fn is_ocean_world(&self) -> bool {
        self.size > 9 && self.atmosphere > 2 && self.hydrographics == 10 && !self.is_fluid_oceans()
    }
    pub fn is_water_world(&self) -> bool {
        self.size > 2
            && self.size < 10
            && self.atmosphere > 2
            && self.hydrographics == 10
            && !self.is_fluid_oceans()
    }

    pub fn is_satellite(&self) -> bool {
        match self.mainworld_type {
            MainWorldType::FarSatellite(_orbit) => true,
            _ => false,
        }
    }

    pub fn is_tidally_locked(&self) -> bool {
        match self.mainworld_type {
            MainWorldType::CloseSatellite(_orbit) => true,
            _ => false,
        }
    }

    // Population Trade Codes
    pub fn is_dieback(&self) -> bool {
        self.population == 0 && self.tech != 0
    }

    pub fn is_barren(&self) -> bool {
        self.population == 0 && self.tech == 0
    }

    pub fn is_low_pop(&self) -> bool {
        (1..=3).contains(&self.population)
    }

    pub fn is_nonindustrial(&self) -> bool {
        (4..=6).contains(&self.population)
    }

    pub fn is_pre_high_pop(&self) -> bool {
        self.population == 8
    }

    pub fn is_high_pop(&self) -> bool {
        self.population > 8
    }

    // Economic Trade Codes
    pub fn is_preagricultural(&self) -> bool {
        (4..=9).contains(&self.atmosphere)
            && (4..=8).contains(&self.hydrographics)
            && [4, 8].contains(&self.population)
    }

    pub fn is_agricultural(&self) -> bool {
        (4..=9).contains(&self.atmosphere)
            && (4..=8).contains(&self.hydrographics)
            && (5..=7).contains(&self.population)
    }

    pub fn is_nonagricultural(&self) -> bool {
        (0..=3).contains(&self.atmosphere)
            && (0..=3).contains(&self.hydrographics)
            && self.population > 5
    }

    pub fn is_prison(&self) -> bool {
        [2, 3, 10, 11].contains(&self.atmosphere)
            && (1..=5).contains(&self.population)
            && self.law > 5
    }

    pub fn is_preindustrial(&self) -> bool {
        [0, 1, 2, 4, 7, 9].contains(&self.atmosphere) && [7, 8].contains(&self.population)
    }

    pub fn is_industrial(&self) -> bool {
        [0, 1, 2, 4, 7, 9, 10, 11, 12].contains(&self.atmosphere) && self.population > 8
    }

    pub fn is_poor(&self) -> bool {
        (2..=5).contains(&self.atmosphere) && self.hydrographics < 4
    }

    pub fn is_pre_rich(&self) -> bool {
        [6, 8].contains(&self.atmosphere) && [5, 9].contains(&self.population)
    }

    pub fn is_rich(&self) -> bool {
        [6, 8].contains(&self.atmosphere) && (6..=8).contains(&self.population)
    }

    // Climate trade codes
    pub fn is_frozen(&self) -> bool {
        self.hz_variance > 1 && (2..=9).contains(&self.size) && self.hydrographics != 0
    }

    pub fn is_hot(&self) -> bool {
        self.hz_variance < 0
    }

    pub fn is_cold(&self) -> bool {
        self.hz_variance == 1
    }

    pub fn is_twilight_zone(&self) -> bool {
        self.orbit < 2
    }

    pub fn is_tropic(&self) -> bool {
        self.is_hot()
            && (6..=9).contains(&self.size)
            && (4..=9).contains(&self.atmosphere)
            && (3..=7).contains(&self.hydrographics)
    }

    pub fn is_tundra(&self) -> bool {
        self.is_cold()
            && (6..=9).contains(&self.size)
            && (4..=9).contains(&self.atmosphere)
            && (3..=7).contains(&self.hydrographics)
    }
    // Secondary Trade Codes TODO: Add NonMainWorld Codes
    pub fn is_reserve(&self) -> bool {
        self.population < 5 && self.government == 6 && [0, 4, 5].contains(&self.law)
    }
    // Political Trade Codes TODO: Add capital logic
    pub fn is_colony(&self) -> bool {
        self.population > 4 && self.government == 6 && self.law < 4
    }

    // Special Trade Codes
    pub fn is_forbiddin(&self) -> bool {
        self.travel_zone == TravelZone::Red
    }
    pub fn is_puzzle(&self) -> bool {
        self.population > 6 && self.travel_zone == TravelZone::Amber
    }

    pub fn is_dangerous(&self) -> bool {
        self.population < 7 && self.travel_zone == TravelZone::Amber
    }

    pub fn trade_codes(&self) -> String {
        let mut tc: Vec<&str> = vec![];
        if self.is_asteroid_belt() {
            tc.push("As")
        }

        if self.is_vacuum() {
            tc.push("Va")
        }

        if self.is_desert() {
            tc.push("De")
        }

        if self.is_fluid_oceans() {
            tc.push("Fl")
        }

        if self.is_garden() {
            tc.push("Ga")
        }

        if self.is_hellworld() {
            tc.push("He")
        }

        if self.is_ice_capped() {
            tc.push("Ic")
        }

        if self.is_ocean_world() {
            tc.push("Oc")
        }

        if self.is_water_world() {
            tc.push("Wa")
        }

        if self.is_satellite() {
            tc.push("Sa")
        }

        if self.is_tidally_locked() {
            tc.push("Lk")
        }

        if self.is_dieback() {
            tc.push("Di")
        }

        if self.is_barren() {
            tc.push("Ba")
        }

        if self.is_low_pop() {
            tc.push("Lo")
        }

        if self.is_nonindustrial() {
            tc.push("Ni")
        }

        if self.is_pre_high_pop() {
            tc.push("Ph")
        }

        if self.is_high_pop() {
            tc.push("Hi")
        }

        if self.is_preagricultural() {
            tc.push("Pa")
        }

        if self.is_agricultural() {
            tc.push("Ag")
        }

        if self.is_nonagricultural() {
            tc.push("Na")
        }

        if self.is_prison() {
            tc.push("Px")
        }

        if self.is_preindustrial() {
            tc.push("Pi")
        }

        if self.is_industrial() {
            tc.push("In")
        }

        if self.is_poor() {
            tc.push("Po")
        }

        if self.is_pre_rich() {
            tc.push("Pr")
        }

        if self.is_rich() {
            tc.push("Ri")
        }

        if self.is_frozen() {
            tc.push("Fr")
        }

        if self.is_hot() {
            tc.push("Ho")
        }

        if self.is_cold() {
            tc.push("Co")
        }

        if self.is_tundra() {
            tc.push("Tu")
        }

        if self.is_tropic() {
            tc.push("Tr")
        }

        if self.is_twilight_zone() {
            tc.push("Tz")
        }

        if self.is_reserve() {
            tc.push("Re")
        }

        if self.is_colony() {
            tc.push("Cy")
        }

        if self.is_forbiddin() {
            tc.push("Fo")
        }

        if self.is_puzzle() {
            tc.push("Pz")
        }

        if self.is_dangerous() {
            tc.push("Da")
        }

        String::from(tc.join(" "))
    }
}
