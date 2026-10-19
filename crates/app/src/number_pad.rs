use leptos::ev;
use leptos::prelude::*;
use sudoku_core::hint::Hint;
use sudoku_core::{Cell, Digit, Game};

use crate::completion::Completion;

#[component]
pub fn NumberPad(
    game: RwSignal<Game>,
    selected: RwSignal<Option<Cell>>,
    hint: RwSignal<Option<Hint>>,
    completion: RwSignal<Option<Completion>>,
    solved: Memo<bool>,
    auto_solves: Memo<bool>,
    accepts_keys: Signal<bool>,
) -> impl IntoView {
    let pencil_mode = RwSignal::new(false);
    let change_game = move |change: &dyn Fn(&mut Game)| {
        let solved_units_before = game.with_untracked(Game::solved_units);
        game.update(|game| {
            change(game);
            if auto_solves.get_untracked() {
                game.auto_solve_naked_singles();
            }
        });
        hint.set(None);
        let next = game.with_untracked(|game| {
            Completion::after_move(
                completion.get_untracked().as_ref(),
                &solved_units_before,
                game,
            )
        });
        if next.is_some() {
            completion.set(next);
        }
    };
    let is_fully_placed = move |digit| game.with(|game| game.is_fully_placed(digit));
    let enter = move |digit| {
        if let Some(cell) = selected.get()
            && !is_fully_placed(digit)
        {
            if pencil_mode.get() {
                change_game(&|game| game.toggle_pencil_mark(cell, digit));
            } else {
                change_game(&|game| game.enter(cell, digit));
            }
        }
    };
    let erase = move || {
        if let Some(cell) = selected.get() {
            change_game(&|game| game.erase(cell));
        }
    };
    let key_listener = window_event_listener(ev::keydown, move |event| {
        let has_modifier = event.ctrl_key() || event.meta_key() || event.alt_key();
        if accepts_keys.get_untracked() && !has_modifier {
            let key = event.key();
            if let Some(digit) = key
                .parse::<Digit>()
                .ok()
                .filter(|digit| (1..=9).contains(digit))
            {
                event.prevent_default();
                enter(digit);
            } else if key == "Backspace" || key == "Delete" {
                event.prevent_default();
                erase();
            }
        }
    });
    on_cleanup(move || key_listener.remove());
    let fills_notes = move || pencil_mode.get() && hint.with(Option::is_none);
    let hint_button_action = move |_| match hint.get() {
        Some(shown) => change_game(&|game| game.apply_hint(&shown)),
        None if pencil_mode.get() => change_game(&Game::fill_in_pencil_marks),
        None => hint.set(game.with(Game::hint)),
    };

    view! {
        <div class="toolbar">
            <button on:click=move |_| change_game(&Game::undo) disabled=move || !game.with(Game::can_undo)>
                "Undo"
            </button>
            <button on:click=move |_| change_game(&Game::redo) disabled=move || !game.with(Game::can_redo)>
                "Redo"
            </button>
            <button
                class:active=move || pencil_mode.get()
                aria-pressed=move || pencil_mode.get().to_string()
                on:click=move |_| pencil_mode.update(|on| *on = !*on)
            >
                "Pencil"
            </button>
            <button on:click=move |_| erase()>"Erase"</button>
            <button
                class:active=move || hint.with(Option::is_some)
                aria-label=move || if fills_notes() { "Fill notes" } else { "Hint" }
                on:click=hint_button_action
                disabled=move || {
                    solved.get() || (fills_notes() && !game.with(Game::can_fill_in_pencil_marks))
                }
            >
                {move || if fills_notes() { "Fill" } else { "Hint" }}
            </button>
        </div>
        <div class="number-pad" class:pencil=move || pencil_mode.get()>
            {(1..=9).map(|digit| {
                view! {
                    <button on:click=move |_| enter(digit) disabled=move || is_fully_placed(digit)>
                        {digit}
                    </button>
                }
            }).collect_view()}
        </div>
    }
}
