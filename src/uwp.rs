use std::fmt;

use crate::prelude::*;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Uwp {
    port: Starport,
    size: Size,
    atmosphere: Atmosphere,
    hydrographics: Hydrographics,
    population: Population,
    government: Government,
    law: Law,
    tech: Tech,
}

impl Uwp {
    pub fn random<R: Rollable>(rng: &mut R) -> Self {
        let port = Starport::random(rng);
        let size = Size::random(rng);
        let atmosphere = Atmosphere::random(rng, &size);
        let hydrographics = Hydrographics::random(rng, &size, &atmosphere);
        let population = Population::random(rng);
        let government = Government::random(rng, &population);
        let law = Law::random(rng, &population, &government);
        let tech_mods = port.tech_level_modifier()
            + size.tech_modifier()
            + atmosphere.tech_modifier()
            + hydrographics.tech_modifier()
            + population.tech_modifier()
            + government.tech_modifier()
            + law.tech_modifier();
        let tech = Tech::random(rng, tech_mods);
        Uwp {
            port,
            size,
            atmosphere,
            hydrographics,
            population,
            government,
            law,
            tech,
        }
    }

    pub fn port(&self) -> Starport {
        self.port
    }

    pub fn size(&self) -> u32 {
        self.size.0
    }

    pub fn atmosphere(&self) -> u32 {
        self.atmosphere.0
    }

    pub fn hydrographics(&self) -> u32 {
        self.hydrographics.0
    }

    pub fn population(&self) -> u32 {
        self.population.0
    }

    pub fn government(&self) -> u32 {
        self.government.0
    }

    pub fn law(&self) -> u32 {
        self.law.0
    }

    pub fn tech(&self) -> u32 {
        self.tech.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum Starport {
    A,
    B,
    C,
    D,
    E,
    X,
}

impl Starport {
    fn random<R: Rollable>(rng: &mut R) -> Self {
        match rng.roll(2, 6) {
            2 => Starport::A,
            3 => Starport::A,
            4 => Starport::A,
            5 => Starport::B,
            6 => Starport::B,
            7 => Starport::C,
            8 => Starport::C,
            9 => Starport::D,
            10 => Starport::E,
            11 => Starport::E,
            _ => Starport::X,
        }
    }

    fn tech_level_modifier(&self) -> i32 {
        match &self {
            Self::A => 6,
            Self::B => 4,
            Self::C => 2,
            Self::X => -4,
            _ => 0,
        }
    }
}

impl fmt::Display for Starport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let class = match &self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",
            Self::E => "E",
            Self::X => "X",
        };
        write!(f, "{}", class)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Size(u32);
impl Size {
    fn random<R: Rollable>(rng: &mut R) -> Self {
        let roll = rng.roll(2, 6) - 2;
        if roll == 10 {
            Self(rng.roll(1, 6) + 9)
        } else {
            Self(roll)
        }
    }

    fn tech_modifier(&self) -> i32 {
        match self.0 {
            0..=1 => 2,
            2..=4 => 1,
            _ => 0,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Atmosphere(u32);
impl Atmosphere {
    const MIN: i32 = 0;
    const MAX: i32 = 15;

    pub fn new(value: u32) -> Option<Self> {
        if Self::MIN <= value as i32 && value as i32 <= Self::MAX {
            Some(Self(value))
        } else {
            None
        }
    }
    fn random<R: Rollable>(rng: &mut R, size: &Size) -> Self {
        let roll = rng.flux();
        let value = if size.0 == 0 {
            0
        } else {
            (roll + size.0 as i32).clamp(Self::MIN, Self::MAX)
        };

        Self(value as u32)
    }

    pub fn tech_modifier(&self) -> i32 {
        match self.0 {
            0..=3 | 10..=15 => 1,
            _ => 0,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Hydrographics(u32);
impl Hydrographics {
    const MIN: i32 = 0;
    const MAX: i32 = 10;

    pub fn new(value: u32) -> Option<Self> {
        if Self::MIN <= value as i32 && value as i32 <= Self::MAX {
            Some(Self(value))
        } else {
            None
        }
    }
    fn random<R: Rollable>(rng: &mut R, size: &Size, atmosphere: &Atmosphere) -> Self {
        let roll = rng.flux() as i32;

        let value = if size.0 < 2 {
            0
        } else {
            let modifier: i32 = match atmosphere.0 {
                0..=2 | 10..=15 => atmosphere.0 as i32 - 4,
                _ => 0 + atmosphere.0 as i32,
            };
            (roll + modifier).clamp(Self::MIN, Self::MAX)
        };

        Self(value as u32)
    }

    pub fn tech_modifier(&self) -> i32 {
        match self.0 {
            9 => 1,
            10 => 2,
            _ => 0,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Population(u32);
impl Population {
    const MIN: u32 = 0;
    const MAX: u32 = 15;

    pub fn new(value: u32) -> Option<Self> {
        if Self::MIN <= value && value <= Self::MAX {
            Some(Self(value))
        } else {
            None
        }
    }
    fn random<R: Rollable>(rng: &mut R) -> Self {
        let roll = rng.roll(2, 6) - 2;
        if roll == 10 {
            Self(rng.roll(2, 6) + 3)
        } else {
            Self(roll)
        }
    }

    fn tech_modifier(&self) -> i32 {
        match self.0 {
            1..=5 => 1,
            9 => 2,
            10..=15 => 4,
            _ => 0,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Government(u32);
impl Government {
    const MIN: i32 = 0;
    const MAX: i32 = 15;

    pub fn new(value: u32) -> Option<Self> {
        if Self::MIN <= value as i32 && value as i32 <= Self::MAX {
            Some(Self(value))
        } else {
            None
        }
    }
    fn random<R: Rollable>(rng: &mut R, population: &Population) -> Self {
        let roll = rng.flux();
        let value = if population.0 == 0 {
            0
        } else {
            (roll + population.0 as i32).clamp(Self::MIN, Self::MAX)
        };

        Self(value as u32)
    }

    fn tech_modifier(&self) -> i32 {
        match self.0 {
            0 | 5 => 1,
            14 => -2,
            _ => 0,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Law(u32);
impl Law {
    const MIN: i32 = 0;
    const MAX: i32 = 18;

    pub fn new(value: u32) -> Option<Self> {
        if Self::MIN <= value as i32 && value as i32 <= Self::MAX {
            Some(Self(value))
        } else {
            None
        }
    }
    fn random<R: Rollable>(rng: &mut R, population: &Population, government: &Government) -> Self {
        let roll = rng.flux();
        let value = if population.0 == 0 {
            0
        } else {
            (roll + government.0 as i32).clamp(Self::MIN, Self::MAX)
        };

        Self(value as u32)
    }

    pub fn tech_modifier(&self) -> i32 {
        0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Tech(u32);

impl Tech {
    const MIN: i32 = 0;
    const MAX: i32 = 33;

    pub fn new(value: u32) -> Option<Self> {
        if Self::MIN <= value as i32 && value as i32 <= Self::MAX {
            Some(Self(value))
        } else {
            None
        }
    }

    fn random<R: Rollable>(rng: &mut R, tech_mods: i32) -> Self {
        Self((rng.roll(1, 6) as i32 + tech_mods).clamp(Self::MIN, Self::MAX) as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_a() {
        assert_eq!(Starport::A.tech_level_modifier(), 6)
    }

    #[test]
    fn test_port_b() {
        assert_eq!(Starport::B.tech_level_modifier(), 4)
    }

    #[test]
    fn test_port_c() {
        assert_eq!(Starport::C.tech_level_modifier(), 2)
    }

    #[test]
    fn test_port_d() {
        assert_eq!(Starport::D.tech_level_modifier(), 0)
    }

    #[test]
    fn test_port_e() {
        assert_eq!(Starport::E.tech_level_modifier(), 0)
    }

    #[test]
    fn test_port_x() {
        assert_eq!(Starport::X.tech_level_modifier(), -4)
    }
}
