use std::fmt::Display;

/// Joins items the way a sentence would: "3", "3 or 8", "3, 5 or 8".
pub(super) fn join<T: Display>(items: impl IntoIterator<Item = T>, conjunction: &str) -> String {
    let words: Vec<String> = items.into_iter().map(|item| item.to_string()).collect();
    match words.split_last() {
        Some((last, rest)) if !rest.is_empty() => {
            format!("{} {conjunction} {last}", rest.join(", "))
        }
        _ => words.concat(),
    }
}

pub(super) fn count_word(count: usize) -> &'static str {
    ["zero", "one", "two", "three", "four"][count]
}
