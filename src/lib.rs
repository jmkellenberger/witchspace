pub mod entities;
pub mod generators;

pub mod prelude {
    pub use crate::entities::*;
    pub use crate::generators::{generate_sector, generate_system};
}
