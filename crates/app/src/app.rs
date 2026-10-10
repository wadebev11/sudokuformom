use std::time::Duration;

use leptos::prelude::*;
use sudoku_core::hint::Hint;
use sudoku_core::stats::Stats;
use sudoku_core::{Cell, Difficulty, Game, PlayedPuzzles, PuzzleBank};

use crate::board::Board;
use crate::celebration::Celebration;
use crate::format::clock;
use crate::hint_panel::HintPanel;
use crate::new_game_menu::NewGameMenu;
use crate::number_pad::NumberPad;
use crate::scenery::Scenery;
use crate::settings::{Settings, SettingsMenu};
use crate::stats_menu::StatsMenu;
use crate::storage::{self, ELAPSED_KEY, GAME_KEY, PLAYED_KEY, SETTINGS_KEY, STATS_KEY};

static PUZZLE_BANK: PuzzleBank = PuzzleBank::new(include_bytes!("../puzzles.bin"));

fn new_game(difficulty: Difficulty, played: &mut PlayedPuzzles) -> Game {
    let puzzle = PUZZLE_BANK.pick_unplayed(difficulty, played, js_sys::Math::random());
    Game::new(puzzle, difficulty)
}

fn app_is_on_screen() -> bool {
    web_sys::window()
        .and_then(|window| window.document())
        .is_some_and(|document| !document.hidden())
}

#[derive(Clone, Copy, PartialEq)]
enum Dialog {
    None,
    NewGame,
    Settings,
    Stats,
}

#[component]
pub fn App() -> impl IntoView {
    let mut played_puzzles: PlayedPuzzles = storage::load(PLAYED_KEY).unwrap_or_default();
    let saved_game = storage::load(GAME_KEY);
    let game = RwSignal::new(
        saved_game.unwrap_or_else(|| new_game(Difficulty::Easy, &mut played_puzzles)),
    );
    let played = RwSignal::new(played_puzzles);
    // Kept apart from the game so the clock ticking each second doesn't redraw the board.
    let elapsed_seconds = RwSignal::new(storage::load::<u64>(ELAPSED_KEY).unwrap_or_default());
    let solved = Memo::new(move |_| game.with(Game::is_solved));
    let settings = RwSignal::new(storage::load::<Settings>(SETTINGS_KEY).unwrap_or_default());
    let stats = RwSignal::new(storage::load::<Stats>(STATS_KEY).unwrap_or_default());
    let selected = RwSignal::new(None::<Cell>);
    let hint = RwSignal::new(None::<Hint>);
    let dialog = RwSignal::new(Dialog::None);
    let celebration_closed = RwSignal::new(false);
    let celebration_open = Memo::new(move |_| solved.get() && !celebration_closed.get());

    Effect::new(move || game.with(|game| storage::save(GAME_KEY, game)));
    Effect::new(move || storage::save(ELAPSED_KEY, &elapsed_seconds.get()));
    Effect::new(move || played.with(|played| storage::save(PLAYED_KEY, played)));
    Effect::new(move || settings.with(|settings| storage::save(SETTINGS_KEY, settings)));
    Effect::new(move || {
        let theme = settings.with(|settings| settings.theme);
        if let Some(root) = document().document_element() {
            let _ = root.set_attribute("data-theme", theme.attribute_value());
        }
    });
    Effect::new(move || stats.with(|stats| storage::save(STATS_KEY, stats)));

    // Records a solve only when it happens, not when an already solved game is reopened.
    Effect::new(move |was_solved: Option<bool>| {
        let is_solved = solved.get();
        if is_solved && was_solved == Some(false) {
            let difficulty = game.with_untracked(Game::difficulty);
            let seconds = elapsed_seconds.get_untracked();
            stats.update(|stats| stats.record_solve(difficulty, seconds));
        }
        is_solved
    });

    set_interval(
        move || {
            if app_is_on_screen() && !solved.get_untracked() {
                *elapsed_seconds.write() += 1;
            }
        },
        Duration::from_secs(1),
    );

    let start_new_game = Callback::new(move |difficulty| {
        played.update(|played| game.set(new_game(difficulty, played)));
        elapsed_seconds.set(0);
        selected.set(None);
        hint.set(None);
        celebration_closed.set(false);
        dialog.set(Dialog::None);
    });
    let close_dialog = Callback::new(move |_| dialog.set(Dialog::None));

    view! {
        <Scenery />
        <main>
            <header>
                <h1>"Sudoku for Mom"</h1>
                <button class="link" on:click=move |_| dialog.set(Dialog::Stats)>
                    "Stats"
                </button>
                <button class="link" on:click=move |_| dialog.set(Dialog::Settings)>
                    "Settings"
                </button>
            </header>
            <div class="status-bar">
                <span class="difficulty">{move || game.with(Game::difficulty).name()}</span>
                <span class="timer" aria-label="Time">
                    {move || clock(elapsed_seconds.get())}
                </span>
                <button on:click=move |_| dialog.set(Dialog::NewGame)>"New game"</button>
            </div>
            <Board game selected hint settings />
            <HintPanel hint />
            <NumberPad
                game
                selected
                hint
                solved
                accepts_keys=Signal::derive(move || {
                    dialog.get() == Dialog::None && !celebration_open.get()
                })
            />
            <Show when=move || celebration_open.get()>
                <Celebration
                    game
                    elapsed_seconds
                    stats
                    on_new_game=Callback::new(move |_| dialog.set(Dialog::NewGame))
                    on_close=Callback::new(move |_| celebration_closed.set(true))
                />
            </Show>
            <Show when=move || dialog.get() == Dialog::NewGame>
                <NewGameMenu
                    progress_will_be_lost=Signal::derive(move || {
                        game.with(Game::has_progress) && !solved.get()
                    })
                    on_choose=start_new_game
                    on_cancel=close_dialog
                />
            </Show>
            <Show when=move || dialog.get() == Dialog::Settings>
                <SettingsMenu settings on_close=close_dialog />
            </Show>
            <Show when=move || dialog.get() == Dialog::Stats>
                <StatsMenu stats on_close=close_dialog />
            </Show>
        </main>
    }
}
