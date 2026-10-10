use std::fmt;
use std::str::FromStr;
use std::sync::LazyLock;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub const CELL_COUNT: usize = 81;

/// Index of a cell, 0..81, counted left to right and top to bottom.
pub type Cell = usize;

/// A number from 1 to 9. 0 is used for an empty cell.
pub type Digit = u8;

/// A set of digits stored as bits 1..=9.
pub type DigitSet = u16;

pub const ALL_DIGITS: DigitSet = 0b11_1111_1110;

pub fn digit_bit(digit: Digit) -> DigitSet {
    1 << digit
}

pub fn digits_in(set: DigitSet) -> impl Iterator<Item = Digit> {
    (1..=9).filter(move |digit| set & digit_bit(*digit) != 0)
}

pub fn row_of(cell: Cell) -> usize {
    cell / 9
}

pub fn column_of(cell: Cell) -> usize {
    cell % 9
}

pub fn box_of(cell: Cell) -> usize {
    (row_of(cell) / 3) * 3 + column_of(cell) / 3
}

/// Human readable position, e.g. "row 3, column 5".
pub fn cell_name(cell: Cell) -> String {
    format!("row {}, column {}", row_of(cell) + 1, column_of(cell) + 1)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UnitKind {
    Row,
    Column,
    Box,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Unit {
    pub kind: UnitKind,
    pub index: usize,
}

impl Unit {
    pub fn cells(&self) -> [Cell; 9] {
        match self.kind {
            UnitKind::Row => std::array::from_fn(|i| self.index * 9 + i),
            UnitKind::Column => std::array::from_fn(|i| i * 9 + self.index),
            UnitKind::Box => {
                let top_left = (self.index / 3) * 27 + (self.index % 3) * 3;
                std::array::from_fn(|i| top_left + (i / 3) * 9 + i % 3)
            }
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let kind = match self.kind {
            UnitKind::Row => "row",
            UnitKind::Column => "column",
            UnitKind::Box => "box",
        };
        write!(formatter, "{kind} {}", self.index + 1)
    }
}

pub static UNITS: LazyLock<Vec<Unit>> = LazyLock::new(|| {
    [UnitKind::Row, UnitKind::Column, UnitKind::Box]
        .into_iter()
        .flat_map(|kind| (0..9).map(move |index| Unit { kind, index }))
        .collect()
});

pub fn units_of(cell: Cell) -> [Unit; 3] {
    [
        Unit {
            kind: UnitKind::Row,
            index: row_of(cell),
        },
        Unit {
            kind: UnitKind::Column,
            index: column_of(cell),
        },
        Unit {
            kind: UnitKind::Box,
            index: box_of(cell),
        },
    ]
}

/// The 20 other cells sharing a row, column or box with each cell.
pub static PEERS: LazyLock<Vec<Vec<Cell>>> = LazyLock::new(|| {
    (0..CELL_COUNT)
        .map(|cell| {
            (0..CELL_COUNT)
                .filter(|other| *other != cell && sees(cell, *other))
                .collect()
        })
        .collect()
});

pub fn sees(cell: Cell, other: Cell) -> bool {
    row_of(cell) == row_of(other)
        || column_of(cell) == column_of(other)
        || box_of(cell) == box_of(other)
}

/// A 9x9 board of digits, where 0 means empty.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Grid {
    cells: [Digit; CELL_COUNT],
}

impl Grid {
    pub fn empty() -> Self {
        Grid {
            cells: [0; CELL_COUNT],
        }
    }

    pub fn get(&self, cell: Cell) -> Digit {
        self.cells[cell]
    }

    pub fn set(&mut self, cell: Cell, digit: Digit) {
        self.cells[cell] = digit;
    }

    pub fn is_empty(&self, cell: Cell) -> bool {
        self.cells[cell] == 0
    }

    pub fn is_complete(&self) -> bool {
        self.cells.iter().all(|digit| *digit != 0)
    }

    pub fn filled_count(&self) -> usize {
        self.cells.iter().filter(|digit| **digit != 0).count()
    }

    /// Digits not yet used by the cell's row, column or box.
    pub fn candidates(&self, cell: Cell) -> DigitSet {
        let used = PEERS[cell]
            .iter()
            .fold(0, |used, peer| used | digit_bit(self.cells[*peer]));
        ALL_DIGITS & !used
    }

    /// True when no digit repeats within any row, column or box.
    pub fn is_consistent(&self) -> bool {
        UNITS.iter().all(|unit| {
            let mut seen: DigitSet = 0;
            unit.cells().iter().all(|cell| {
                let digit = self.cells[*cell];
                let repeated = digit != 0 && seen & digit_bit(digit) != 0;
                seen |= digit_bit(digit);
                !repeated
            })
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseGridError(String);

impl fmt::Display for ParseGridError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for ParseGridError {}

impl FromStr for Grid {
    type Err = ParseGridError;

    /// Parses 81 characters, using 1-9 for digits and 0 or . for empty cells.
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let digits: Vec<Digit> = text
            .chars()
            .filter(|character| !character.is_whitespace())
            .map(|character| match character {
                '.' => Ok(0),
                '0'..='9' => Ok(character as Digit - b'0'),
                other => Err(ParseGridError(format!("unexpected character '{other}'"))),
            })
            .collect::<Result<_, _>>()?;
        let cells = digits.try_into().map_err(|digits: Vec<Digit>| {
            ParseGridError(format!("expected 81 cells, found {}", digits.len()))
        })?;
        Ok(Grid { cells })
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.cells
            .iter()
            .try_for_each(|digit| write!(formatter, "{digit}"))
    }
}

impl Serialize for Grid {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Grid {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        text.parse().map_err(serde::de::Error::custom)
    }
}
