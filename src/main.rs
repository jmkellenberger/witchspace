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
    let mut rng: Pcg64 = match args.seed {
        Some(seed) => rng_from_seed(seed),
        None => rng(),
    };
    for world in 0..args.worlds {
        println!("World {}: {}", world + 1, Uwp::generate_mainworld(&mut rng));
    }
}
