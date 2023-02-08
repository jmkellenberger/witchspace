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
    let mut rng: Dice = match args.seed {
        Some(seed) => Dice::seed(seed),
        None => <Dice as rand::SeedableRng>::from_entropy(),
    };
    for world in 0..args.worlds {
        println!("World {}: {}", world + 1, System::generate(&mut rng));
    }
}
