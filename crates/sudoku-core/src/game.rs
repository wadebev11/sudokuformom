use serde::{Deserialize, Serialize};

use crate::grid::{CELL_COUNT, Cell, Digit, DigitSet, Grid, PEERS, digit_bit};
use crate::hint::Hint;
use crate::solver::solve;
use crate::techniques::Difficulty;

/// What the player has changed, so it can be undone.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
struct PlayerMarks {
    entries: Grid,
    #[serde(with = "pencil_mark_list")]
    pencil_marks: [DigitSet; CELL_COUNT],
}

/// Serde only derives arrays up to 32 long, so the 81 cells are saved as a list.
mod pencil_mark_list {
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use crate::grid::{CELL_COUNT, DigitSet};

    pub fn serialize<S: Serializer>(
        marks: &[DigitSet; CELL_COUNT],
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        marks.as_slice().serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<[DigitSet; CELL_COUNT], D::Error> {
        Vec::<DigitSet>::deserialize(deserializer)?
            .try_into()
            .map_err(|marks: Vec<DigitSet>| Error::invalid_length(marks.len(), &"81 cells"))
    }
}

/// One puzzle being played: its Givens, its solution and what the player has entered.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Game {
    difficulty: Difficulty,
    givens: Grid,
    solution: Grid,
    marks: PlayerMarks,
    #[serde(skip)]
    undo_history: Vec<PlayerMarks>,
    #[serde(skip)]
    redo_history: Vec<PlayerMarks>,
}

impl Game {
    pub fn new(puzzle: Grid, difficulty: Difficulty) -> Self {
        let solution = solve(&puzzle).expect("puzzles in the bank always have a solution");
        Game {
            difficulty,
            givens: puzzle,
            solution,
            marks: PlayerMarks {
                entries: Grid::empty(),
                pencil_marks: [0; CELL_COUNT],
            },
            undo_history: Vec::new(),
            redo_history: Vec::new(),
        }
    }

    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }

    pub fn is_given(&self, cell: Cell) -> bool {
        !self.givens.is_empty(cell)
    }

    fn is_editable(&self, cell: Cell) -> bool {
        !self.is_given(cell) && !self.is_solved()
    }

    /// The number showing in a cell, whether it's a Given or entered by the player.
    pub fn value(&self, cell: Cell) -> Option<Digit> {
        let digit = if self.is_given(cell) {
            self.givens.get(cell)
        } else {
            self.marks.entries.get(cell)
        };
        (digit != 0).then_some(digit)
    }

    pub fn pencil_marks(&self, cell: Cell) -> DigitSet {
        self.marks.pencil_marks[cell]
    }

    pub fn board(&self) -> Grid {
        let mut board = self.givens;
        (0..CELL_COUNT)
            .filter(|cell| !self.marks.entries.is_empty(*cell))
            .for_each(|cell| board.set(cell, self.marks.entries.get(cell)));
        board
    }

    fn record_for_undo(&mut self) {
        self.undo_history.push(self.marks);
        self.redo_history.clear();
    }

    /// Places a number and clears it from the Pencil Marks of every cell it rules out.
    pub fn enter(&mut self, cell: Cell, digit: Digit) {
        if self.is_editable(cell) && self.value(cell) != Some(digit) {
            self.record_for_undo();
            self.marks.entries.set(cell, digit);
            self.marks.pencil_marks[cell] = 0;
            for peer in &PEERS[cell] {
                self.marks.pencil_marks[*peer] &= !digit_bit(digit);
            }
        }
    }

    pub fn erase(&mut self, cell: Cell) {
        let has_marks = self.value(cell).is_some() || self.pencil_marks(cell) != 0;
        if self.is_editable(cell) && has_marks {
            self.record_for_undo();
            self.marks.entries.set(cell, 0);
            self.marks.pencil_marks[cell] = 0;
        }
    }

    pub fn toggle_pencil_mark(&mut self, cell: Cell, digit: Digit) {
        if self.is_editable(cell) && self.value(cell).is_none() {
            self.record_for_undo();
            self.marks.pencil_marks[cell] ^= digit_bit(digit);
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_history.is_empty() && !self.is_solved()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_history.is_empty()
    }

    pub fn undo(&mut self) {
        if !self.is_solved()
            && let Some(previous) = self.undo_history.pop()
        {
            self.redo_history
                .push(std::mem::replace(&mut self.marks, previous));
        }
    }

    pub fn redo(&mut self) {
        if let Some(next) = self.redo_history.pop() {
            self.undo_history
                .push(std::mem::replace(&mut self.marks, next));
        }
    }

    /// Clears a Mistake, places a number, or crosses Candidates out of the
    /// Pencil Marks, filling in a cell's marks first if it has none.
    pub fn apply_hint(&mut self, hint: &Hint) {
        match hint {
            Hint::Mistake(cell) => self.erase(*cell),
            Hint::Step(step) => match step.placement {
                Some((cell, digit)) => self.enter(cell, digit),
                None => {
                    self.record_for_undo();
                    let board = self.board();
                    for (cell, digit) in &step.eliminations {
                        let marks = &mut self.marks.pencil_marks[*cell];
                        if *marks == 0 {
                            *marks = board.candidates(*cell);
                        }
                        *marks &= !digit_bit(*digit);
                    }
                }
            },
        }
    }

    /// True once the player has entered anything, so starting over would lose work.
    pub fn has_progress(&self) -> bool {
        self.marks.entries.filled_count() > 0
            || self.marks.pencil_marks.iter().any(|marks| *marks != 0)
    }

    /// True once the number shows in nine cells, so there is nowhere left to put it.
    pub fn is_fully_placed(&self, digit: Digit) -> bool {
        (0..CELL_COUNT)
            .filter(|cell| self.value(*cell) == Some(digit))
            .count()
            == 9
    }

    pub fn is_mistake(&self, cell: Cell) -> bool {
        !self.marks.entries.is_empty(cell)
            && self.marks.entries.get(cell) != self.solution.get(cell)
    }

    pub fn is_solved(&self) -> bool {
        (0..CELL_COUNT).all(|cell| self.value(cell) == Some(self.solution.get(cell)))
    }
}
