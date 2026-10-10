use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::grid::Grid;
use crate::techniques::Difficulty;

/// The pre-generated puzzles shipped with the app, grouped by Difficulty.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PuzzleBank(BTreeMap<Difficulty, Vec<Grid>>);

impl PuzzleBank {
    pub fn puzzles(&self, difficulty: Difficulty) -> &[Grid] {
        self.0.get(&difficulty).map_or(&[], Vec::as_slice)
    }

    pub fn add(&mut self, difficulty: Difficulty, puzzle: Grid) {
        self.0.entry(difficulty).or_default().push(puzzle);
    }
}
