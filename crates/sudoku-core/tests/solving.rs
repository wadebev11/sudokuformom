mod common;

use common::{
    BEYOND_EXPERT_PUZZLE, EASY_PUZZLE, EASY_SOLUTION, EXPERT_PUZZLE, HARD_PUZZLE, MEDIUM_PUZZLE,
    grid,
};
use sudoku_core::solver::{has_unique_solution, solve};
use sudoku_core::techniques::LogicBoard;
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
fn puzzles_are_graded_by_the_hardest_technique_they_need() {
    assert_eq!(grade(&grid(EASY_PUZZLE)), Some(Difficulty::Easy));
    assert_eq!(grade(&grid(MEDIUM_PUZZLE)), Some(Difficulty::Medium));
    assert_eq!(grade(&grid(HARD_PUZZLE)), Some(Difficulty::Hard));
    assert_eq!(grade(&grid(EXPERT_PUZZLE)), Some(Difficulty::Expert));
}

#[test]
fn every_logical_step_agrees_with_the_solution() {
    for puzzle in [EASY_PUZZLE, MEDIUM_PUZZLE, HARD_PUZZLE, EXPERT_PUZZLE].map(grid) {
        let solution = solve(&puzzle).unwrap();
        let mut board = LogicBoard::new(&puzzle);
        while let Some(step) = board.next_step() {
            if let Some((cell, digit)) = step.placement {
                assert_eq!(solution.get(cell), digit, "{step:?}");
            }
            for (cell, digit) in &step.eliminations {
                assert_ne!(solution.get(*cell), *digit, "{step:?}");
            }
            assert!(!step.explanation.is_empty());
            board.apply(&step);
        }
        assert_eq!(*board.grid(), solution);
    }
}

#[test]
fn puzzle_needing_guesswork_has_no_difficulty() {
    assert_eq!(grade(&grid(BEYOND_EXPERT_PUZZLE)), None);
}
