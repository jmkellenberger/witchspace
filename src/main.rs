use clap::Parser;
use witchspace::prelude::*;

/// Roll a number of dice
#[derive(Parser)]
struct Cli {
    /// The number of six-sided dice to roll
    dice: usize,
    /// Number of sides on the dice
    sides: i32,
    /// A seed for the PRNG
    #[arg(long)]
    seed: Option<String>,
    /// Sum the results
    #[arg(short, long)]
    sum: bool,
}

fn main() {
    let args = Cli::parse();
    let mut rng: Pcg64 = match args.seed {
        Some(seed) => rng_from_seed(seed),
        None => rng(),
    };
    let results: String = if args.sum {
        rng.roll(args.dice, args.sides, 0).to_string()
    } else {
        rng.roll_dice(args.dice, args.sides)
            .iter()
            .map(|&id| id.to_string() + ", ")
            .collect()
    };

    println!("rolling {} dice, got: {}", args.dice, results);
    println!("Have a random UWP: {}", Uwp::generate_mainworld(&mut rng));
}
