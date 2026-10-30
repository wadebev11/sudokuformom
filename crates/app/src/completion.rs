use sudoku_core::grid::{CELL_COUNT, Unit};
use sudoku_core::{Cell, Game};

use crate::ink_in::InkIn;

pub const CELL_ANIMATION_MS: u32 = 600;
const UNIT_SPREAD_MS: u32 = 400;
const BOARD_SPREAD_MS: u32 = 1200;
const MAX_BEAT_MS: u32 = 150;
pub const BOARD_ANIMATION_MS: u32 = BOARD_SPREAD_MS + CELL_ANIMATION_MS;

#[derive(Clone, Copy)]
enum Pattern {
    OneByOne,
    Random,
    CrissCross,
    EveryOther,
    AllAtOnce,
}

const UNIT_PATTERNS: [Pattern; 5] = [
    Pattern::OneByOne,
    Pattern::Random,
    Pattern::CrissCross,
    Pattern::EveryOther,
    Pattern::AllAtOnce,
];

/// Flashing all 81 cells together is too abrupt for finishing the puzzle.
const BOARD_PATTERNS: [Pattern; 4] = [
    Pattern::OneByOne,
    Pattern::Random,
    Pattern::CrissCross,
    Pattern::EveryOther,
];

impl Pattern {
    fn pick(patterns: &[Pattern]) -> Self {
        patterns[random_below(patterns.len())]
    }

    /// Which beat each cell in a line of cells starts animating on.
    fn beats(self, length: usize) -> Vec<usize> {
        match self {
            Pattern::OneByOne => (0..length).collect(),
            Pattern::Random => shuffled((0..length).collect()),
            Pattern::CrissCross => (0..length)
                .map(|position| {
                    let from_end = length - 1 - position;
                    if position <= from_end {
                        position * 2
                    } else {
                        from_end * 2 + 1
                    }
                })
                .collect(),
            Pattern::EveryOther => (0..length).map(|position| position % 2).collect(),
            Pattern::AllAtOnce => vec![0; length],
        }
    }
}

fn random_below(count: usize) -> usize {
    (js_sys::Math::random() * count as f64) as usize
}

fn shuffled(mut items: Vec<usize>) -> Vec<usize> {
    for index in (1..items.len()).rev() {
        items.swap(index, random_below(index + 1));
    }
    items
}

/// The cells a move just finished, and how long each waits before animating.
#[derive(Clone, PartialEq)]
pub struct Completion {
    /// Counts up with each Completion so a cell finished twice in a row animates again.
    pub round: u32,
    delays_ms: [Option<u32>; CELL_COUNT],
}

impl Completion {
    pub fn delay_ms(&self, cell: Cell) -> Option<u32> {
        self.delays_ms[cell]
    }

    /// The whole board when the move solved the puzzle, otherwise each newly solved Unit.
    /// Each waits for the cells Auto-solve filled in it to finish inking in.
    pub fn after_move(
        previous: Option<&Completion>,
        solved_units_before: &[Unit],
        game: &Game,
        ink_in: &InkIn,
    ) -> Option<Completion> {
        let mut delays_ms = [None; CELL_COUNT];
        if game.is_solved() {
            let all_cells: Vec<Cell> = (0..CELL_COUNT).collect();
            let pattern = Pattern::pick(&BOARD_PATTERNS);
            let start_ms = ink_in.all_finished_ms();
            add_delays(
                &mut delays_ms,
                &all_cells,
                pattern,
                start_ms,
                BOARD_SPREAD_MS,
            );
        } else {
            let pattern = Pattern::pick(&UNIT_PATTERNS);
            game.solved_units()
                .into_iter()
                .filter(|unit| !solved_units_before.contains(unit))
                .for_each(|unit| {
                    let cells = unit.cells();
                    let start_ms = ink_in.finished_ms(&cells);
                    add_delays(&mut delays_ms, &cells, pattern, start_ms, UNIT_SPREAD_MS)
                });
        }
        delays_ms.iter().any(Option::is_some).then(|| Completion {
            round: previous.map_or(0, |previous| previous.round + 1),
            delays_ms,
        })
    }
}

/// A cell in two finished Units keeps whichever delay is shorter.
fn add_delays(
    delays_ms: &mut [Option<u32>; CELL_COUNT],
    cells: &[Cell],
    pattern: Pattern,
    start_ms: u32,
    spread_ms: u32,
) {
    let beats = pattern.beats(cells.len());
    let last_beat = beats.iter().copied().max().unwrap_or_default() as u32;
    let beat_ms = spread_ms
        .checked_div(last_beat)
        .unwrap_or_default()
        .min(MAX_BEAT_MS);
    for (cell, beat) in cells.iter().zip(beats) {
        let delay_ms = start_ms + beat as u32 * beat_ms;
        let current = &mut delays_ms[*cell];
        *current = Some(current.map_or(delay_ms, |current| current.min(delay_ms)));
    }
}
