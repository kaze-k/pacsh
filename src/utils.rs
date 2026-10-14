use std::iter;

pub fn repeat(ch: char, count: usize) -> String {
    let mut text = String::new();

    text.reserve(count);
    text.extend(iter::repeat_n(ch, count));

    text
}

pub fn unknown(value: &str) -> Option<()> {
    if value == "unknown" { Some(()) } else { None }
}
