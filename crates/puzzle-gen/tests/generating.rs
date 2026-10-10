use rand::SeedableRng;
use rand::rngs::StdRng;
use sudoku_core::solver::{has_unique_solution, solve};
use sudoku_core::techniques::LogicBoard;
use sudoku_core::{Difficulty, grade};

#[test]
fn generated_puzzles_have_one_solution_the_requested_difficulty_and_enough_givens() {
    for difficulty in [Difficulty::Easy, Difficulty::Medium, Difficulty::Expert] {
        let mut rng = StdRng::seed_from_u64(7);
        let puzzle = std::iter::repeat_with(|| puzzle_gen::generate(difficulty, &mut rng))
            .flatten()
            .next()
            .unwrap();
        assert!(has_unique_solution(&puzzle));
        assert_eq!(grade(&puzzle), Some(difficulty));
        assert!(puzzle.filled_count() >= puzzle_gen::minimum_givens(difficulty));
    }
}

/// Hints come from these steps, so a wrong deduction would mislead the player.
#[test]
fn logical_steps_never_contradict_the_solution_across_many_puzzles() {
    let mut rng = StdRng::seed_from_u64(11);
    let puzzles: Vec<_> =
        std::iter::repeat_with(|| puzzle_gen::generate(Difficulty::Expert, &mut rng))
            .flatten()
            .take(5)
            .collect();
    for puzzle in puzzles {
        let solution = solve(&puzzle).unwrap();
        let mut board = LogicBoard::new(&puzzle);
        while let Some(step) = board.next_step() {
            let placement_is_right = step
                .placement
                .is_none_or(|(cell, digit)| solution.get(cell) == digit);
            let eliminations_are_right = step
                .eliminations
                .iter()
                .all(|(cell, digit)| solution.get(*cell) != *digit);
            assert!(placement_is_right && eliminations_are_right, "{step:?}");
            board.apply(&step);
        }
    }
}
