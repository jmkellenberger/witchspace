pub mod generators;
pub mod rng;
pub mod system;
pub mod world;

pub mod prelude {
    pub use crate::generators::*;
    pub use crate::rng::*;
    pub use crate::system::*;
    pub use crate::world::*;

    pub fn to_ehex(value: i32) -> String {
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
}
