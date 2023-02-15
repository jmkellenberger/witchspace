use crate::prelude::*;

fn star_present(flux: i32) -> bool {
    flux > 2
}

fn spectral_class(flux: i32, aux_roll: i32) -> Class {
    match flux {
        -6 => {
            if aux_roll > 3 {
                Class::O
            } else {
                Class::B
            }
        }
        -5 | -4 => Class::A,
        -3 | -2 => Class::F,
        -1 | 0 => Class::G,
        1 | 2 => Class::K,
        3..=5 => Class::M,
        _ => Class::BD,
    }
}

fn o_class_sizes(flux: i32) -> Size {
    match flux {
        -6 | -5 => Size::Ia,
        -4 => Size::Ib,
        -3 => Size::II,
        -2..=0 => Size::III,
        1..=3 => Size::V,
        5 => Size::D,
        _ => Size::IV,
    }
}

fn b_class_sizes(flux: i32) -> Size {
    match flux {
        -6 | -5 => Size::Ia,
        -4 => Size::Ib,
        -3 => Size::II,
        -2..=1 => Size::III,
        2 | 3 => Size::V,
        5 => Size::D,
        _ => Size::IV,
    }
}

fn a_class_sizes(flux: i32) -> Size {
    match flux {
        -6 | -5 => Size::Ia,
        -4 => Size::Ib,
        -3 => Size::II,
        -2 => Size::III,
        -1 => Size::IV,
        5 => Size::D,
        _ => Size::V,
    }
}

fn fgk_class_sizes(flux: i32) -> Size {
    match flux {
        -6 | -5 => Size::II,
        -4 => Size::III,
        -3 => Size::IV,
        -2..=3 => Size::V,
        5 => Size::D,
        _ => Size::VI,
    }
}

fn m_class_sizes(flux: i32) -> Size {
    match flux {
        -6..=-3 => Size::II,
        -2 => Size::III,
        -1..=3 => Size::V,
        5 => Size::D,
        _ => Size::VI,
    }
}

fn class_to_size(class: &Class, lum: u8, flux: i32) -> Option<Size> {
    let size = match class {
        Class::O => Some(o_class_sizes(flux)),
        Class::B => Some(b_class_sizes(flux)),
        Class::A => Some(a_class_sizes(flux)),
        Class::F | Class::G | Class::K => Some(fgk_class_sizes(flux)),
        Class::M => Some(m_class_sizes(flux)),
        Class::BD => None,
    };

    // Size IV not for K5-K9. Size VI not for F0-F4.

    match (class, size, lum) {
        (Class::K, Some(Size::IV), 5..=9) => Some(Size::V),
        (Class::F, Some(Size::VI), 0..=4) => Some(Size::V),
        _ => size,
    }
}

fn generate_star(class_flux: i32, class_d6: i32, decimal: u8, size_flux: i32) -> Star {
    let class = spectral_class(class_flux, class_d6);
    let lum = decimal;
    let size = class_to_size(&class, lum, size_flux);

    match (class, size) {
        (class, Some(Size::D)) => Star::Dwarf(class),
        (Class::BD, _) => Star::BrownDwarf,
        (class, Some(size)) => Star::Star(class, lum, size),
        (class, None) => panic!("Class {} star with no size!", class),
    }
}

fn generate_companion<R: Rollable>(
    primary_spectral_flux: i32,
    primary_size_flux: i32,
    rng: &mut R,
) -> Option<Star> {
    if star_present(rng.flux(0)) {
        Some(generate_star(
            rng.roll(1, 6, primary_spectral_flux - 1),
            rng.roll(1, 6, 0),
            rng.roll(1, 10, -1) as u8,
            rng.roll(1, 6, primary_size_flux + 2),
        ))
    } else {
        None
    }
}

pub fn generate_stars<R: Rollable>(rng: &mut R) -> Vec<Star> {
    let primary_spectral_flux = rng.flux(0);
    let primary_size_flux = rng.flux(0);
    let primary = generate_star(
        primary_spectral_flux,
        rng.roll(1, 6, 0),
        rng.roll(1, 10, -1) as u8,
        primary_size_flux,
    );

    let mut stars: Vec<Star> = (0..7)
        .filter_map(|_| generate_companion(primary_spectral_flux, primary_size_flux, rng))
        .collect();

    stars.push(primary);
    stars.sort_unstable_by(|a, b| match (a, b) {
        (Star::Star(c1, l1, s1), Star::Star(c2, l2, s2)) => {
            if c1 == c2 {
                if s1 == s2 {
                    l2.cmp(l1).reverse()
                } else {
                    s1.cmp(&s2)
                }
            } else {
                a.cmp(&b)
            }
        }
        (_, _) => a.cmp(&b),
    });

    return stars;
}
