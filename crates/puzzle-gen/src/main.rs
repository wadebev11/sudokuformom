use std::collections::BTreeMap;
use std::path::PathBuf;

use clap::Parser;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rayon::prelude::*;
use sudoku_core::{Difficulty, PuzzleBank};

/// Generates the puzzle bank that ships with the app.
#[derive(Parser)]
struct Arguments {
    /// Where to write the packed puzzle bank.
    #[arg(long, default_value = "crates/app/puzzles.bin")]
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
    /// Regenerate only these difficulties, keeping the rest of the existing bank.
    #[arg(long, value_delimiter = ',', value_parser = parse_difficulty)]
    only: Vec<Difficulty>,
}

fn parse_difficulty(text: &str) -> Result<Difficulty, String> {
    Difficulty::ALL
        .into_iter()
        .find(|difficulty| difficulty.name().eq_ignore_ascii_case(text))
        .ok_or_else(|| format!("expected one of easy, medium, hard, expert, got {text}"))
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
    let mut puzzles = BTreeMap::new();
    let difficulties = if arguments.only.is_empty() {
        Difficulty::ALL.to_vec()
    } else {
        let existing_bytes = std::fs::read(&arguments.out)?;
        let existing = PuzzleBank::new(&existing_bytes);
        for difficulty in Difficulty::ALL {
            puzzles.insert(difficulty, existing.puzzles(difficulty).collect());
        }
        arguments.only.clone()
    };
    for difficulty in difficulties {
        let wanted = arguments.count(difficulty);
        let found: &mut Vec<_> = puzzles.entry(difficulty).or_default();
        found.clear();
        let mut attempts = 0;
        while found.len() < wanted {
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
            let still_wanted = wanted - found.len();
            found.extend(batch.into_iter().flatten().take(still_wanted));
            eprint!("\r{}: {}/{wanted}", difficulty.name(), found.len());
        }
        eprintln!(" from {attempts} attempts");
    }
    std::fs::write(&arguments.out, PuzzleBank::pack(&puzzles))?;
    eprintln!("Wrote {}", arguments.out.display());
    Ok(())
}
