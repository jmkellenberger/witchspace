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
