#![allow(dead_code)]

use sudoku_core::Grid;

pub const EASY_PUZZLE: &str = "
    530070000 600195000 098000060
    800060003 400803001 700020006
    060000280 000419005 000080079";

pub const EASY_SOLUTION: &str = "
    534678912 672195348 198342567
    859761423 426853791 713924856
    961537284 287419635 345286179";

/// Needs guessing; no human Technique cracks it.
pub const BEYOND_EXPERT_PUZZLE: &str = "
    800000000 003600000 070090200
    050007000 000045700 000100030
    001000068 008500010 090000400";

pub fn grid(text: &str) -> Grid {
    text.parse().expect("test grid is valid")
}
