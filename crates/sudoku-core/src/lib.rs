//! Sudoku rules, solving, grading and game state. Pure Rust with no I/O so it
//! can run both natively (puzzle generation, tests) and in the browser.

pub mod bank;
pub mod game;
pub mod grid;
pub mod hint;
pub mod solver;
pub mod techniques;

pub use bank::PuzzleBank;
pub use game::Game;
pub use grid::{Cell, Digit, Grid};
pub use techniques::{Difficulty, Step, Technique, grade};
