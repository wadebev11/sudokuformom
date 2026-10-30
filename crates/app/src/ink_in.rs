use sudoku_core::Cell;
use sudoku_core::grid::CELL_COUNT;

pub const INK_IN_MS: u32 = 300;
const WAVE_GAP_MS: u32 = 150;
const LAST_WAVE_BY_MS: u32 = 900;
/// Lets a new puzzle, or the board behind a closing dialog, show before it fills in.
pub const SETTLE_MS: u32 = 300;

/// The cells Auto-solve just filled, and how long each waits before inking in.
#[derive(Clone, PartialEq)]
pub struct InkIn {
    /// Counts up with each InkIn so a cell filled twice in a row animates again.
    pub round: u32,
    delays_ms: [Option<u32>; CELL_COUNT],
}

impl Default for InkIn {
    fn default() -> Self {
        InkIn {
            round: 0,
            delays_ms: [None; CELL_COUNT],
        }
    }
}

impl InkIn {
    pub fn delay_ms(&self, cell: Cell) -> Option<u32> {
        self.delays_ms[cell]
    }

    /// Each wave starts a beat after the one before, squeezed together on long cascades.
    pub fn after_auto_solve(previous: &InkIn, waves: &[Vec<Cell>], start_ms: u32) -> InkIn {
        let gaps = waves.len().saturating_sub(1) as u32;
        let gap_ms = LAST_WAVE_BY_MS
            .checked_div(gaps)
            .unwrap_or_default()
            .min(WAVE_GAP_MS);
        let mut delays_ms = [None; CELL_COUNT];
        for (index, wave) in waves.iter().enumerate() {
            for cell in wave {
                delays_ms[*cell] = Some(start_ms + index as u32 * gap_ms);
            }
        }
        InkIn {
            round: previous.round + 1,
            delays_ms,
        }
    }

    /// How long until every cell in `cells` has finished inking in.
    pub fn finished_ms(&self, cells: &[Cell]) -> u32 {
        cells
            .iter()
            .filter_map(|cell| self.delays_ms[*cell])
            .max()
            .map_or(0, |delay_ms| delay_ms + INK_IN_MS)
    }

    pub fn all_finished_ms(&self) -> u32 {
        let all_cells: Vec<Cell> = (0..CELL_COUNT).collect();
        self.finished_ms(&all_cells)
    }
}
