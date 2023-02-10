use clap::Parser;
use witchspace::prelude::*;

/// Generate a traveller main world profile
#[derive(Parser)]
struct Cli {
    /// A seed for the PRNG
    #[arg(long)]
    seed: Option<String>,
}

fn main() {
    let args = Cli::parse();
    let galaxy: Galaxy = match args.seed {
        Some(seed) => Galaxy::new(seed),
        None => Galaxy::random(),
    };
    let sector = galaxy.get_sector(Coordinate::new(0, 0));
    println!("{sector}")
}
