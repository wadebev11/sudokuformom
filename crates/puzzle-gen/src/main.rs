use std::path::PathBuf;

use clap::Parser;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rayon::prelude::*;
use sudoku_core::{Difficulty, PuzzleBank};

/// Generates the puzzle bank that ships with the app.
#[derive(Parser)]
struct Arguments {
    /// Where to write the puzzle bank JSON.
    #[arg(long, default_value = "crates/app/puzzles.json")]
    out: PathBuf,
    /// Seed for the random generator, so runs are reproducible.
    #[arg(long, default_value_t = 1)]
    seed: u64,
    #[arg(long, default_value_t = 750)]
    easy: usize,
    #[arg(long, default_value_t = 1750)]
    medium: usize,
    #[arg(long, default_value_t = 1750)]
    hard: usize,
    #[arg(long, default_value_t = 750)]
    expert: usize,
}

impl Arguments {
    fn count(&self, difficulty: Difficulty) -> usize {
        match difficulty {
            Difficulty::Easy => self.easy,
            Difficulty::Medium => self.medium,
            Difficulty::Hard => self.hard,
            Difficulty::Expert => self.expert,
        }
    }
}

const ATTEMPTS_PER_BATCH: u64 = 256;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = Arguments::parse();
    let mut bank = PuzzleBank::default();
    for difficulty in Difficulty::ALL {
        let wanted = arguments.count(difficulty);
        let mut attempts = 0;
        while bank.puzzles(difficulty).len() < wanted {
            // Each attempt gets its own seed, so the output doesn't depend on thread scheduling.
            let batch: Vec<_> = (attempts..attempts + ATTEMPTS_PER_BATCH)
                .into_par_iter()
                .map(|attempt| {
                    let mut rng =
                        StdRng::seed_from_u64(arguments.seed ^ (attempt << 8) ^ difficulty as u64);
                    puzzle_gen::generate(difficulty, &mut rng)
                })
                .collect();
            attempts += ATTEMPTS_PER_BATCH;
            batch
                .into_iter()
                .flatten()
                .take(wanted - bank.puzzles(difficulty).len())
                .for_each(|puzzle| bank.add(difficulty, puzzle));
            eprint!(
                "\r{}: {}/{wanted}",
                difficulty.name(),
                bank.puzzles(difficulty).len()
            );
        }
        eprintln!(" from {attempts} attempts");
    }
    std::fs::write(&arguments.out, serde_json::to_string(&bank)?)?;
    eprintln!("Wrote {}", arguments.out.display());
    Ok(())
}
