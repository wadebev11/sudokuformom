use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::grid::{CELL_COUNT, Grid};
use crate::techniques::Difficulty;

/// Two cells per byte, so a puzzle takes 41 bytes.
const PACKED_PUZZLE_LEN: usize = CELL_COUNT.div_ceil(2);

/// A little-endian u16 puzzle count per Difficulty, in `Difficulty::ALL` order.
const HEADER_LEN: usize = 2 * Difficulty::ALL.len();

/// The pre-generated puzzles shipped with the app, grouped by Difficulty.
/// Puzzles stay packed in the bundled bytes and are only unpacked when played,
/// so a phone never holds thousands of decoded puzzles in memory.
#[derive(Clone, Copy, Debug)]
pub struct PuzzleBank<'a> {
    bytes: &'a [u8],
}

impl<'a> PuzzleBank<'a> {
    pub const fn new(bytes: &'a [u8]) -> Self {
        PuzzleBank { bytes }
    }

    /// Packs puzzles into the bytes `new` reads.
    pub fn pack(puzzles: &BTreeMap<Difficulty, Vec<Grid>>) -> Vec<u8> {
        let for_difficulty = |difficulty| puzzles.get(&difficulty).map_or(&[][..], Vec::as_slice);
        let counts = Difficulty::ALL.iter().flat_map(|difficulty| {
            let count = u16::try_from(for_difficulty(*difficulty).len())
                .expect("a Difficulty has at most 65535 puzzles");
            count.to_le_bytes()
        });
        let packed_puzzles = Difficulty::ALL
            .iter()
            .flat_map(|difficulty| for_difficulty(*difficulty))
            .flat_map(|puzzle| {
                (0..CELL_COUNT).step_by(2).map(|cell| {
                    let next = if cell + 1 < CELL_COUNT {
                        puzzle.get(cell + 1)
                    } else {
                        0
                    };
                    puzzle.get(cell) | next << 4
                })
            });
        counts.chain(packed_puzzles).collect()
    }

    pub fn count(&self, difficulty: Difficulty) -> usize {
        let position = 2 * difficulty as usize;
        usize::from(u16::from_le_bytes([
            self.bytes[position],
            self.bytes[position + 1],
        ]))
    }

    pub fn puzzle(&self, difficulty: Difficulty, index: usize) -> Grid {
        let puzzles_before: usize = Difficulty::ALL
            .iter()
            .take_while(|other| **other != difficulty)
            .map(|other| self.count(*other))
            .sum();
        let start = HEADER_LEN + (puzzles_before + index) * PACKED_PUZZLE_LEN;
        let packed = &self.bytes[start..start + PACKED_PUZZLE_LEN];
        let mut puzzle = Grid::empty();
        (0..CELL_COUNT).for_each(|cell| puzzle.set(cell, packed[cell / 2] >> (cell % 2 * 4) & 0xF));
        puzzle
    }

    pub fn puzzles(&self, difficulty: Difficulty) -> impl Iterator<Item = Grid> {
        (0..self.count(difficulty)).map(move |index| self.puzzle(difficulty, index))
    }
}

/// Which puzzles in the bank have been started, so new games don't repeat.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlayedPuzzles(BTreeMap<Difficulty, BTreeSet<usize>>);

impl PuzzleBank<'_> {
    /// Picks a puzzle that hasn't been played yet, starting over once every
    /// puzzle at that Difficulty has been played. `random` is in 0..1.
    pub fn pick_unplayed(
        &self,
        difficulty: Difficulty,
        played: &mut PlayedPuzzles,
        random: f64,
    ) -> Grid {
        let count = self.count(difficulty);
        let played_here = played.0.entry(difficulty).or_default();
        if played_here.len() >= count {
            played_here.clear();
        }
        let unplayed_count = count - played_here.len();
        let choice = (random * unplayed_count as f64) as usize % unplayed_count;
        let index = (0..count)
            .filter(|index| !played_here.contains(index))
            .nth(choice)
            .expect("choice is below the number of unplayed puzzles");
        played_here.insert(index);
        self.puzzle(difficulty, index)
    }
}
