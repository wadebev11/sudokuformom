mod app;
mod board;
mod hint_panel;
mod new_game_menu;
mod number_pad;
mod settings;
mod storage;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
}
