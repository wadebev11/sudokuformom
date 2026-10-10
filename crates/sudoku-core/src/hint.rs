use crate::game::Game;
use crate::grid::{CELL_COUNT, Cell, cell_name, digit_bit};
use crate::techniques::{LogicBoard, Step};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Hint {
    /// A number the player entered that doesn't match the solution.
    Mistake(Cell),
    Step(Step),
}

impl Hint {
    pub fn cells(&self) -> Vec<Cell> {
        match self {
            Hint::Mistake(cell) => vec![*cell],
            Hint::Step(step) => step.cells.clone(),
        }
    }

    /// Cells the Hint changes when applied.
    pub fn target_cells(&self) -> Vec<Cell> {
        match self {
            Hint::Mistake(cell) => vec![*cell],
            Hint::Step(step) => step
                .placement
                .iter()
                .chain(&step.eliminations)
                .map(|(cell, _)| *cell)
                .collect(),
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Hint::Mistake(_) => "Mistake",
            Hint::Step(step) => step.technique.name(),
        }
    }

    pub fn explanation(&self) -> String {
        match self {
            Hint::Mistake(cell) => format!("The number in {} isn't right.", cell_name(*cell)),
            Hint::Step(step) => step.explanation.clone(),
        }
    }

    pub fn places_a_number(&self) -> bool {
        matches!(self, Hint::Step(step) if step.placement.is_some())
    }
}

impl Game {
    /// Finds the simplest next step from the numbers on the board, ignoring
    /// Pencil Marks as evidence. Steps that only remove Candidates the player
    /// has already crossed out are skipped, so repeated Hints keep moving forward.
    pub fn hint(&self) -> Option<Hint> {
        match (0..CELL_COUNT).find(|cell| self.is_mistake(*cell)) {
            Some(cell) => Some(Hint::Mistake(cell)),
            None => self.next_unseen_step().map(Hint::Step),
        }
    }

    fn next_unseen_step(&self) -> Option<Step> {
        let mut board = LogicBoard::new(&self.board());
        let mut found = None;
        while found.is_none()
            && let Some(step) = board.next_step()
        {
            if self.already_reflects(&step) {
                board.apply(&step);
            } else {
                found = Some(step);
            }
        }
        found
    }

    fn already_reflects(&self, step: &Step) -> bool {
        step.placement.is_none()
            && step.eliminations.iter().all(|(cell, digit)| {
                let marks = self.pencil_marks(*cell);
                marks != 0 && marks & digit_bit(*digit) == 0
            })
    }
}
