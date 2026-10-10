use super::{LogicBoard, Step, Technique};
use crate::grid::{UNITS, Unit, UnitKind, box_of, column_of, row_of};

fn line_through(cells: &[usize]) -> Option<Unit> {
    let first = *cells.first()?;
    let same_row = cells.iter().all(|cell| row_of(*cell) == row_of(first));
    let same_column = cells
        .iter()
        .all(|cell| column_of(*cell) == column_of(first));
    match (same_row, same_column) {
        (true, _) => Some(Unit {
            kind: UnitKind::Row,
            index: row_of(first),
        }),
        (_, true) => Some(Unit {
            kind: UnitKind::Column,
            index: column_of(first),
        }),
        _ => None,
    }
}

/// Within a box, a digit confined to one row or column can't appear in the
/// rest of that row or column.
pub(super) fn pointing(board: &LogicBoard) -> Option<Step> {
    UNITS.iter().filter(|unit| unit.kind == UnitKind::Box).find_map(|box_unit| {
        (1..=9).find_map(|digit| {
            let cells = board.cells_for(box_unit, digit);
            let line = line_through(&cells)?;
            let eliminations: Vec<_> = board
                .cells_for(&line, digit)
                .into_iter()
                .filter(|cell| box_of(*cell) != box_unit.index)
                .map(|cell| (cell, digit))
                .collect();
            (!eliminations.is_empty()).then(|| Step {
                technique: Technique::Pointing,
                placement: None,
                eliminations,
                explanation: format!(
                    "In {box_unit}, the {digit} can only go in the highlighted cells, which are all in {line}. So the {digit} can't go anywhere else in {line}."
                ),
                cells,
            })
        })
    })
}

/// Within a row or column, a digit confined to one box can't appear in the
/// rest of that box.
pub(super) fn box_line_reduction(board: &LogicBoard) -> Option<Step> {
    UNITS.iter().filter(|unit| unit.kind != UnitKind::Box).find_map(|line| {
        (1..=9).find_map(|digit| {
            let cells = board.cells_for(line, digit);
            let first = *cells.first()?;
            let box_unit = Unit { kind: UnitKind::Box, index: box_of(first) };
            let eliminations: Vec<_> = if cells.iter().all(|cell| box_of(*cell) == box_unit.index) {
                    board
                        .cells_for(&box_unit, digit)
                        .into_iter()
                        .filter(|cell| !cells.contains(cell))
                        .map(|cell| (cell, digit))
                        .collect()
            } else {
                Vec::new()
            };
            (!eliminations.is_empty()).then(|| Step {
                technique: Technique::BoxLineReduction,
                placement: None,
                eliminations,
                explanation: format!(
                    "In {line}, the {digit} can only go in the highlighted cells, which are all in {box_unit}. So the {digit} can't go anywhere else in {box_unit}."
                ),
                cells,
            })
        })
    })
}
