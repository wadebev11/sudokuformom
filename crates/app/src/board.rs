use leptos::prelude::*;
use sudoku_core::grid::{CELL_COUNT, DigitSet, column_of, digit_bit, row_of, sees};
use sudoku_core::hint::Hint;
use sudoku_core::{Cell, Digit, Game};

use crate::settings::Settings;

/// Everything a cell shows from the game. Each cell watches its own copy, so
/// a move only redraws the cells it actually changed.
#[derive(Clone, Copy, PartialEq)]
struct CellContents {
    value: Option<Digit>,
    pencil_marks: DigitSet,
    is_given: bool,
    is_mistake: bool,
}

/// The cells the shown Hint points at, worked out once per Hint rather than once per cell.
#[derive(Clone, Default, PartialEq)]
struct HintedCells {
    cells: Vec<Cell>,
    targets: Vec<Cell>,
}

#[component]
pub fn Board(
    game: RwSignal<Game>,
    selected: RwSignal<Option<Cell>>,
    hint: RwSignal<Option<Hint>>,
    settings: RwSignal<Settings>,
) -> impl IntoView {
    let selected_value = Memo::new(move |_| {
        selected
            .get()
            .and_then(|cell| game.with(|game| game.value(cell)))
    });
    let hinted = Memo::new(move |_| {
        hint.with(|hint| {
            hint.as_ref()
                .map(|hint| HintedCells {
                    cells: hint.cells(),
                    targets: hint.target_cells(),
                })
                .unwrap_or_default()
        })
    });
    let show_mistakes = Memo::new(move |_| settings.with(|settings| settings.show_mistakes));

    view! {
        <div class="board" role="group" aria-label="Sudoku board">
            {(0..CELL_COUNT)
                .map(|cell| {
                    view! { <BoardCell cell game selected selected_value hinted show_mistakes /> }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn BoardCell(
    cell: Cell,
    game: RwSignal<Game>,
    selected: RwSignal<Option<Cell>>,
    selected_value: Memo<Option<Digit>>,
    hinted: Memo<HintedCells>,
    show_mistakes: Memo<bool>,
) -> impl IntoView {
    let contents = Memo::new(move |_| {
        game.with(|game| CellContents {
            value: game.value(cell),
            pencil_marks: game.pencil_marks(cell),
            is_given: game.is_given(cell),
            is_mistake: game.is_mistake(cell),
        })
    });
    // A memo, so the page is only touched for cells whose classes really change.
    let class = Memo::new(move |_| {
        let contents = contents.get();
        let selected_cell = selected.get();
        let mut class = String::from("cell");
        class.push_str(&edge_classes(row_of(cell), column_of(cell), 8, 8));
        if contents.is_given {
            class.push_str(" given");
        }
        if show_mistakes.get() && contents.is_mistake {
            class.push_str(" mistake");
        }
        if hinted.with(|hinted| hinted.cells.contains(&cell)) {
            class.push_str(" hinted");
        }
        if hinted.with(|hinted| hinted.targets.contains(&cell)) {
            class.push_str(" hint-target");
        }
        if selected_cell == Some(cell) {
            class.push_str(" selected");
        } else if selected_cell.is_some_and(|other| sees(cell, other)) {
            class.push_str(" related");
        }
        let selected_value = selected_value.get();
        if selected_value.is_some() && contents.value == selected_value {
            class.push_str(" same-number");
        }
        class
    });
    let label = move || {
        let CellContents {
            value,
            pencil_marks,
            ..
        } = contents.get();
        let notes: Vec<String> = (1..=9)
            .filter(|digit| pencil_marks & digit_bit(*digit) != 0)
            .map(|digit| digit.to_string())
            .collect();
        let description = match value {
            Some(digit) => digit.to_string(),
            None if notes.is_empty() => "empty".to_string(),
            None => format!("empty, notes {}", notes.join(" ")),
        };
        format!(
            "Row {}, column {}, {description}",
            row_of(cell) + 1,
            column_of(cell) + 1
        )
    };

    view! {
        <button class=class aria-label=label on:click=move |_| selected.set(Some(cell))>
            {move || {
                let CellContents {
                    value,
                    pencil_marks,
                    ..
                } = contents.get();
                match value {
                    Some(digit) => digit.to_string().into_any(),
                    None => pencil_marks_view(pencil_marks, 0).into_any(),
                }
            }}
        </button>
    }
}

/// Classes for the lines on a cell's right and bottom: thick between Boxes,
/// none on the outside where the board's own border is.
pub fn edge_classes(row: usize, column: usize, last_row: usize, last_column: usize) -> String {
    let mut classes = String::new();
    if column == last_column {
        classes.push_str(" last-column");
    } else if column % 3 == 2 {
        classes.push_str(" box-edge-right");
    }
    if row == last_row {
        classes.push_str(" last-row");
    } else if row % 3 == 2 {
        classes.push_str(" box-edge-bottom");
    }
    classes
}

/// Drawn whole when the marks change rather than as 9 reactive spans per cell,
/// and left out entirely when a cell has no marks. Crossed out marks are
/// only used to show Eliminations in technique examples.
pub fn pencil_marks_view(pencil_marks: DigitSet, crossed_out: DigitSet) -> impl IntoView {
    (pencil_marks | crossed_out != 0).then(|| {
        view! {
            <span class="pencil-marks" aria-hidden="true">
                {(1..=9)
                    .map(|digit| {
                        let is_crossed_out = crossed_out & digit_bit(digit) != 0;
                        let is_shown = is_crossed_out || pencil_marks & digit_bit(digit) != 0;
                        view! { <span class:crossed-out=is_crossed_out>{is_shown.then_some(digit)}</span> }
                    })
                    .collect_view()}
            </span>
        }
    })
}
