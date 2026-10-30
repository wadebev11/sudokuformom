mod app;
mod board;
mod celebration;
mod completion;
mod format;
mod hint_panel;
mod modal;
mod new_game_menu;
mod number_pad;
mod problem_report;
mod scenery;
mod settings;
mod stats_menu;
mod storage;
mod technique_example;
mod technique_guide;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
}
