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
    /// From simplest to hardest, the order Hints try them in.
    pub const ALL: [Technique; 14] = [
        Technique::HiddenSingle,
        Technique::NakedSingle,
        Technique::Pointing,
        Technique::BoxLineReduction,
        Technique::NakedPair,
        Technique::HiddenPair,
        Technique::NakedTriple,
        Technique::HiddenTriple,
        Technique::XWing,
        Technique::NakedQuad,
        Technique::HiddenQuad,
        Technique::XYWing,
        Technique::XYZWing,
        Technique::Swordfish,
    ];

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

    /// How the Technique works, in plain words for the player.
    pub fn description(self) -> &'static str {
        match self {
            Technique::HiddenSingle => {
                "Pick a number and a row, column or box. If every cell but one is ruled out for that number, it must go in that one cell."
            }
            Technique::NakedSingle => {
                "Look at one empty cell. If its row, column and box already hold eight different numbers, the ninth is the only one that fits."
            }
            Technique::Pointing => {
                "Inside a box, if a number can only go in cells that share one row, the box's copy of that number must be in that row. So the number can't go anywhere else in that row. The same works for a column."
            }
            Technique::BoxLineReduction => {
                "The reverse of Pointing. Inside a row, if a number can only go in cells that all sit in one box, the row's copy of that number must be in that box. So the number can't go anywhere else in that box. The same works for a column."
            }
            Technique::NakedPair => {
                "Two cells in the same row, column or box can each only be the same two numbers. Those two numbers will fill those two cells, so you can cross them out of every other cell in that row, column or box."
            }
            Technique::HiddenPair => {
                "In a row, column or box, two numbers can only go in the same two cells. Those cells must hold those two numbers, so you can cross every other number out of them."
            }
            Technique::NakedTriple => {
                "Like a Naked pair with three cells. Three cells in one row, column or box can only be the same three numbers between them, so those numbers can't go anywhere else there."
            }
            Technique::HiddenTriple => {
                "Like a Hidden pair with three numbers. In one row, column or box, three numbers can only go in the same three cells, so those cells can't hold anything else."
            }
            Technique::XWing => {
                "In two rows, a number can only go in two cells each, and those cells line up in the same two columns, making the corners of a rectangle. Whichever way it goes, the number takes one corner in each of those columns. So you can cross it out of the rest of both columns. It also works with rows and columns swapped."
            }
            Technique::NakedQuad => {
                "Like a Naked pair with four cells. Four cells in one row, column or box can only be the same four numbers between them, so those numbers can't go anywhere else there."
            }
            Technique::HiddenQuad => {
                "Like a Hidden pair with four numbers. In one row, column or box, four numbers can only go in the same four cells, so those cells can't hold anything else."
            }
            Technique::XYWing => {
                "Three cells, each with only two options. The middle cell (say it's 1 or 2) sees the other two, which are 1 or 3 and 2 or 3. Whichever number the middle cell turns out to be, one of the other two must be 3. So any cell that sees both of them can't be 3."
            }
            Technique::XYZWing => {
                "Like an XY-Wing, but the middle cell has three options (say 1, 2 or 3) and the other two cells are 1 or 3 and 2 or 3. One of the three cells must be 3, so any cell that sees all three can't be 3."
            }
            Technique::Swordfish => {
                "An X-Wing with three rows and three columns. In three rows, a number can only go in cells that sit in the same three columns. The number must take one cell in each of those columns, so you can cross it out of the rest of those columns."
            }
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
