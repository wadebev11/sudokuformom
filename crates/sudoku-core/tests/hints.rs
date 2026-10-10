mod common;

use common::{EASY_PUZZLE, EXPERT_PUZZLE, grid};
use sudoku_core::hint::Hint;
use sudoku_core::{Difficulty, Game};

#[test]
fn a_mistake_is_pointed_out_before_anything_else_and_applying_clears_it() {
    let mut game = Game::new(grid(EASY_PUZZLE), Difficulty::Easy);
    game.enter(2, 1);
    let hint = game.hint().unwrap();
    assert_eq!(hint, Hint::Mistake(2));
    game.apply_hint(&hint);
    assert_eq!(game.value(2), None);
}

#[test]
fn easy_hints_place_a_number_and_explain_why() {
    let mut game = Game::new(grid(EASY_PUZZLE), Difficulty::Easy);
    let Some(Hint::Step(step)) = game.hint() else {
        panic!("expected a step")
    };
    let (cell, digit) = step.placement.unwrap();
    assert!(step.explanation.contains(&digit.to_string()));
    game.apply_hint(&Hint::Step(step));
    assert_eq!(game.value(cell), Some(digit));
}

#[test]
fn following_hints_alone_solves_even_an_expert_puzzle() {
    let mut game = Game::new(grid(EXPERT_PUZZLE), Difficulty::Expert);
    let mut techniques_seen = Vec::new();
    for _ in 0..1000 {
        if let Some(hint) = game.hint() {
            if let Hint::Step(step) = &hint {
                techniques_seen.push(step.technique);
            }
            game.apply_hint(&hint);
        }
    }
    assert!(game.is_solved());
    assert!(
        techniques_seen
            .iter()
            .any(|technique| technique.difficulty() == Difficulty::Expert)
    );
}

#[test]
fn hints_ignore_wrong_pencil_marks() {
    let mut game = Game::new(grid(EASY_PUZZLE), Difficulty::Easy);
    let expected = game.hint();
    (0..81).for_each(|cell| game.toggle_pencil_mark(cell, 1));
    assert_eq!(game.hint(), expected);
}

#[test]
fn applying_a_crossing_out_hint_fills_in_notes_without_the_ruled_out_numbers() {
    let mut game = Game::new(grid(EXPERT_PUZZLE), Difficulty::Expert);
    let mut crossing_out = game.hint().unwrap();
    while crossing_out.places_a_number() {
        game.apply_hint(&crossing_out);
        crossing_out = game.hint().unwrap();
    }
    let Hint::Step(step) = &crossing_out else {
        panic!("expected a step")
    };

    game.apply_hint(&crossing_out);
    for (cell, digit) in &step.eliminations {
        assert_ne!(game.pencil_marks(*cell), 0);
        assert_eq!(game.pencil_marks(*cell) & (1 << digit), 0);
    }
    assert_ne!(game.hint(), Some(crossing_out));
}
