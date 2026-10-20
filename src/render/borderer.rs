use std::io;

use crate::colors;
use crate::constants::{Border, Color, Icon, Layout};
use crate::utils;

use unicode_width::UnicodeWidthChar;

pub struct Borderer;

impl Borderer {
    pub fn new() -> Self {
        Self
    }

    pub fn top<W: io::Write>(&self, writer: &mut W, width: usize) -> io::Result<()> {
        let count =
            (width + Layout::PADDING) - 2 - 1 - Icon::PACMAN.width().unwrap_or_default() - 1;

        let string = format!(
            "{}{} {} {}{}",
            Border::TOP_LEFT,
            utils::repeat(Border::HORIZONTAL, 2),
            Icon::PACMAN,
            utils::repeat(Border::HORIZONTAL, count),
            Border::TOP_RIGHT
        );

        colors::gradient_string(writer, &string, Color::BLUE, Color::PINK)
    }

    pub fn bottom<W: io::Write>(&self, writer: &mut W, width: usize) -> io::Result<()> {
        let count = (width + Layout::PADDING) - 2 - 1 - Icon::GHOST.width().unwrap_or_default() - 1;

        let string = format!(
            "{}{} {} {}{}",
            Border::BOTTOM_LEFT,
            utils::repeat(Border::HORIZONTAL, count),
            Icon::GHOST,
            utils::repeat(Border::HORIZONTAL, 2),
            Border::BOTTOM_RIGHT
        );

        colors::gradient_string(writer, &string, Color::ORANGE, Color::CYAN)
    }

    pub fn left<W: io::Write>(
        &self,
        writer: &mut W,
        index: usize,
        height: usize,
    ) -> io::Result<()> {
        colors::gradient_char(
            writer,
            Border::VERTICAL,
            index,
            height,
            Color::BLUE,
            Color::ORANGE,
        )
    }

    pub fn right<W: io::Write>(
        &self,
        writer: &mut W,
        index: usize,
        height: usize,
    ) -> io::Result<()> {
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
