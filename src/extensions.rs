use std::fmt::Display;

use crate::prelude::*;

pub struct Extensions {
    pub importance: i32,
    pub resources: i32,
    pub labor: i32,
    pub infrastructure: i32,
    pub efficiency: i32,
    pub heterogeneity: i32,
    pub acceptance: i32,
    pub strangeness: i32,
    pub symbols: i32,
}

impl Display for Extensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ix_sign = if self.importance < 0 { "" } else { "+" };

        let rli = [self.resources, self.labor, self.infrastructure]
            .into_iter()
            .map(|x| to_ehex(x))
            .collect::<Vec<String>>()
            .join("");

        let eff = if self.efficiency >= 0 {
            format!("+{}", self.efficiency)
        } else {
            format!("{}", self.efficiency)
        };

        let cx = [
            self.heterogeneity,
            self.acceptance,
            self.strangeness,
            self.symbols,
        ]
        .into_iter()
        .map(|x| to_ehex(x))
        .collect::<Vec<String>>()
        .join("");

        write!(f, "{{{}{}}}({rli}{eff})[{cx}]", ix_sign, self.importance)
    }
}

impl Extensions {
    pub fn new(rng: &mut Dice, world: &World, orbital_resources: i32) -> Self {
        // Extensions
        let importance = importance_extension(&world);
        // Economic
        let (resources, labor, infrastructure, efficiency) =
            economic_extension(rng, importance, world, orbital_resources);

        let (heterogeneity, acceptance, strangeness, symbols) =
            cultural_extensions(rng, importance, world);

        Self {
            importance,
            resources,
            labor,
            infrastructure,
            efficiency,
            heterogeneity,
            acceptance,
            strangeness,
            symbols,
        }
    }

    pub fn expected_daily_ship_traffic(&self) -> i32 {
        match self.importance {
            5 => 100,
            4 => 15,
            3 => 6,
            2 => 4,
            1 => 2,
            0 => 1,
            -1 => 1,
            _ => 0,
        }
    }

    pub fn resource_units(&self) -> i32 {
        self.resources.max(1)
            * self.labor.max(1)
            * self.infrastructure.max(1)
            * self.efficiency.max(1)
    }

    pub fn is_important(&self) -> bool {
        self.importance > 3
    }

    pub fn is_unimportant(&self) -> bool {
        self.importance < 1
    }
}

fn importance_extension(world: &World) -> i32 {
    [
        ["A", "B"].contains(&world.port.as_str()),
        world.tech > 15,
        world.tech > 9,
        world.is_agricultural(),
        world.is_rich(),
        world.is_high_pop(),
        world.is_industrial(),
        world.has_naval_base() && world.has_scout_base(),
        world.has_way_station(),
        !world.tech < 8,
        !world.population < 7,
        !["D", "E", "X"].contains(&world.port.as_str()),
    ]
    .into_iter()
    .fold(-3, |acc, x| acc + x as i32)
}

fn economic_extension(
    rng: &mut Dice,
    importance: i32,
    world: &World,
    orbital_resources: i32,
) -> (i32, i32, i32, i32) {
    let resources = rng.roll(2, 6, if world.tech > 7 { orbital_resources } else { 0 });
    let labor = (world.population - 1).max(0);
    let infrastructure = match world.population {
        0 => 0,
        1..=3 => importance,
        4..=6 => rng.roll(1, 6, importance),
        _ => rng.roll(2, 6, importance),
    }
    .max(0);

    let efficiency = rng.flux(0);

    (resources, labor, infrastructure, efficiency)
}

fn cultural_extensions(rng: &mut Dice, importance: i32, world: &World) -> (i32, i32, i32, i32) {
    if world.population == 0 {
        (0, 0, 0, 0)
    } else {
        let heterogeneity = rng.flux(world.population).max(1);
        let acceptance = (world.population + importance).max(1);
        let strangeness = rng.flux(5).max(1);
        let symbols = rng.flux(world.tech).max(1);
        (heterogeneity, acceptance, strangeness, symbols)
    }
}
