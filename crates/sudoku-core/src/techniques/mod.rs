mod explain;
mod fish;
mod intersections;
mod singles;
mod subsets;
mod wings;

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
    Pointing,
    BoxLineReduction,
    NakedPair,
    HiddenPair,
    NakedTriple,
    HiddenTriple,
    XWing,
    NakedQuad,
    HiddenQuad,
    XYWing,
    XYZWing,
    Swordfish,
}

impl Technique {
    pub fn difficulty(self) -> Difficulty {
        match self {
            Technique::HiddenSingle | Technique::NakedSingle => Difficulty::Easy,
            Technique::Pointing
            | Technique::BoxLineReduction
            | Technique::NakedPair
            | Technique::HiddenPair => Difficulty::Medium,
            Technique::NakedTriple
            | Technique::HiddenTriple
            | Technique::XWing
            | Technique::NakedQuad
            | Technique::HiddenQuad => Difficulty::Hard,
            Technique::XYWing | Technique::XYZWing | Technique::Swordfish => Difficulty::Expert,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Technique::HiddenSingle => "Hidden single",
            Technique::NakedSingle => "Naked single",
            Technique::Pointing => "Pointing",
            Technique::BoxLineReduction => "Box/line reduction",
            Technique::NakedPair => "Naked pair",
            Technique::HiddenPair => "Hidden pair",
            Technique::NakedTriple => "Naked triple",
            Technique::HiddenTriple => "Hidden triple",
            Technique::XWing => "X-Wing",
            Technique::NakedQuad => "Naked quad",
            Technique::HiddenQuad => "Hidden quad",
            Technique::XYWing => "XY-Wing",
            Technique::XYZWing => "XYZ-Wing",
            Technique::Swordfish => "Swordfish",
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

    pub(super) fn has_candidate(&self, cell: Cell, digit: Digit) -> bool {
        self.candidates[cell] & digit_bit(digit) != 0
    }

    /// Empty cells in the unit that can still hold the digit.
    pub(super) fn cells_for(&self, unit: &Unit, digit: Digit) -> Vec<Cell> {
        unit.cells()
            .into_iter()
            .filter(|cell| self.has_candidate(*cell, digit))
            .collect()
    }

    pub(super) fn empty_cells(&self) -> impl Iterator<Item = Cell> + '_ {
        (0..CELL_COUNT).filter(|cell| self.grid.is_empty(*cell))
    }

    /// Finds the simplest Technique that makes progress.
    pub fn next_step(&self) -> Option<Step> {
        let finders: [fn(&LogicBoard) -> Option<Step>; 14] = [
            singles::hidden_single,
            singles::naked_single,
            intersections::pointing,
            intersections::box_line_reduction,
            |board| subsets::naked_subset(board, 2),
            |board| subsets::hidden_subset(board, 2),
            |board| subsets::naked_subset(board, 3),
            |board| subsets::hidden_subset(board, 3),
            |board| fish::fish(board, 2),
            |board| subsets::naked_subset(board, 4),
            |board| subsets::hidden_subset(board, 4),
            wings::xy_wing,
            wings::xyz_wing,
            |board| fish::fish(board, 3),
        ];
        finders.iter().find_map(|find| find(self))
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
