pub mod rng;
pub mod uwp;

pub mod prelude {
    pub use crate::rng::*;
    pub use crate::uwp::*;
}
