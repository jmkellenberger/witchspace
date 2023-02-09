use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub struct Stars {
    pub primary: Star,
    pub primary_companion: Option<Star>,
    pub close: Option<Star>,
    pub close_companion: Option<Star>,
    pub near: Option<Star>,
    pub near_companion: Option<Star>,
    pub far: Option<Star>,
    pub far_companion: Option<Star>,
}

impl Display for Stars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let companions = vec![
            self.primary_companion,
            self.close,
            self.close_companion,
            self.near,
            self.near_companion,
            self.far,
            self.far_companion,
        ]
        .into_iter()
        .filter(|star| star.is_some())
        .map(|star| star.unwrap().to_string())
        .collect::<Vec<String>>()
        .join(" ");

        write!(f, "{} {}", self.primary, companions)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Star {
    Star(Class, Decimal, Size),
    Dwarf(Class),
    BrownDwarf,
}

impl Star {
    pub fn class(&self) -> &Class {
        match &self {
            Star::BrownDwarf => &Class::BD,
            Star::Dwarf(class) => &class,
            Star::Star(class, _, _) => &class,
        }
    }

    pub fn size(&self) -> &Size {
        match &self {
            Star::BrownDwarf => &Size::D,
            Star::Dwarf(_) => &Size::D,
            Star::Star(_, _, size) => &size,
        }
    }

    pub fn habitable_zone_mod(&self) -> i32 {
        match self.class() {
            Class::M => 2,
            Class::O | Class::B => -2,
            _ => 0,
        }
    }

    pub fn habitable_zone_orbit(&self, hz_var: i32) -> i32 {
        let size = self.size();
        let class = self.class();
        let hz_orbit = class.habitable_zone(size) - hz_var;

        hz_orbit.max(0)
    }
}

impl Display for Star {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Star(class, decimal, size) => write!(f, "{}{} {}", class, decimal, size),
            Self::Dwarf(class) => write!(f, "{}D", class),
            Self::BrownDwarf => write!(f, "BD"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Class {
    O,
    B,
    A,
    F,
    G,
    K,
    M,
    BD,
}

impl Class {
    fn habitable_zone(&self, size: &Size) -> i32 {
        match &self {
            Class::O => Self::o_habitable_zones(size),
            Class::B => Self::b_habitable_zones(size),
            Class::A => Self::a_habitable_zones(size),
            Class::F => Self::f_habitable_zones(size),
            Class::G => Self::g_habitable_zones(size),
            Class::K => Self::k_habitable_zones(size),
            Class::M => Self::m_habitable_zones(size),
            Class::BD => 0,
        }
    }
    fn o_habitable_zones(size: &Size) -> i32 {
        match size {
            Size::Ia | Size::Ib => 15,
            Size::II => 14,
            Size::III => 13,
            Size::IV => 12,
            Size::V => 11,
            Size::D => 1,
            _ => 0,
        }
    }

    fn b_habitable_zones(size: &Size) -> i32 {
        match size {
            Size::Ia | Size::Ib => 13,
            Size::II => 12,
            Size::III => 11,
            Size::IV => 10,
            Size::V => 9,
            Size::D => 0,
            _ => 0,
        }
    }

    fn a_habitable_zones(size: &Size) -> i32 {
        match size {
            Size::Ia => 12,
            Size::Ib => 11,
            Size::II => 9,
            Size::III | Size::IV | Size::V => 7,
            _ => 0,
        }
    }

    fn f_habitable_zones(size: &Size) -> i32 {
        match size {
            Size::Ia => 11,
            Size::Ib => 10,
            Size::II => 9,
            Size::III | Size::IV => 6,
            Size::V => 4,
            Size::VI => 3,
            Size::D => 0,
        }
    }

    fn g_habitable_zones(size: &Size) -> i32 {
        match size {
            Size::Ia => 12,
            Size::Ib => 10,
            Size::II => 9,
            Size::III => 7,
            Size::IV => 5,
            Size::V => 3,
            Size::VI => 2,
            Size::D => 0,
        }
    }

    fn k_habitable_zones(size: &Size) -> i32 {
        match size {
            Size::Ia => 12,
            Size::Ib => 10,
            Size::II => 9,
            Size::III => 8,
            Size::IV => 5,
            Size::V => 2,
            Size::VI => 1,
            Size::D => 0,
        }
    }
    fn m_habitable_zones(size: &Size) -> i32 {
        match size {
            Size::Ia => 12,
            Size::Ib => 11,
            Size::II => 10,
            Size::III => 9,
            _ => 0,
        }
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::O => write!(f, "O"),
            Self::B => write!(f, "B"),
            Self::A => write!(f, "A"),
            Self::F => write!(f, "F"),
            Self::G => write!(f, "G"),
            Self::K => write!(f, "K"),
            Self::M => write!(f, "M"),
            Self::BD => write!(f, "BD"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Size {
    Ia,
    Ib,
    II,
    III,
    IV,
    V,
    VI,
    D,
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Ia => write!(f, "Ia"),
            Self::Ib => write!(f, "Ib"),
            Self::II => write!(f, "II"),
            Self::III => write!(f, "III"),
            Self::IV => write!(f, "IV"),
            Self::V => write!(f, "V"),
            Self::VI => write!(f, "VI"),
            Self::D => write!(f, "D"),
        }
    }
}

pub type Decimal = u8;
