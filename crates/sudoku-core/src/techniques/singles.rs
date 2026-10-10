use super::{LogicBoard, Step, Technique};
use crate::grid::{UNITS, UnitKind, cell_name, digits_in};

/// A digit that has only one possible cell left in a row, column or box.
/// Boxes are checked first because they are the easiest for people to spot.
pub(super) fn hidden_single(board: &LogicBoard) -> Option<Step> {
    let mut units: Vec<_> = UNITS.iter().collect();
    units.sort_by_key(|unit| unit.kind != UnitKind::Box);
    units.into_iter().find_map(|unit| {
        (1..=9).find_map(|digit| match board.cells_for(unit, digit).as_slice() {
            [cell] => Some(Step {
                technique: Technique::HiddenSingle,
                placement: Some((*cell, digit)),
                eliminations: Vec::new(),
                cells: unit.cells().to_vec(),
                explanation: format!(
                    "In {unit}, the {digit} can only go in one place: {}.",
                    cell_name(*cell)
                ),
            }),
            _ => None,
        })
    })
}

/// A cell where every number but one is already used by its row, column or box.
pub(super) fn naked_single(board: &LogicBoard) -> Option<Step> {
    board.empty_cells().find_map(|cell| {
        let candidates: Vec<_> = digits_in(board.candidates(cell)).collect();
        match candidates.as_slice() {
            [digit] => Some(Step {
                technique: Technique::NakedSingle,
                placement: Some((cell, *digit)),
                eliminations: Vec::new(),
                cells: vec![cell],
                explanation: format!(
                    "The only number that fits in {} is {digit}. Every other number is already in its row, column or box.",
                    cell_name(cell)
                ),
            }),
            _ => None,
        }
    })
}
