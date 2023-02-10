pub mod coordinate;
pub use coordinate::*;
pub mod extensions;
pub use extensions::*;
pub mod sector;
pub use sector::*;
pub mod star;
pub use star::*;
pub mod system;
pub use system::*;
pub mod world;
pub use world::*;
pub mod rng;
pub use rng::*;
pub mod galaxy;
pub use galaxy::*;

fn to_ehex(value: i32) -> String {
    match value {
        0..=9 => value.to_string(),
        10..=33 => ehex(value),
        _ => format!("({value})?"),
    }
}

fn ehex(value: i32) -> String {
    let chars = (b'A'..=b'Z')
        .filter_map(|c| {
            let c = c as char;
            if c.is_alphabetic() && c != 'O' && c != 'I' {
                Some(c)
            } else {
                None
            }
        })
        .collect::<Vec<char>>();

    String::from(chars[(value - 10) as usize])
}
