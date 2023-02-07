mod rng;
use rng::{rng_from_seed, Pcg64, Rollable};

fn main() {
    let seed = "Aamidal";
    let mut rng1: Pcg64 = rng_from_seed(seed);
    println!("{}, {}", rng1.roll(10), rng1.flux());
}
