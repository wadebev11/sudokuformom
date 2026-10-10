use leptos::prelude::*;
use sudoku_core::Technique;
use sudoku_core::grid::{Digit, DigitSet, digit_bit};

use crate::board::{edge_classes, pencil_marks_view};

/// A corner of a board, starting at the top left, showing a Technique at work.
///
/// Each row is a list of cells: `5` is a Given, `.` is empty, `=5` is the
/// number the Technique places, and `{12-3}` is notes of 1 and 2 with 3
/// crossed out. A trailing `*` highlights the cells that make up the pattern.
struct Example {
    rows: &'static [&'static str],
    caption: &'static str,
}

fn example(technique: Technique) -> Example {
    match technique {
        Technique::HiddenSingle => Example {
            rows: &[
                "5 . . . . 4* . . .",
                "=4 3 7 . . . . . .",
                "1 . 9 . . . . 4* .",
            ],
            caption: "The 4s in the top and bottom rows rule out the other empty cells in the left box, so its 4 goes here.",
        },
        Technique::NakedSingle => Example {
            rows: &[
                "3 . 6 . . . . . .",
                "1 4 =7* 8 . 9 . 5 .",
                ". 2 . . . . . . .",
            ],
            caption: "Between its row and its box, every number but 7 is already used.",
        },
        Technique::Pointing => Example {
            rows: &[
                "{38}* {13}* 5 . {6-3} . . {27-3} .",
                "2 7 9 . . . . . .",
                "4 6 {18} . . . . . .",
            ],
            caption: "In the left box, 3 can only go in the top row. So the rest of the top row can't be 3.",
        },
        Technique::BoxLineReduction => Example {
            rows: &[
                "{38}* {13}* 5 2 7 9 4 6 {18}",
                ". . . . . . . 3 .",
                "{6-3} {2-3} . . . . . . .",
            ],
            caption: "In the top row, 3 can only go in the left box. So the rest of that box can't be 3.",
        },
        Technique::NakedPair => Example {
            rows: &["1 {27}* 5 {27}* 8 {36-2} {46-7} 9 {34-27}"],
            caption: "The highlighted cells can only be 2 or 7, so no other cell in the row can be.",
        },
        Technique::HiddenPair => Example {
            rows: &["1 {27-36}* 5 {27-4}* 8 {36} {46} 9 {34}"],
            caption: "2 and 7 only fit in the highlighted cells, so those cells can't be anything else.",
        },
        Technique::NakedTriple => Example {
            rows: &["{15}* {18}* 3 {58}* 4 {26-1} 9 {67-5} {27-8}"],
            caption: "The highlighted cells can only be 1, 5 or 8, so no other cell in the row can be.",
        },
        Technique::HiddenTriple => Example {
            rows: &["{15-26}* {18-7}* 3 {58-2}* 4 {26} 9 {67} {27}"],
            caption: "1, 5 and 8 only fit in the highlighted cells, so those cells can't be anything else.",
        },
        Technique::XWing => Example {
            rows: &[
                "3 {15}* 8 2 {69} 4 {15}* 7 {69}",
                ". {2-5} . . . . {3-5} . .",
                "6 {25}* 1 9 3 {78} {25}* 4 {78}",
            ],
            caption: "In the top and bottom rows, 5 only fits in the highlighted corners. Those corners take the 5 for both columns, so the rest of each column can't be 5.",
        },
        Technique::NakedQuad => Example {
            rows: &["{12}* {23}* {34}* {14}* 9 {56-1} {57-23} 8 {67-4}"],
            caption: "The highlighted cells can only be 1, 2, 3 or 4, so no other cell in the row can be.",
        },
        Technique::HiddenQuad => Example {
            rows: &["{12-5}* {23-67}* {34-5}* {14-7}* 9 {56} {57} 8 {67}"],
            caption: "1, 2, 3 and 4 only fit in the highlighted cells, so those cells can't be anything else.",
        },
        Technique::XYWing => Example {
            rows: &[
                ". {12}* {5-3} . . . {13}* . .",
                ". . . . . . . . .",
                "{23}* . . . . . . {6-3} .",
            ],
            caption: "If the 1 or 2 cell is 1, the cell on the right is 3. If it's 2, the cell below is 3. Either way, a cell that sees both can't be 3.",
        },
        Technique::XYZWing => Example {
            rows: &[
                ". {123}* {5-3} . . . {13}* . .",
                ". . . . . . . . .",
                "{23}* . . . . . . . .",
            ],
            caption: "Whatever the 1, 2 or 3 cell turns out to be, one of the highlighted cells is 3. So a cell that sees all three can't be 3.",
        },
        Technique::Swordfish => Example {
            rows: &[
                "6 {24}* 1 3 {48}* 9 5 {28} 7",
                ". {7-4} . . . . . {1-4} .",
                "2 {69} 5 1 {46}* 3 7 {49}* 8",
                ". . . . {5-4} . . . .",
                "9 {34}* 8 6 {37} 2 5 {47}* 1",
            ],
            caption: "In the three full rows, 4 only fits in the highlighted cells, which sit in three columns. Each of those columns gets its 4 from these rows, so the rest of each column can't be 4.",
        },
    }
}

#[derive(Debug, PartialEq)]
enum Contents {
    Empty,
    Given(Digit),
    Placed(Digit),
    Notes {
        kept: DigitSet,
        crossed_out: DigitSet,
    },
}

#[derive(Debug, PartialEq)]
struct ExampleCell {
    contents: Contents,
    is_highlighted: bool,
}

impl ExampleCell {
    fn parse(token: &str) -> ExampleCell {
        let contents_token = token.trim_end_matches('*');
        let contents = if contents_token == "." {
            Contents::Empty
        } else if let Some(digit) = contents_token.strip_prefix('=') {
            Contents::Placed(parse_digit(digit))
        } else if let Some(notes) = contents_token
            .strip_prefix('{')
            .and_then(|notes| notes.strip_suffix('}'))
        {
            let (kept, crossed_out) = notes.split_once('-').unwrap_or((notes, ""));
            Contents::Notes {
                kept: parse_digit_set(kept),
                crossed_out: parse_digit_set(crossed_out),
            }
        } else {
            Contents::Given(parse_digit(contents_token))
        };
        ExampleCell {
            contents,
            is_highlighted: token.ends_with('*'),
        }
    }

    fn is_changed_by_technique(&self) -> bool {
        match self.contents {
            Contents::Placed(_) => true,
            Contents::Notes { crossed_out, .. } => crossed_out != 0,
            Contents::Empty | Contents::Given(_) => false,
        }
    }
}

fn parse_digit(text: &str) -> Digit {
    text.parse()
        .unwrap_or_else(|_| panic!("{text:?} isn't a number in a technique example"))
}

fn parse_digit_set(text: &str) -> DigitSet {
    text.chars()
        .map(|character| parse_digit(&character.to_string()))
        .fold(0, |set, digit| set | digit_bit(digit))
}

fn parse_rows(rows: &[&str]) -> Vec<Vec<ExampleCell>> {
    rows.iter()
        .map(|row| row.split_whitespace().map(ExampleCell::parse).collect())
        .collect()
}

#[component]
pub fn TechniqueExample(technique: Technique) -> impl IntoView {
    let Example { rows, caption } = example(technique);
    let rows = parse_rows(rows);
    let last_row = rows.len() - 1;
    let last_column = rows[0].len() - 1;

    view! {
        <figure class="technique-example">
            <div
                class="board"
                style=format!("--rows: {}; --columns: {}", last_row + 1, last_column + 1)
                aria-hidden="true"
            >
                {rows
                    .into_iter()
                    .enumerate()
                    .flat_map(|(row, cells)| {
                        cells.into_iter().enumerate().map(move |(column, cell)| (row, column, cell))
                    })
                    .map(|(row, column, cell)| {
                        let mut class = format!(
                            "cell{}",
                            edge_classes(row, column, last_row, last_column),
                        );
                        if matches!(cell.contents, Contents::Given(_)) {
                            class.push_str(" given");
                        }
                        if cell.is_highlighted {
                            class.push_str(" hinted");
                        }
                        if cell.is_changed_by_technique() {
                            class.push_str(" hint-target");
                        }
                        let contents = match cell.contents {
                            Contents::Empty => ().into_any(),
                            Contents::Given(digit) | Contents::Placed(digit) => digit.into_any(),
                            Contents::Notes { kept, crossed_out } => {
                                pencil_marks_view(kept, crossed_out).into_any()
                            }
                        };
                        view! { <div class=class>{contents}</div> }
                    })
                    .collect_view()}
            </div>
            <figcaption>{caption}</figcaption>
        </figure>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_example_is_a_rectangle_of_cells() {
        for technique in Technique::ALL {
            let rows = parse_rows(example(technique).rows);
            let width = rows[0].len();
            assert!(
                rows.iter().all(|row| row.len() == width),
                "{} example has uneven rows",
                technique.name()
            );
        }
    }

    #[test]
    fn every_example_shows_what_the_technique_changes() {
        for technique in Technique::ALL {
            let rows = parse_rows(example(technique).rows);
            assert!(
                rows.iter()
                    .flatten()
                    .any(ExampleCell::is_changed_by_technique),
                "{} example changes nothing",
                technique.name()
            );
        }
    }

    #[test]
    fn notes_split_into_kept_and_crossed_out() {
        assert_eq!(
            ExampleCell::parse("{34-27}*"),
            ExampleCell {
                contents: Contents::Notes {
                    kept: digit_bit(3) | digit_bit(4),
                    crossed_out: digit_bit(2) | digit_bit(7),
                },
                is_highlighted: true,
            }
        );
    }
}
