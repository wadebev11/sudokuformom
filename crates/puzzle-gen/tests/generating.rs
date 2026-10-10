use rand::SeedableRng;
use rand::rngs::StdRng;
use sudoku_core::solver::has_unique_solution;
use sudoku_core::{Difficulty, grade};

#[test]
fn generated_puzzles_have_one_solution_and_the_requested_difficulty() {
    let mut rng = StdRng::seed_from_u64(7);
    let puzzles: Vec<_> =
        std::iter::from_fn(|| Some(puzzle_gen::generate(Difficulty::Easy, &mut rng)))
            .flatten()
            .take(3)
            .collect();
    for puzzle in puzzles {
        assert!(has_unique_solution(&puzzle));
        assert_eq!(grade(&puzzle), Some(Difficulty::Easy));
    }
}
