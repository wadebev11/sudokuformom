use std::sync::LazyLock;

use leptos::prelude::*;
use sudoku_core::hint::Hint;
use sudoku_core::{Cell, Difficulty, Game, PlayedPuzzles, PuzzleBank};

use crate::board::Board;
use crate::hint_panel::HintPanel;
use crate::new_game_menu::NewGameMenu;
use crate::number_pad::NumberPad;
use crate::settings::{Settings, SettingsMenu};
use crate::storage::{self, GAME_KEY, PLAYED_KEY, SETTINGS_KEY};

static PUZZLE_BANK: LazyLock<PuzzleBank> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../puzzles.json"))
        .expect("bundled puzzle bank is valid JSON")
});

fn new_game(difficulty: Difficulty, played: &mut PlayedPuzzles) -> Game {
    let puzzle = PUZZLE_BANK.pick_unplayed(difficulty, played, js_sys::Math::random());
    Game::new(puzzle, difficulty)
}

#[derive(Clone, Copy, PartialEq)]
enum Dialog {
    None,
    NewGame,
    Settings,
}

#[component]
pub fn App() -> impl IntoView {
    let mut played_puzzles: PlayedPuzzles = storage::load(PLAYED_KEY).unwrap_or_default();
    let saved_game = storage::load(GAME_KEY);
    let game = RwSignal::new(
        saved_game.unwrap_or_else(|| new_game(Difficulty::Easy, &mut played_puzzles)),
    );
    let played = RwSignal::new(played_puzzles);
    let settings = RwSignal::new(storage::load::<Settings>(SETTINGS_KEY).unwrap_or_default());
    let selected = RwSignal::new(None::<Cell>);
    let hint = RwSignal::new(None::<Hint>);
    let dialog = RwSignal::new(Dialog::None);

    Effect::new(move || game.with(|game| storage::save(GAME_KEY, game)));
    Effect::new(move || played.with(|played| storage::save(PLAYED_KEY, played)));
    Effect::new(move || settings.with(|settings| storage::save(SETTINGS_KEY, settings)));

    let start_new_game = Callback::new(move |difficulty| {
        let fresh = played
            .try_update(|played| new_game(difficulty, played))
            .unwrap();
        game.set(fresh);
        selected.set(None);
        hint.set(None);
        dialog.set(Dialog::None);
    });
    let close_dialog = Callback::new(move |_| dialog.set(Dialog::None));

    view! {
        <main>
            <header>
                <h1>"Sudoku for Mom"</h1>
                <span class="difficulty">{move || game.with(Game::difficulty).name()}</span>
                <button aria-label="Settings" on:click=move |_| dialog.set(Dialog::Settings)>
                    "⚙"
                </button>
                <button on:click=move |_| dialog.set(Dialog::NewGame)>"New game"</button>
            </header>
            <Board game selected hint settings />
            <Show when=move || game.with(Game::is_solved)>
                <p class="solved" role="status">"Solved!"</p>
            </Show>
            <HintPanel hint />
            <NumberPad game selected hint />
            <Show when=move || dialog.get() == Dialog::NewGame>
                <NewGameMenu
                    progress_will_be_lost=Signal::derive(move || {
                        game.with(|game| game.has_progress() && !game.is_solved())
                    })
                    on_choose=start_new_game
                    on_cancel=close_dialog
                />
            </Show>
            <Show when=move || dialog.get() == Dialog::Settings>
                <SettingsMenu settings on_close=close_dialog />
            </Show>
        </main>
    }
}
