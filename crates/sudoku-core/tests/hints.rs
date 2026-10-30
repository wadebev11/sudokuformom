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
fn applying_an_elimination_hint_fills_in_pencil_marks_without_the_eliminated_candidates() {
    let mut game = Game::new(grid(EXPERT_PUZZLE), Difficulty::Expert);
    let mut elimination_hint = game.hint().unwrap();
    while elimination_hint.places_a_number() {
        game.apply_hint(&elimination_hint);
        elimination_hint = game.hint().unwrap();
    }
    let Hint::Step(step) = &elimination_hint else {
        panic!("expected a step")
    };

    game.apply_hint(&elimination_hint);
    for (cell, digit) in &step.eliminations {
        assert_ne!(game.pencil_marks(*cell), 0);
        assert_eq!(game.pencil_marks(*cell) & (1 << digit), 0);
    }
    assert_ne!(game.hint(), Some(elimination_hint));
}

#[test]
fn incomplete_pencil_marks_dont_skip_an_elimination_hint() {
    let mut game = Game::new(grid(EXPERT_PUZZLE), Difficulty::Expert);
    let mut elimination_hint = game.clone().hint().unwrap();
    while elimination_hint.places_a_number() {
        game.apply_hint(&elimination_hint);
        elimination_hint = game.clone().hint().unwrap();
    }
    let Hint::Step(step) = &elimination_hint else {
        panic!("expected a step")
    };

    for (cell, _) in &step.eliminations {
        let unrelated_digit = (1..=9)
            .find(|digit| !step.eliminations.contains(&(*cell, *digit)))
            .unwrap();
        if game.pencil_marks(*cell) == 0 {
            game.toggle_pencil_mark(*cell, unrelated_digit);
        }
    }
    assert_eq!(game.hint(), Some(elimination_hint));
}

#[test]
fn a_step_names_its_technique_but_a_mistake_does_not() {
    let mut game = Game::new(grid(EASY_PUZZLE), Difficulty::Easy);
    assert!(game.hint().unwrap().technique().is_some());
    game.enter(2, 1);
    assert_eq!(game.hint().unwrap().technique(), None);
}

#[test]
fn a_naked_single_that_needs_an_earlier_hint_says_so() {
    let mut game = Game::new(
        grid("196275384005183692823496050000007020000500000651924873060749000509600200010052069"),
        Difficulty::Hard,
    );
    let explanations: Vec<String> = (0..3).map(|_| game.hint().unwrap().explanation()).collect();
    assert!(explanations[1].starts_with("The three highlighted cells in row 4"));
    assert_eq!(
        explanations[2],
        "The only number left for row 4, column 1 is 9. Every other number is already in its row, column or box, or was ruled out by an earlier hint."
    );
}
