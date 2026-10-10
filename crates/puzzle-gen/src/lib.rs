//! Builds puzzles of a chosen Difficulty by removing numbers from a random
//! solved board for as long as the puzzle stays at or below that Difficulty.

use rand::Rng;
use rand::seq::SliceRandom;
use sudoku_core::grid::{CELL_COUNT, Cell, Digit, Unit, UnitKind};
use sudoku_core::solver::{has_unique_solution, solve};
use sudoku_core::{Difficulty, Grid, grade};

pub fn random_solution(rng: &mut impl Rng) -> Grid {
    // The three boxes on the diagonal share no row or column, so any shuffle of each is valid.
    let mut grid = Grid::empty();
    for index in [0, 4, 8] {
        let mut digits: Vec<Digit> = (1..=9).collect();
        digits.shuffle(rng);
        let cells = Unit {
            kind: UnitKind::Box,
            index,
        }
        .cells();
        cells
            .iter()
            .zip(digits)
            .for_each(|(cell, digit)| grid.set(*cell, digit));
    }
    solve(&grid).expect("a grid with only the diagonal boxes filled always has a solution")
}

/// One attempt at a puzzle of the target Difficulty. Returns None when the
/// attempt ends up easier than the target.
pub fn generate(target: Difficulty, rng: &mut impl Rng) -> Option<Grid> {
    let mut puzzle = random_solution(rng);
    let mut cells: Vec<Cell> = (0..CELL_COUNT).collect();
    cells.shuffle(rng);
    for cell in cells {
        let digit = puzzle.get(cell);
        puzzle.set(cell, 0);
        let still_within_target = grade(&puzzle).is_some_and(|difficulty| difficulty <= target);
        if !still_within_target {
            puzzle.set(cell, digit);
        }
    }
    (grade(&puzzle) == Some(target) && has_unique_solution(&puzzle)).then_some(puzzle)
}
