mod common;

use common::{EASY_PUZZLE, EASY_SOLUTION, grid};
use sudoku_core::{Difficulty, Game, PuzzleBank};

fn new_game() -> Game {
    Game::new(grid(EASY_PUZZLE), Difficulty::Easy)
}

fn fill_in_solution(game: &mut Game) {
    let solution = grid(EASY_SOLUTION);
    (0..81).for_each(|cell| game.enter(cell, solution.get(cell)));
}

#[test]
fn entering_every_correct_number_solves_the_puzzle() {
    let mut game = new_game();
    assert!(!game.is_solved());
    fill_in_solution(&mut game);
    assert!(game.is_solved());
}

#[test]
fn givens_cannot_be_changed() {
    let mut game = new_game();
    game.enter(0, 9);
    game.erase(0);
    assert_eq!(game.value(0), Some(5));
}

#[test]
fn a_wrong_number_is_a_mistake() {
    let mut game = new_game();
    game.enter(2, 1);
    assert!(game.is_mistake(2));
    game.enter(2, 4);
    assert!(!game.is_mistake(2));
}

#[test]
fn a_solved_puzzle_is_locked() {
    let mut game = new_game();
    fill_in_solution(&mut game);
    game.erase(2);
    assert!(game.is_solved());
}

#[test]
fn games_survive_saving_and_loading() {
    let mut game = new_game();
    game.enter(2, 4);
    game.toggle_pencil_mark(3, 6);
    let saved = serde_json::to_string(&game).unwrap();
    let loaded: Game = serde_json::from_str(&saved).unwrap();
    assert_eq!(loaded.board(), game.board());
    assert_eq!(loaded.pencil_marks(3), game.pencil_marks(3));
}

#[test]
fn puzzle_bank_survives_saving_and_loading() {
    let mut bank = PuzzleBank::default();
    bank.add(Difficulty::Easy, grid(EASY_PUZZLE));
    let saved = serde_json::to_string(&bank).unwrap();
    assert_eq!(serde_json::from_str::<PuzzleBank>(&saved).unwrap(), bank);
}

fn marks(game: &Game, cell: usize) -> Vec<u8> {
    (1..=9)
        .filter(|digit| game.pencil_marks(cell) & (1 << digit) != 0)
        .collect()
}

#[test]
fn pencil_marks_toggle_on_and_off() {
    let mut game = new_game();
    game.toggle_pencil_mark(2, 1);
    game.toggle_pencil_mark(2, 4);
    game.toggle_pencil_mark(2, 1);
    assert_eq!(marks(&game, 2), [4]);
}

#[test]
fn entering_a_number_clears_it_from_pencil_marks_it_rules_out() {
    let mut game = new_game();
    game.toggle_pencil_mark(2, 4);
    game.toggle_pencil_mark(2, 1);
    game.toggle_pencil_mark(3, 4);
    game.toggle_pencil_mark(78, 4);
    game.enter(2, 4);
    assert_eq!(marks(&game, 2), Vec::<u8>::new());
    assert_eq!(marks(&game, 3), Vec::<u8>::new());
    assert_eq!(marks(&game, 78), [4]);
}

#[test]
fn undo_and_redo_step_through_changes() {
    let mut game = new_game();
    game.enter(2, 4);
    game.toggle_pencil_mark(3, 6);
    game.undo();
    game.undo();
    assert_eq!(game.value(2), None);
    game.redo();
    assert_eq!(game.value(2), Some(4));
    game.enter(3, 6);
    assert!(!game.can_redo());
}

#[test]
fn new_games_do_not_repeat_a_puzzle_until_all_have_been_played() {
    let mut bank = PuzzleBank::default();
    let puzzle = grid(EASY_PUZZLE);
    let mut other = puzzle;
    other.set(2, 4);
    bank.add(Difficulty::Easy, puzzle);
    bank.add(Difficulty::Easy, other);
    let mut played = sudoku_core::PlayedPuzzles::default();

    let first = bank.pick_unplayed(Difficulty::Easy, &mut played, 0.0);
    let second = bank.pick_unplayed(Difficulty::Easy, &mut played, 0.0);
    assert_ne!(first, second);
    let third = bank.pick_unplayed(Difficulty::Easy, &mut played, 0.99);
    assert!([puzzle, other].contains(&third));
}

#[test]
fn a_fresh_game_has_no_progress_to_lose() {
    let mut game = new_game();
    assert!(!game.has_progress());
    game.toggle_pencil_mark(2, 4);
    assert!(game.has_progress());
}
