use std::io::{self, Write};

use crate::colors::{self, gradient_string};
use crate::constants::{Border, Color, Icon, Layout};
use crate::utils::repeat;

use unicode_width::UnicodeWidthChar;

pub struct Borderer;

impl Borderer {
    pub fn new() -> Self {
        Self
    }

    pub fn top<W: Write>(&self, writer: &mut W, width: usize) -> io::Result<()> {
        let count =
            (width + Layout::PADDING) - 2 - 1 - Icon::PACMAN.width().unwrap_or_default() - 1;

        let string = format!(
            "{}{} {} {}{}",
            Border::TOP_LEFT,
            repeat(Border::HORIZONTAL, 2),
            Icon::PACMAN,
            repeat(Border::HORIZONTAL, count),
            Border::TOP_RIGHT
        );

        gradient_string(writer, &string, Color::BLUE, Color::PINK)
    }

    pub fn bottom<W: Write>(&self, writer: &mut W, width: usize) -> io::Result<()> {
        let count = (width + Layout::PADDING) - 2 - 1 - Icon::GHOST.width().unwrap_or_default() - 1;

        let string = format!(
            "{}{} {} {}{}",
            Border::BOTTOM_LEFT,
            repeat(Border::HORIZONTAL, count),
            Icon::GHOST,
            repeat(Border::HORIZONTAL, 2),
            Border::BOTTOM_RIGHT
        );

        colors::gradient_string(writer, &string, Color::ORANGE, Color::CYAN)
    }

    pub fn left<W: Write>(&self, writer: &mut W, index: usize, height: usize) -> io::Result<()> {
        colors::gradient_char(
            writer,
            Border::VERTICAL,
            index,
            height,
            Color::BLUE,
            Color::ORANGE,
        )
    }

    pub fn right<W: Write>(&self, writer: &mut W, index: usize, height: usize) -> io::Result<()> {
        colors::gradient_char(
            writer,
            Border::VERTICAL,
            index,
            height,
            Color::PINK,
            Color::CYAN,
        )
    }
}
