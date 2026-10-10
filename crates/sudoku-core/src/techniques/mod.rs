mod singles;

use serde::{Deserialize, Serialize};

use crate::grid::{CELL_COUNT, Cell, Digit, DigitSet, Grid, PEERS, Unit, digit_bit};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

impl Difficulty {
    pub const ALL: [Difficulty; 4] = [
        Difficulty::Easy,
        Difficulty::Medium,
        Difficulty::Hard,
        Difficulty::Expert,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
            Difficulty::Expert => "Expert",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Technique {
    HiddenSingle,
    NakedSingle,
}

impl Technique {
    pub fn difficulty(self) -> Difficulty {
        match self {
            Technique::HiddenSingle | Technique::NakedSingle => Difficulty::Easy,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Technique::HiddenSingle => "Hidden single",
            Technique::NakedSingle => "Naked single",
        }
    }
}

/// One logical deduction: either a number to place or Candidates to remove.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Step {
    pub technique: Technique,
    pub placement: Option<(Cell, Digit)>,
    pub eliminations: Vec<(Cell, Digit)>,
    /// Cells that make up the pattern, for highlighting.
    pub cells: Vec<Cell>,
    pub explanation: String,
}

/// Placed numbers plus the Candidates still open in each empty cell.
#[derive(Clone, Debug)]
pub struct LogicBoard {
    grid: Grid,
    candidates: [DigitSet; CELL_COUNT],
}

impl LogicBoard {
    pub fn new(grid: &Grid) -> Self {
        let candidates = std::array::from_fn(|cell| {
            if grid.is_empty(cell) {
                grid.candidates(cell)
            } else {
                0
            }
        });
        LogicBoard {
            grid: *grid,
            candidates,
        }
    }

    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    pub fn candidates(&self, cell: Cell) -> DigitSet {
        self.candidates[cell]
    }

    fn has_candidate(&self, cell: Cell, digit: Digit) -> bool {
        self.candidates[cell] & digit_bit(digit) != 0
    }

    /// Empty cells in the unit that can still hold the digit.
    fn cells_for(&self, unit: &Unit, digit: Digit) -> Vec<Cell> {
        unit.cells()
            .into_iter()
            .filter(|cell| self.has_candidate(*cell, digit))
            .collect()
    }

    fn empty_cells(&self) -> impl Iterator<Item = Cell> + '_ {
        (0..CELL_COUNT).filter(|cell| self.grid.is_empty(*cell))
    }

    /// Finds the simplest Technique that makes progress.
    pub fn next_step(&self) -> Option<Step> {
        singles::hidden_single(self).or_else(|| singles::naked_single(self))
    }

    pub fn apply(&mut self, step: &Step) {
        if let Some((cell, digit)) = step.placement {
            self.place(cell, digit);
        }
        for (cell, digit) in &step.eliminations {
            self.candidates[*cell] &= !digit_bit(*digit);
        }
    }

    fn place(&mut self, cell: Cell, digit: Digit) {
        self.grid.set(cell, digit);
        self.candidates[cell] = 0;
        for peer in &PEERS[cell] {
            self.candidates[*peer] &= !digit_bit(digit);
        }
    }

    pub fn is_solved(&self) -> bool {
        self.grid.is_complete()
    }
}

/// The Difficulty of a puzzle, or None when it can't be solved with the known Techniques.
pub fn grade(puzzle: &Grid) -> Option<Difficulty> {
    let mut board = LogicBoard::new(puzzle);
    let mut hardest = Difficulty::Easy;
    let mut stuck = false;
    while !board.is_solved() && !stuck {
        match board.next_step() {
            Some(step) => {
                hardest = hardest.max(step.technique.difficulty());
                board.apply(&step);
            }
            None => stuck = true,
        }
    }
    board.is_solved().then_some(hardest)
}
