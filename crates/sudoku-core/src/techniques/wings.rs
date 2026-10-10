use super::{LogicBoard, Step, Technique};
use crate::grid::{CELL_COUNT, Cell, Digit, DigitSet, PEERS, cell_name, digits_in, sees};

fn cells_seeing_all(board: &LogicBoard, pattern: &[Cell], digit: Digit) -> Vec<(Cell, Digit)> {
    (0..CELL_COUNT)
        .filter(|cell| !pattern.contains(cell))
        .filter(|cell| pattern.iter().all(|other| sees(*cell, *other)))
        .filter(|cell| board.has_candidate(*cell, digit))
        .map(|cell| (cell, digit))
        .collect()
}

fn single_digit(set: DigitSet) -> Option<Digit> {
    (set.count_ones() == 1)
        .then(|| digits_in(set).next())
        .flatten()
}

/// Pivot holding {x, y} sees one pincer with {x, z} and another with {y, z}.
/// Whichever the pivot is, one pincer becomes z, so z goes from any cell
/// that sees both pincers.
pub(super) fn xy_wing(board: &LogicBoard) -> Option<Step> {
    wing(board, 2, Technique::XYWing, false)
}

/// Like XY-Wing, but the pivot also holds z, so the eliminated cells must
/// see the pivot as well.
pub(super) fn xyz_wing(board: &LogicBoard) -> Option<Step> {
    wing(board, 3, Technique::XYZWing, true)
}

fn wing(
    board: &LogicBoard,
    pivot_size: u32,
    technique: Technique,
    pivot_holds_z: bool,
) -> Option<Step> {
    board
        .empty_cells()
        .filter(|cell| board.candidates(*cell).count_ones() == pivot_size)
        .find_map(|pivot| {
            let pivot_digits = board.candidates(pivot);
            let pincers: Vec<Cell> = PEERS[pivot]
                .iter()
                .copied()
                .filter(|cell| board.candidates(*cell).count_ones() == 2)
                .filter(|cell| {
                    (board.candidates(*cell) & pivot_digits).count_ones()
                        == if pivot_holds_z { 2 } else { 1 }
                })
                .collect();
            pincers.iter().enumerate().find_map(|(position, first)| {
                pincers[position + 1..].iter().find_map(|second| {
                    let first_digits = board.candidates(*first);
                    let second_digits = board.candidates(*second);
                    let z = single_digit(first_digits & second_digits)?;
                    let z_set = 1 << z;
                    let wing_shape = if pivot_holds_z {
                        pivot_digits & z_set != 0
                            && (first_digits | second_digits) == pivot_digits
                            && first_digits != second_digits
                    } else {
                        pivot_digits & z_set == 0
                            && (first_digits | second_digits) & !z_set == pivot_digits
                    };
                    let pattern: Vec<Cell> = if pivot_holds_z {
                        vec![pivot, *first, *second]
                    } else {
                        vec![*first, *second]
                    };
                    let eliminations = if wing_shape {
                        cells_seeing_all(board, &pattern, z)
                    } else {
                        Vec::new()
                    };
                    (!eliminations.is_empty()).then(|| Step {
                        technique,
                        placement: None,
                        eliminations,
                        explanation: wing_explanation(technique, pivot, *first, *second, z),
                        cells: vec![pivot, *first, *second],
                    })
                })
            })
        })
}

fn wing_explanation(
    technique: Technique,
    pivot: Cell,
    first: Cell,
    second: Cell,
    z: Digit,
) -> String {
    match technique {
        Technique::XYWing => format!(
            "The cell at {} has two options, and each one forces a {z} into one of the other highlighted cells ({} or {}). One of those two must be {z}, so the {z} can't go in any cell that sees both.",
            cell_name(pivot),
            cell_name(first),
            cell_name(second),
        ),
        _ => format!(
            "One of the three highlighted cells must be {z}, whichever option the cell at {} turns out to be. So the {z} can't go in any cell that sees all three.",
            cell_name(pivot),
        ),
    }
}
