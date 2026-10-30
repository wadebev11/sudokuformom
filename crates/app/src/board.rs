use leptos::prelude::*;
use sudoku_core::grid::{CELL_COUNT, DigitSet, column_of, digit_bit, row_of, sees};
use sudoku_core::hint::Hint;
use sudoku_core::{Cell, Digit, Game};

use crate::completion::{CELL_ANIMATION_MS, Completion};
use crate::ink_in::{INK_IN_MS, InkIn};
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

#[derive(Clone, Copy, PartialEq)]
struct CellHighlight {
    is_given: bool,
    is_mistake: bool,
    is_hinted: bool,
    is_hint_target: bool,
    is_selected: bool,
    is_related: bool,
    is_same_number: bool,
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
    completion: RwSignal<Option<Completion>>,
    ink_in: RwSignal<InkIn>,
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
                    view! { <BoardCell cell game selected selected_value hinted completion ink_in show_mistakes /> }
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
    completion: RwSignal<Option<Completion>>,
    ink_in: RwSignal<InkIn>,
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
    // A memo of flags rather than of the class text, so selecting a cell doesn't
    // build class text for the cells whose highlighting stays the same.
    let highlight = Memo::new(move |_| {
        let contents = contents.get();
        let selected_cell = selected.get();
        let selected_value = selected_value.get();
        CellHighlight {
            is_given: contents.is_given,
            is_mistake: show_mistakes.get() && contents.is_mistake,
            is_hinted: hinted.with(|hinted| hinted.cells.contains(&cell)),
            is_hint_target: hinted.with(|hinted| hinted.targets.contains(&cell)),
            is_selected: selected_cell == Some(cell),
            is_related: selected_cell.is_some_and(|other| other != cell && sees(cell, other)),
            is_same_number: selected_value.is_some() && contents.value == selected_value,
        }
    });
    let matching_pencil_marks = Memo::new(move |_| {
        selected_value.get().map(digit_bit).unwrap_or_default() & contents.get().pencil_marks
    });
    // Alternating the animation's name between rounds restarts it on a cell finished twice in a row.
    let completing = Memo::new(move |_| {
        completion.with(|completion| {
            completion.as_ref().and_then(|completion| {
                completion
                    .delay_ms(cell)
                    .map(|delay_ms| (completion.round % 2 == 0, delay_ms))
            })
        })
    });
    let inking = Memo::new(move |_| {
        ink_in.with(|ink_in| {
            ink_in
                .delay_ms(cell)
                .map(|delay_ms| (ink_in.round % 2 == 0, delay_ms))
        })
    });
    let digit_class = move || match inking.get() {
        Some((true, _)) => "digit ink-in-even",
        Some((false, _)) => "digit ink-in-odd",
        None => "digit",
    };
    let digit_style = move || {
        inking
            .get()
            .map(|(_, delay_ms)| {
                format!("animation-delay: {delay_ms}ms; animation-duration: {INK_IN_MS}ms")
            })
            .unwrap_or_default()
    };
    let edges = edge_classes(row_of(cell), column_of(cell), 8, 8);
    let class = move || {
        let highlight = highlight.get();
        let mut class = format!("cell{edges}");
        for (applies, name) in [
            (highlight.is_given, " given"),
            (highlight.is_mistake, " mistake"),
            (highlight.is_hinted, " hinted"),
            (highlight.is_hint_target, " hint-target"),
            (highlight.is_selected, " selected"),
            (highlight.is_related, " related"),
            (highlight.is_same_number, " same-number"),
        ] {
            if applies {
                class.push_str(name);
            }
        }
        if let Some((is_even_round, _)) = completing.get() {
            class.push_str(if is_even_round {
                " completing-even"
            } else {
                " completing-odd"
            });
        }
        class
    };
    let style = move || {
        completing
            .get()
            .map(|(_, delay_ms)| {
                format!("animation-delay: {delay_ms}ms; animation-duration: {CELL_ANIMATION_MS}ms")
            })
            .unwrap_or_default()
    };
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
        <button class=class style=style aria-label=label on:click=move |_| selected.set(Some(cell))>
            {move || {
                let CellContents {
                    value,
                    pencil_marks,
                    ..
                } = contents.get();
                match value {
                    Some(digit) => {
                        view! { <span class=digit_class style=digit_style>{digit}</span> }.into_any()
                    }
                    None => {
                        pencil_marks_view(pencil_marks, 0, matching_pencil_marks.get()).into_any()
                    }
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
pub fn pencil_marks_view(
    pencil_marks: DigitSet,
    crossed_out: DigitSet,
    matching: DigitSet,
) -> impl IntoView {
    (pencil_marks | crossed_out != 0).then(|| {
        view! {
            <span class="pencil-marks" aria-hidden="true">
                {(1..=9)
                    .map(|digit| {
                        let is_crossed_out = crossed_out & digit_bit(digit) != 0;
                        let is_matching = matching & digit_bit(digit) != 0;
                        let is_shown = is_crossed_out || pencil_marks & digit_bit(digit) != 0;
                        view! {
                            <span class:crossed-out=is_crossed_out class:matching=is_matching>
                                {is_shown.then_some(digit)}
                            </span>
                        }
                    })
                    .collect_view()}
            </span>
        }
    })
}
