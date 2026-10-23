use std::fmt::Display;
use std::io;

use crate::constants::Color;
use crate::types::Rgb;

use owo_colors::{OwoColorize, Style, Styled};

pub(crate) fn status<T: Display>(text: T, ok: bool) -> Styled<T> {
    let style = if ok {
        Style::new().bright_green().bold()
    } else {
        Style::new().bright_red().bold()
    };

    style.style(text)
}

pub(crate) fn message<T: Display>(text: T, ok: bool) -> Styled<T> {
    let style = if ok {
        Style::new().bright_white().bold()
    } else {
        Style::new().red().bold()
    };

    style.style(text)
}

pub(crate) fn session<T: Display>(text: T, ok: bool) -> Styled<T> {
    let style = if ok {
        Style::new().white().bold()
    } else {
        Style::new().red().bold()
    };

    style.style(text)
}

pub(crate) fn username<T: Display>(text: T, ok: bool) -> Styled<T> {
    let style = if ok {
        let (r, g, b) = Color::ORANGE;
        Style::new().truecolor(r, g, b).bold()
    } else {
        Style::new().red().bold()
    };

    style.style(text)
}

pub(crate) fn hostname<T: Display>(text: T, ok: bool) -> Styled<T> {
    let style = if ok {
        let (r, g, b) = Color::PINK;
        Style::new().truecolor(r, g, b).bold()
    } else {
        Style::new().red().bold()
    };

    style.style(text)
}

pub(crate) fn label<T: Display>(text: T, ok: bool) -> Styled<T> {
    let style = if ok {
        Style::new().bright_cyan().bold()
    } else {
        Style::new().bright_red().bold()
    };

    style.style(text)
}

pub(crate) fn info<T: Display>(text: T, ok: bool) -> Styled<T> {
    let style = if ok {
        Style::new()
    } else {
        Style::new().red().bold()
    };

    style.style(text)
}

fn color_at(i: usize, len: usize, start: Rgb, end: Rgb) -> Rgb {
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

pub(crate) fn gradient_char<W: io::Write>(
    writer: &mut W,
    ch: char,
    i: usize,
    len: usize,
    start: Rgb,
    end: Rgb,
) -> io::Result<()> {
    let (r, g, b) = color_at(i, len, start, end);

    write!(writer, "{}", ch.truecolor(r, g, b))
}

pub(crate) fn gradient_string<W: io::Write>(
    writer: &mut W,
    string: &str,
    start: Rgb,
    end: Rgb,
) -> io::Result<()> {
    let chars = string.chars().count();

    for (i, ch) in string.chars().enumerate() {
        let (r, g, b) = color_at(i, chars, start, end);
        write!(writer, "{}", ch.truecolor(r, g, b))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_at_returns_endpoints() {
        let start = (10, 20, 30);
        let end = (110, 220, 250);

        assert_eq!(color_at(0, 2, start, end), start);
        assert_eq!(color_at(1, 2, start, end), end);
        assert_eq!(color_at(0, 1, start, end), start);
        assert_eq!(color_at(5, 1, start, end), start);
        assert_eq!(color_at(5, 0, start, end), start);
    }

    #[test]
    fn color_at_interpolates_midpoint() {
        assert_eq!(color_at(1, 3, (0, 0, 0), (100, 100, 100)), (50, 50, 50));
    }

    #[test]
    fn color_at_interpolates_each_channel_and_decreasing_values() {
        assert_eq!(
            color_at(1, 3, (10, 20, 30), (110, 220, 250)),
            (60, 120, 140)
        );
        assert_eq!(color_at(1, 3, (200, 100, 50), (0, 0, 0)), (100, 50, 25));
    }

    #[test]
    fn color_at_interpolates_non_midpoint_positions() {
        assert_eq!(color_at(1, 4, (0, 0, 0), (30, 60, 90)), (10, 20, 30));
        assert_eq!(color_at(2, 4, (0, 0, 0), (30, 60, 90)), (20, 40, 60));
    }

    #[test]
    fn color_at_handles_equal_colors_and_the_last_index() {
        let same = (10, 20, 30);

        assert_eq!(color_at(1, 3, same, same), same);
        assert_eq!(color_at(2, 3, (0, 0, 0), (200, 100, 50)), (200, 100, 50));
    }

    #[test]
    fn gradient_char_writes_exact_colors() {
        let cases = [
            (0, 2, "\x1b[38;2;96;165;250m│\x1b[39m"),
            (1, 2, "\x1b[38;2;255;105;180m│\x1b[39m"),
            (1, 3, "\x1b[38;2;175;135;215m│\x1b[39m"),
            (0, 1, "\x1b[38;2;96;165;250m│\x1b[39m"),
            (5, 0, "\x1b[38;2;96;165;250m│\x1b[39m"),
        ];

        for (index, len, expected) in cases {
            let mut output = Vec::new();

            gradient_char(&mut output, '│', index, len, Color::BLUE, Color::PINK).unwrap();

            assert_eq!(String::from_utf8(output).unwrap(), expected);
        }
    }

    #[test]
    fn gradient_string_writes_exact_colors_in_order() {
        let mut output = Vec::new();

        gradient_string(&mut output, "ab", Color::BLUE, Color::PINK).unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            "\x1b[38;2;96;165;250ma\x1b[39m\x1b[38;2;255;105;180mb\x1b[39m"
        );
    }

    #[test]
    fn gradient_string_handles_empty_and_single_unicode_character() {
        let mut empty = Vec::new();

        gradient_string(&mut empty, "", Color::BLUE, Color::PINK).unwrap();

        assert!(empty.is_empty());

        let mut single = Vec::new();

        gradient_string(&mut single, "─", Color::BLUE, Color::PINK).unwrap();

        assert_eq!(
            String::from_utf8(single).unwrap(),
            "\x1b[38;2;96;165;250m─\x1b[39m"
        );
    }

    #[test]
    fn gradient_string_interpolates_middle_unicode_character() {
        let mut output = Vec::new();

        gradient_string(&mut output, "a─b", Color::BLUE, Color::PINK).unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            concat!(
                "\x1b[38;2;96;165;250ma\x1b[39m",
                "\x1b[38;2;175;135;215m─\x1b[39m",
                "\x1b[38;2;255;105;180mb\x1b[39m"
            )
        );
    }

    #[test]
    fn gradient_helpers_propagate_write_errors() {
        struct FailingWriter;

        impl std::io::Write for FailingWriter {
            fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
                Err(std::io::Error::other("write failed"))
            }

            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }

        let error = gradient_char(&mut FailingWriter, 'x', 0, 1, Color::BLUE, Color::PINK)
            .expect_err("gradient_char should propagate writer errors");

        assert_eq!(error.kind(), std::io::ErrorKind::Other);

        let error = gradient_string(&mut FailingWriter, "x", Color::BLUE, Color::PINK)
            .expect_err("gradient_string should propagate writer errors");

        assert_eq!(error.kind(), std::io::ErrorKind::Other);
    }

    #[test]
    fn styled_text_has_exact_true_and_false_styles() {
        let cases = [
            ("status true", status("a─b", true), "\x1b[92;1ma─b\x1b[0m"),
            ("status false", status("a─b", false), "\x1b[91;1ma─b\x1b[0m"),
            ("message true", message("a─b", true), "\x1b[97;1ma─b\x1b[0m"),
            (
                "message false",
                message("a─b", false),
                "\x1b[31;1ma─b\x1b[0m",
            ),
            ("session true", session("a─b", true), "\x1b[37;1ma─b\x1b[0m"),
            (
                "session false",
                session("a─b", false),
                "\x1b[31;1ma─b\x1b[0m",
            ),
            (
                "username true",
                username("a─b", true),
                "\x1b[38;2;255;165;0;1ma─b\x1b[0m",
            ),
            (
                "username false",
                username("a─b", false),
                "\x1b[31;1ma─b\x1b[0m",
            ),
            (
                "hostname true",
                hostname("a─b", true),
                "\x1b[38;2;255;105;180;1ma─b\x1b[0m",
            ),
            (
                "hostname false",
                hostname("a─b", false),
                "\x1b[31;1ma─b\x1b[0m",
            ),
            ("label true", label("a─b", true), "\x1b[96;1ma─b\x1b[0m"),
            ("label false", label("a─b", false), "\x1b[91;1ma─b\x1b[0m"),
            ("info true", info("a─b", true), "a─b"),
            ("info false", info("a─b", false), "\x1b[31;1ma─b\x1b[0m"),
        ];

        for (name, actual, expected) in cases {
            assert_eq!(actual.to_string(), expected, "wrong style for {name}");
        }
    }
}
