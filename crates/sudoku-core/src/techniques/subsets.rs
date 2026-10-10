use itertools::Itertools;

use super::explain::{count_word, join};
use super::{LogicBoard, Step, Technique};
use crate::grid::{Cell, DigitSet, UNITS, digit_bit, digits_in};

fn technique(size: usize, naked: bool) -> Technique {
    match (size, naked) {
        (2, true) => Technique::NakedPair,
        (3, true) => Technique::NakedTriple,
        (4, true) => Technique::NakedQuad,
        (2, false) => Technique::HiddenPair,
        (3, false) => Technique::HiddenTriple,
        _ => Technique::HiddenQuad,
    }
}

/// `size` cells in a unit that between them can only hold `size` digits,
/// so those digits can't go anywhere else in the unit.
pub(super) fn naked_subset(board: &LogicBoard, size: usize) -> Option<Step> {
    UNITS.iter().find_map(|unit| {
        let open_cells: Vec<Cell> = unit
            .cells()
            .into_iter()
            .filter(|cell| board.candidates(*cell) != 0)
            .collect();
        open_cells.iter().copied().combinations(size).find_map(|subset| {
            let digits: DigitSet = subset.iter().fold(0, |union, cell| union | board.candidates(*cell));
            let eliminations: Vec<_> = if digits.count_ones() as usize == size {
                    open_cells
                        .iter()
                        .filter(|cell| !subset.contains(cell))
                        .flat_map(|cell| {
                            digits_in(board.candidates(*cell) & digits).map(move |digit| (*cell, digit))
                        })
                        .collect()
            } else {
                Vec::new()
            };
            (!eliminations.is_empty()).then(|| Step {
                technique: technique(size, true),
                placement: None,
                eliminations,
                explanation: format!(
                    "The {} highlighted cells in {unit} can only be {}. Between them they use up those numbers, so no other cell in {unit} can be {}.",
                    count_word(size),
                    join(digits_in(digits), "or"),
                    join(digits_in(digits), "or"),
                ),
                cells: subset,
            })
        })
    })
}

/// `size` digits that can only go in the same `size` cells of a unit,
/// so those cells can't hold any other digit.
pub(super) fn hidden_subset(board: &LogicBoard, size: usize) -> Option<Step> {
    UNITS.iter().find_map(|unit| {
        let open_digits: Vec<_> = (1..=9)
            .filter(|digit| !board.cells_for(unit, *digit).is_empty())
            .collect();
        open_digits.iter().copied().combinations(size).find_map(|digits| {
            let cells: Vec<Cell> = digits
                .iter()
                .flat_map(|digit| board.cells_for(unit, *digit))
                .unique()
                .sorted()
                .collect();
            let kept: DigitSet = digits.iter().fold(0, |set, digit| set | digit_bit(*digit));
            let eliminations: Vec<_> = if cells.len() == size {
                    cells
                        .iter()
                        .flat_map(|cell| digits_in(board.candidates(*cell) & !kept).map(move |digit| (*cell, digit)))
                        .collect()
            } else {
                Vec::new()
            };
            (!eliminations.is_empty()).then(|| Step {
                technique: technique(size, false),
                placement: None,
                eliminations,
                explanation: format!(
                    "In {unit}, the {} can only go in the {} highlighted cells, so those cells can't be anything else.",
                    join(&digits, "and"),
                    count_word(size),
                ),
                cells,
            })
        })
    })
}
