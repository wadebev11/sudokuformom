mod common;

use common::{BEYOND_EXPERT_PUZZLE, EASY_PUZZLE, EASY_SOLUTION, grid};
use sudoku_core::solver::{has_unique_solution, solve};
use sudoku_core::{Difficulty, grade};

#[test]
fn solves_a_puzzle() {
    assert_eq!(solve(&grid(EASY_PUZZLE)), Some(grid(EASY_SOLUTION)));
}

#[test]
fn hard_puzzles_have_one_solution() {
    assert!(has_unique_solution(&grid(BEYOND_EXPERT_PUZZLE)));
}

#[test]
fn puzzle_with_too_few_givens_has_many_solutions() {
    let mut puzzle = grid(EASY_PUZZLE);
    (0..27).for_each(|cell| puzzle.set(cell, 0));
    assert!(!has_unique_solution(&puzzle));
}

#[test]
fn puzzle_with_a_repeated_number_has_no_solution() {
    let mut puzzle = grid(EASY_PUZZLE);
    puzzle.set(2, 5);
    assert_eq!(solve(&puzzle), None);
}

#[test]
fn puzzle_solvable_with_singles_is_easy() {
    assert_eq!(grade(&grid(EASY_PUZZLE)), Some(Difficulty::Easy));
}

#[test]
fn puzzle_needing_guesswork_has_no_difficulty() {
    assert_eq!(grade(&grid(BEYOND_EXPERT_PUZZLE)), None);
}
