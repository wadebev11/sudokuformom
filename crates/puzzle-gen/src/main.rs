use std::path::PathBuf;

use clap::Parser;
use rand::SeedableRng;
use rand::rngs::StdRng;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = Arguments::parse();
    let mut rng = StdRng::seed_from_u64(arguments.seed);
    let mut bank = PuzzleBank::default();
    for difficulty in Difficulty::ALL {
        let wanted = arguments.count(difficulty);
        let mut attempts = 0;
        while bank.puzzles(difficulty).len() < wanted {
            attempts += 1;
            if let Some(puzzle) = puzzle_gen::generate(difficulty, &mut rng) {
                bank.add(difficulty, puzzle);
            }
        }
        eprintln!(
            "{}: {wanted} puzzles from {attempts} attempts",
            difficulty.name()
        );
    }
    std::fs::write(&arguments.out, serde_json::to_string(&bank)?)?;
    eprintln!("Wrote {}", arguments.out.display());
    Ok(())
}
