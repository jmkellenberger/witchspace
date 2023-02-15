use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct System {
    pub location: Coordinate,
    pub name: String,
    pub stars: Vec<Star>,
    pub mainworld: World,
    pub extensions: Extensions,
    pub worlds: i32,
    pub belts: i32,
    pub gas_giants: i32,
    pub allegiance: String,
}

impl System {
    pub fn has_knight(&self) -> bool {
        //TODO: Check allegiance eventually
        true
    }
    pub fn has_baronet(&self) -> bool {
        self.mainworld.is_pre_rich() || self.mainworld.is_preagricultural()
    }

    pub fn has_baron(&self) -> bool {
        self.mainworld.is_rich() || self.mainworld.is_agricultural()
    }

    pub fn has_marquis(&self) -> bool {
        self.mainworld.is_preindustrial()
    }

    pub fn has_viscount(&self) -> bool {
        self.mainworld.is_pre_high_pop()
    }

    pub fn has_count(&self) -> bool {
        self.mainworld.is_industrial() || self.mainworld.is_high_pop()
    }

    pub fn has_duke(&self) -> bool {
        self.extensions.importance > 3
    }

    pub fn bases(&self) -> String {
        self.mainworld.bases_to_string()
    }

    pub fn travel_zone(&self) -> String {
        self.mainworld.travel_zone.to_string()
    }

    pub fn pbg(&self) -> String {
        format!(
            "{}{}{}",
            self.mainworld.population_digit, self.belts, self.gas_giants
        )
    }

    pub fn stars(&self) -> String {
        self.stars
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn trade_codes(&self) -> String {
        self.mainworld.trade_codes()
    }

    pub fn nobility(&self) -> String {
        let mut ns = Vec::new();

        if self.has_knight() {
            ns.push("B")
        }

        if self.has_baronet() {
            ns.push("c")
        }

        if self.has_baron() {
            ns.push("C")
        }

        if self.has_marquis() {
            ns.push("D")
        }

        if self.has_viscount() {
            ns.push("e")
        }

        if self.has_count() {
            ns.push("E")
        }

        if self.has_duke() {
            ns.push("f")
        }

        ns.join("")
    }
}

impl std::fmt::Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {:16} {:10} {:24} {} {:8} {:4} {} {} {:2} {:2} {}",
            self.location,
            self.name,
            self.mainworld,
            self.trade_codes(),
            self.extensions,
            self.nobility(),
            self.bases(),
            self.travel_zone(),
            self.pbg(),
            self.worlds,
            self.allegiance,
            self.stars()
        )
    }
}
