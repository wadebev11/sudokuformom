use itertools::Itertools;

use super::explain::join;
use super::{LogicBoard, Step, Technique};
use crate::grid::{Cell, Unit, UnitKind, column_of, row_of};

/// X-Wing (size 2) and Swordfish (size 3): in `size` rows a digit only fits
/// in the same `size` columns, so each of those columns gets its digit from
/// one of those rows. The same holds with rows and columns swapped.
pub(super) fn fish(board: &LogicBoard, size: usize) -> Option<Step> {
    [
        (UnitKind::Row, UnitKind::Column),
        (UnitKind::Column, UnitKind::Row),
    ]
    .into_iter()
    .find_map(|(base_kind, cover_kind)| {
        (1..=9).find_map(|digit| find_fish(board, size, digit, base_kind, cover_kind))
    })
}

fn find_fish(
    board: &LogicBoard,
    size: usize,
    digit: u8,
    base_kind: UnitKind,
    cover_kind: UnitKind,
) -> Option<Step> {
    let cover_index = |cell: Cell| {
        if cover_kind == UnitKind::Column {
            column_of(cell)
        } else {
            row_of(cell)
        }
    };
    let bases: Vec<(Unit, Vec<Cell>)> = (0..9)
        .map(|index| Unit {
            kind: base_kind,
            index,
        })
        .map(|unit| (unit, board.cells_for(&unit, digit)))
        .filter(|(_, cells)| (2..=size).contains(&cells.len()))
        .collect();
    bases.iter().combinations(size).find_map(|chosen| {
        let covers: Vec<usize> = chosen
            .iter()
            .flat_map(|(_, cells)| cells.iter().map(|cell| cover_index(*cell)))
            .unique()
            .sorted()
            .collect();
        let pattern: Vec<Cell> = chosen.iter().flat_map(|(_, cells)| cells.clone()).collect();
        let eliminations: Vec<_> = if covers.len() == size {
                covers
                    .iter()
                    .flat_map(|index| board.cells_for(&Unit { kind: cover_kind, index: *index }, digit))
                    .filter(|cell| !pattern.contains(cell))
                    .map(|cell| (cell, digit))
                    .collect()
        } else {
            Vec::new()
        };
        let base_names = join(chosen.iter().map(|(unit, _)| unit.to_string()), "and");
        let cover_names = join(covers.iter().map(|index| Unit { kind: cover_kind, index: *index }), "and");
        (!eliminations.is_empty()).then(|| Step {
            technique: if size == 2 { Technique::XWing } else { Technique::Swordfish },
            placement: None,
            eliminations,
            explanation: format!(
                "In {base_names}, the {digit} can only go in the highlighted cells, which sit in {cover_names}. Each of those rows or columns needs its {digit} from those cells, so the {digit} can't go anywhere else in {cover_names}."
            ),
            cells: pattern,
        })
    })
}
