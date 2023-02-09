use clap::Parser;
use witchspace::prelude::*;

/// Generate a traveller main world profile
#[derive(Parser)]
struct Cli {
    /// The number of worlds to generate
    worlds: usize,
    /// A seed for the PRNG
    #[arg(long)]
    seed: Option<String>,
}

fn main() {
    let args = Cli::parse();
    let seed: Seed = match args.seed {
        Some(seed) => Seed::new(seed, Coordinate::new(0, 0)),
        None => Seed::random(),
    };
    for world in 0..args.worlds {
        let subseed = seed.subseed(Coordinate::new(world as i32, world as i32));
        println!("World {}: {}", world + 1, generate_system(subseed));
    }
}
