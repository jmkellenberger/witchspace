use clap::Parser;
use witchspace::prelude::*;

/// Generate a traveller main world profile
#[derive(Parser)]
struct Cli {
    /// The number of rows in the sector
    rows: u32,
    /// The number of columns in the sector
    cols: u32,
    /// A seed for the PRNG
    #[arg(long)]
    seed: Option<String>,
}

fn main() {
    let args = Cli::parse();
    let seed: Seed = match args.seed {
        Some(seed) => Seed::new(seed, vec![1, 2, 3]),
        None => Seed::random(),
    };
    let sector = generate_sector(seed, args.rows, args.cols);
    println!("{sector}")
}
