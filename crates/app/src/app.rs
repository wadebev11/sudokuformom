use std::sync::LazyLock;

use leptos::prelude::*;
use sudoku_core::{Cell, Difficulty, Game, PuzzleBank};

use crate::board::Board;
use crate::new_game_menu::NewGameMenu;
use crate::number_pad::NumberPad;

static PUZZLE_BANK: LazyLock<PuzzleBank> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../puzzles.json"))
        .expect("bundled puzzle bank is valid JSON")
});

fn new_game(difficulty: Difficulty) -> Game {
    let puzzles = PUZZLE_BANK.puzzles(difficulty);
    let index = (js_sys::Math::random() * puzzles.len() as f64) as usize;
    Game::new(puzzles[index], difficulty)
}

#[component]
pub fn App() -> impl IntoView {
    let game = RwSignal::new(new_game(Difficulty::Easy));
    let selected = RwSignal::new(None::<Cell>);
    let choosing_difficulty = RwSignal::new(false);
    let start_new_game = Callback::new(move |difficulty| {
        game.set(new_game(difficulty));
        selected.set(None);
        choosing_difficulty.set(false);
    });

    view! {
        <main>
            <header>
                <h1>"Sudoku for Mom"</h1>
                <span class="difficulty">{move || game.with(Game::difficulty).name()}</span>
                <button on:click=move |_| choosing_difficulty.set(true)>"New game"</button>
            </header>
            <Board game selected />
            <Show when=move || game.with(Game::is_solved)>
                <p class="solved" role="status">"Solved!"</p>
            </Show>
            <NumberPad game selected />
            <Show when=move || choosing_difficulty.get()>
                <NewGameMenu
                    on_choose=start_new_game
                    on_cancel=Callback::new(move |_| choosing_difficulty.set(false))
                />
            </Show>
        </main>
    }
}
