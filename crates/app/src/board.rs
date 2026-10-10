use leptos::prelude::*;
use sudoku_core::grid::{CELL_COUNT, column_of, digit_bit, row_of, sees};
use sudoku_core::{Cell, Game};

#[component]
pub fn Board(game: RwSignal<Game>, selected: RwSignal<Option<Cell>>) -> impl IntoView {
    view! {
        <div class="board" role="group" aria-label="Sudoku board">
            {(0..CELL_COUNT).map(|cell| view! { <BoardCell cell game selected /> }).collect_view()}
        </div>
    }
}

#[component]
fn BoardCell(cell: Cell, game: RwSignal<Game>, selected: RwSignal<Option<Cell>>) -> impl IntoView {
    let value = move || game.with(|game| game.value(cell));
    let class = move || {
        let selected_cell = selected.get();
        let selected_value = selected_cell.and_then(|other| game.with(|game| game.value(other)));
        let mut classes = vec!["cell"];
        if column_of(cell) % 3 == 2 && column_of(cell) != 8 {
            classes.push("box-edge-right");
        }
        if row_of(cell) % 3 == 2 && row_of(cell) != 8 {
            classes.push("box-edge-bottom");
        }
        if game.with(|game| game.is_given(cell)) {
            classes.push("given");
        }
        if game.with(|game| game.is_mistake(cell)) {
            classes.push("mistake");
        }
        if selected_cell == Some(cell) {
            classes.push("selected");
        } else if selected_cell.is_some_and(|other| sees(cell, other)) {
            classes.push("related");
        }
        if selected_value.is_some() && value() == selected_value {
            classes.push("same-number");
        }
        classes.join(" ")
    };
    let label = move || {
        let pencil_marks: Vec<String> = (1..=9)
            .filter(|digit| game.with(|game| game.pencil_marks(cell)) & digit_bit(*digit) != 0)
            .map(|digit| digit.to_string())
            .collect();
        let contents = match value() {
            Some(digit) => digit.to_string(),
            None if pencil_marks.is_empty() => "empty".to_string(),
            None => format!("empty, notes {}", pencil_marks.join(" ")),
        };
        format!(
            "Row {}, column {}, {contents}",
            row_of(cell) + 1,
            column_of(cell) + 1
        )
    };

    view! {
        <button class=class aria-label=label on:click=move |_| selected.set(Some(cell))>
            {move || match value() {
                Some(digit) => digit.to_string().into_any(),
                None => view! { <PencilMarks cell game /> }.into_any(),
            }}
        </button>
    }
}

#[component]
fn PencilMarks(cell: Cell, game: RwSignal<Game>) -> impl IntoView {
    let marks = move || game.with(|game| game.pencil_marks(cell));
    view! {
        <span class="pencil-marks" aria-hidden="true">
            {(1..=9)
                .map(|digit| {
                    view! {
                        <span>{move || (marks() & digit_bit(digit) != 0).then_some(digit)}</span>
                    }
                })
                .collect_view()}
        </span>
    }
}
