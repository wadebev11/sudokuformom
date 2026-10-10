use std::collections::{BTreeMap, BTreeSet};

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

    pub fn remove(&mut self, difficulty: Difficulty) {
        self.0.remove(&difficulty);
    }
}

/// Which puzzles in the bank have been started, so new games don't repeat.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlayedPuzzles(BTreeMap<Difficulty, BTreeSet<usize>>);

impl PuzzleBank {
    /// Picks a puzzle that hasn't been played yet, starting over once every
    /// puzzle at that Difficulty has been played. `random` is in 0..1.
    pub fn pick_unplayed(
        &self,
        difficulty: Difficulty,
        played: &mut PlayedPuzzles,
        random: f64,
    ) -> Grid {
        let puzzles = self.puzzles(difficulty);
        let played_here = played.0.entry(difficulty).or_default();
        if played_here.len() >= puzzles.len() {
            played_here.clear();
        }
        let unplayed: Vec<usize> = (0..puzzles.len())
            .filter(|index| !played_here.contains(index))
            .collect();
        let index = unplayed[(random * unplayed.len() as f64) as usize % unplayed.len()];
        played_here.insert(index);
        puzzles[index]
    }
}
