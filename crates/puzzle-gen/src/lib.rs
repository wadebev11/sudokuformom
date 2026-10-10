//! Builds puzzles of a chosen Difficulty. Numbers are removed from a random
//! solved board for as long as the puzzle stays solvable with the known
//! Techniques, then Givens are added back until it eases to the target.
//! Carving straight to a target rarely lands on Hard, because removing one
//! more number usually jumps a puzzle from Medium to beyond Hard. Easier
//! puzzles then get more Givens back, since a sparse board feels harder than
//! the Techniques it needs.

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

fn shuffled_cells(rng: &mut impl Rng) -> Vec<Cell> {
    let mut cells: Vec<Cell> = (0..CELL_COUNT).collect();
    cells.shuffle(rng);
    cells
}

fn carve(solution: &Grid, rng: &mut impl Rng) -> Grid {
    let mut puzzle = *solution;
    for cell in shuffled_cells(rng) {
        puzzle.set(cell, 0);
        if grade(&puzzle).is_none() {
            puzzle.set(cell, solution.get(cell));
        }
    }
    puzzle
}

fn ease(puzzle: &mut Grid, solution: &Grid, target: Difficulty, rng: &mut impl Rng) {
    for cell in shuffled_cells(rng) {
        if grade(puzzle).is_some_and(|difficulty| difficulty > target) {
            puzzle.set(cell, solution.get(cell));
        }
    }
}

pub fn minimum_givens(difficulty: Difficulty) -> usize {
    match difficulty {
        Difficulty::Easy => 36,
        Difficulty::Medium => 30,
        Difficulty::Hard | Difficulty::Expert => 0,
    }
}

fn add_givens_to_minimum(
    puzzle: &mut Grid,
    solution: &Grid,
    target: Difficulty,
    rng: &mut impl Rng,
) {
    let minimum = minimum_givens(target);
    for cell in shuffled_cells(rng) {
        if puzzle.filled_count() < minimum && puzzle.is_empty(cell) {
            puzzle.set(cell, solution.get(cell));
            if grade(puzzle) != Some(target) {
                puzzle.set(cell, 0);
            }
        }
    }
}

/// One attempt at a puzzle of the target Difficulty. Returns None when the
/// attempt ends up easier than the target or can't reach its minimum Givens.
pub fn generate(target: Difficulty, rng: &mut impl Rng) -> Option<Grid> {
    let solution = random_solution(rng);
    let mut puzzle = carve(&solution, rng);
    ease(&mut puzzle, &solution, target, rng);
    add_givens_to_minimum(&mut puzzle, &solution, target, rng);
    let is_wanted = grade(&puzzle) == Some(target)
        && puzzle.filled_count() >= minimum_givens(target)
        && has_unique_solution(&puzzle);
    is_wanted.then_some(puzzle)
}
