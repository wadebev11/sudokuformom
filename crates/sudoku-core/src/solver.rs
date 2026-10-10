use crate::grid::{
    ALL_DIGITS, CELL_COUNT, Cell, Digit, DigitSet, Grid, box_of, column_of, digit_bit, digits_in,
    row_of,
};

/// Brute-force search used to check that a puzzle has exactly one solution.
struct Search {
    grid: Grid,
    rows: [DigitSet; 9],
    columns: [DigitSet; 9],
    boxes: [DigitSet; 9],
    solutions_found: usize,
    limit: usize,
    first_solution: Option<Grid>,
}

impl Search {
    fn new(grid: &Grid, limit: usize) -> Option<Self> {
        let mut search = Search {
            grid: *grid,
            rows: [0; 9],
            columns: [0; 9],
            boxes: [0; 9],
            solutions_found: 0,
            limit,
            first_solution: None,
        };
        let consistent = grid.is_consistent();
        if consistent {
            (0..CELL_COUNT)
                .filter(|cell| !grid.is_empty(*cell))
                .for_each(|cell| search.mark(cell, grid.get(cell)));
        }
        consistent.then_some(search)
    }

    fn options(&self, cell: Cell) -> DigitSet {
        ALL_DIGITS
            & !(self.rows[row_of(cell)] | self.columns[column_of(cell)] | self.boxes[box_of(cell)])
    }

    fn mark(&mut self, cell: Cell, digit: Digit) {
        self.rows[row_of(cell)] |= digit_bit(digit);
        self.columns[column_of(cell)] |= digit_bit(digit);
        self.boxes[box_of(cell)] |= digit_bit(digit);
        self.grid.set(cell, digit);
    }

    fn unmark(&mut self, cell: Cell, digit: Digit) {
        self.rows[row_of(cell)] &= !digit_bit(digit);
        self.columns[column_of(cell)] &= !digit_bit(digit);
        self.boxes[box_of(cell)] &= !digit_bit(digit);
        self.grid.set(cell, 0);
    }

    /// Picks the empty cell with the fewest options to keep the search small.
    fn most_constrained_cell(&self) -> Option<(Cell, DigitSet)> {
        (0..CELL_COUNT)
            .filter(|cell| self.grid.is_empty(*cell))
            .map(|cell| (cell, self.options(cell)))
            .min_by_key(|(_, options)| options.count_ones())
    }

    fn run(&mut self) {
        match self.most_constrained_cell() {
            None => {
                self.solutions_found += 1;
                self.first_solution.get_or_insert(self.grid);
            }
            Some((cell, options)) => {
                for digit in digits_in(options) {
                    if self.solutions_found < self.limit {
                        self.mark(cell, digit);
                        self.run();
                        self.unmark(cell, digit);
                    }
                }
            }
        }
    }
}

/// Counts solutions, stopping once `limit` have been found.
pub fn count_solutions(grid: &Grid, limit: usize) -> usize {
    Search::new(grid, limit).map_or(0, |mut search| {
        search.run();
        search.solutions_found
    })
}

pub fn solve(grid: &Grid) -> Option<Grid> {
    Search::new(grid, 1).and_then(|mut search| {
        search.run();
        search.first_solution
    })
}

pub fn has_unique_solution(grid: &Grid) -> bool {
    count_solutions(grid, 2) == 1
}
