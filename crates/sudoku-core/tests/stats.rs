use sudoku_core::Difficulty;
use sudoku_core::stats::Stats;

#[test]
fn solving_puzzles_tracks_count_best_and_average_per_difficulty() {
    let mut stats = Stats::default();
    stats.record_solve(Difficulty::Hard, 600);
    stats.record_solve(Difficulty::Hard, 300);
    stats.record_solve(Difficulty::Easy, 100);

    let hard = stats.for_difficulty(Difficulty::Hard);
    assert_eq!(hard.solved, 2);
    assert_eq!(hard.best_seconds, Some(300));
    assert_eq!(hard.average_seconds(), Some(450));
    assert_eq!(stats.for_difficulty(Difficulty::Easy).solved, 1);
}

#[test]
fn unplayed_difficulties_have_no_times() {
    let stats = Stats::default();
    let expert = stats.for_difficulty(Difficulty::Expert);
    assert_eq!(expert.solved, 0);
    assert_eq!(expert.best_seconds, None);
    assert_eq!(expert.average_seconds(), None);
}
