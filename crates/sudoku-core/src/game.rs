use serde::{Deserialize, Serialize};

use crate::grid::{CELL_COUNT, Cell, Digit, Grid};
use crate::solver::solve;
use crate::techniques::Difficulty;

/// One puzzle being played: its Givens, its solution and what the player has entered.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Game {
    difficulty: Difficulty,
    givens: Grid,
    solution: Grid,
    entries: Grid,
}

impl Game {
    pub fn new(puzzle: Grid, difficulty: Difficulty) -> Self {
        let solution = solve(&puzzle).expect("puzzles in the bank always have a solution");
        Game {
            difficulty,
            givens: puzzle,
            solution,
            entries: Grid::empty(),
        }
    }

    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }

    pub fn is_given(&self, cell: Cell) -> bool {
        !self.givens.is_empty(cell)
    }

    /// The number showing in a cell, whether it's a Given or entered by the player.
    pub fn value(&self, cell: Cell) -> Option<Digit> {
        let digit = if self.is_given(cell) {
            self.givens.get(cell)
        } else {
            self.entries.get(cell)
        };
        (digit != 0).then_some(digit)
    }

    pub fn board(&self) -> Grid {
        let mut board = self.givens;
        (0..CELL_COUNT)
            .filter(|cell| !self.entries.is_empty(*cell))
            .for_each(|cell| board.set(cell, self.entries.get(cell)));
        board
    }

    pub fn enter(&mut self, cell: Cell, digit: Digit) {
        if !self.is_given(cell) && !self.is_solved() {
            self.entries.set(cell, digit);
        }
    }

    pub fn erase(&mut self, cell: Cell) {
        if !self.is_solved() {
            self.entries.set(cell, 0);
        }
    }

    pub fn is_mistake(&self, cell: Cell) -> bool {
        !self.entries.is_empty(cell) && self.entries.get(cell) != self.solution.get(cell)
    }

    pub fn is_solved(&self) -> bool {
        self.board() == self.solution
    }
}
