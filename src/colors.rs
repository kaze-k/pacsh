use std::fmt::Display;
use std::io::{self, Write};

use crate::constants::Color;
use crate::types::RGB;

use owo_colors::{OwoColorize, Style, Styled};

pub fn status<T: Display>(text: T, ok: bool) -> Styled<T> {
    let style = if ok {
        Style::new().bright_green().bold()
    } else {
        Style::new().bright_red().bold()
    };

    style.style(text)
}

pub fn message<T: Display>(text: T, ok: bool) -> Styled<T> {
    let style = if ok {
        Style::new().bright_white().bold()
    } else {
        Style::new().red().bold()
    };

    style.style(text)
}

pub fn session<T: Display>(text: T, ok: bool) -> Styled<T> {
    let style = if ok {
        Style::new().white().bold()
    } else {
        Style::new().red().bold()
    };

    style.style(text)
}

pub fn username<T: Display>(text: T, ok: bool) -> Styled<T> {
    let style = if ok {
        let (r, g, b) = Color::ORANGE;
        Style::new().truecolor(r, g, b).bold()
    } else {
        Style::new().red().bold()
    };

    style.style(text)
}

pub fn hostname<T: Display>(text: T, ok: bool) -> Styled<T> {
    let style = if ok {
        let (r, g, b) = Color::PINK;
        Style::new().truecolor(r, g, b).bold()
    } else {
        Style::new().red().bold()
    };

    style.style(text)
}

pub fn label<T: Display>(text: T, ok: bool) -> Styled<T> {
    let style = if ok {
        Style::new().bright_cyan().bold()
    } else {
        Style::new().bright_red().bold()
    };

    style.style(text)
}

pub fn info<T: Display>(text: T, ok: bool) -> Styled<T> {
    let style = if ok {
        Style::new()
    } else {
        Style::new().red().bold()
    };

    style.style(text)
}

fn color_at(i: usize, len: usize, start: RGB, end: RGB) -> RGB {
    let t = if len <= 1 {
        0.0
    } else {
        i as f32 / (len - 1) as f32
    };

    let r = start.0 as f32 + (end.0 as f32 - start.0 as f32) * t;
    let g = start.1 as f32 + (end.1 as f32 - start.1 as f32) * t;
    let b = start.2 as f32 + (end.2 as f32 - start.2 as f32) * t;

    (r as u8, g as u8, b as u8)
}

pub fn gradient_char<W: Write>(
    writer: &mut W,
    ch: char,
    i: usize,
    len: usize,
    start: RGB,
    end: RGB,
) -> io::Result<()> {
    let (r, g, b) = color_at(i, len, start, end);

    write!(writer, "{}", ch.truecolor(r, g, b))
}

pub fn gradient_string<W: Write>(
    writer: &mut W,
    string: &str,
    start: RGB,
    end: RGB,
) -> io::Result<()> {
    let chars = string.chars().count();

    for (i, ch) in string.chars().enumerate() {
        let (r, g, b) = color_at(i, chars, start, end);
        write!(writer, "{}", ch.truecolor(r, g, b))?;
    }

    Ok(())
}
