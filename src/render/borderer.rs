use std::io;

use crate::colors;
use crate::constants::{Border, Color, Icon, Layout};
use crate::utils;

use unicode_width::UnicodeWidthChar;

pub(crate) struct Borderer;

impl Borderer {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn top<W: io::Write>(&self, writer: &mut W, width: usize) -> io::Result<()> {
        let count = (width + Layout::PADDING)
            - 2
            - 1
            - Icon::PACMAN
                .width()
                .expect("failed to get PACMAN icon width: width should be available")
            - 1;

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

    pub(crate) fn bottom<W: io::Write>(&self, writer: &mut W, width: usize) -> io::Result<()> {
        let count = (width + Layout::PADDING)
            - 2
            - 1
            - Icon::GHOST
                .width()
                .expect("failed to get GHOST icon width: width should be available")
            - 1;

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

    pub(crate) fn left<W: io::Write>(
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

    pub(crate) fn right<W: io::Write>(
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_TOP: &str = concat!(
        "\x1b[38;2;96;165;250m╭\x1b[39m",
        "\x1b[38;2;106;161;245m─\x1b[39m",
        "\x1b[38;2;117;157;240m─\x1b[39m",
        "\x1b[38;2;127;153;236m \x1b[39m",
        "\x1b[38;2;138;149;231m󰮯\x1b[39m",
        "\x1b[38;2;149;145;226m \x1b[39m",
        "\x1b[38;2;159;141;222m─\x1b[39m",
        "\x1b[38;2;170;137;217m─\x1b[39m",
        "\x1b[38;2;180;133;212m─\x1b[39m",
        "\x1b[38;2;191;129;208m─\x1b[39m",
        "\x1b[38;2;202;125;203m─\x1b[39m",
        "\x1b[38;2;212;121;198m─\x1b[39m",
        "\x1b[38;2;223;117;194m─\x1b[39m",
        "\x1b[38;2;233;113;189m─\x1b[39m",
        "\x1b[38;2;244;109;184m─\x1b[39m",
        "\x1b[38;2;255;105;180m╮\x1b[39m",
    );

    const EXPECTED_BOTTOM: &str = concat!(
        "\x1b[38;2;255;165;0m╰\x1b[39m",
        "\x1b[38;2;241;168;12m─\x1b[39m",
        "\x1b[38;2;227;171;25m─\x1b[39m",
        "\x1b[38;2;213;174;38m─\x1b[39m",
        "\x1b[38;2;199;177;50m─\x1b[39m",
        "\x1b[38;2;185;180;63m─\x1b[39m",
        "\x1b[38;2;171;183;76m─\x1b[39m",
        "\x1b[38;2;157;186;89m─\x1b[39m",
        "\x1b[38;2;143;190;101m─\x1b[39m",
        "\x1b[38;2;129;193;114m─\x1b[39m",
        "\x1b[38;2;115;196;127m \x1b[39m",
        "\x1b[38;2;101;199;140m󰊠\x1b[39m",
        "\x1b[38;2;87;202;152m \x1b[39m",
        "\x1b[38;2;73;205;165m─\x1b[39m",
        "\x1b[38;2;59;208;178m─\x1b[39m",
        "\x1b[38;2;45;212;191m╯\x1b[39m",
    );

    const LEFT_0_1: &str = "\x1b[38;2;96;165;250m│\x1b[39m";
    const LEFT_1_2: &str = "\x1b[38;2;255;165;0m│\x1b[39m";
    const LEFT_5_10: &str = "\x1b[38;2;184;165;111m│\x1b[39m";
    const LEFT_9_10: &str = "\x1b[38;2;255;165;0m│\x1b[39m";

    const RIGHT_0_1: &str = "\x1b[38;2;255;105;180m│\x1b[39m";
    const RIGHT_1_2: &str = "\x1b[38;2;45;212;191m│\x1b[39m";
    const RIGHT_2_2: &str = "\x1b[38;2;0;255;202m│\x1b[39m";
    const RIGHT_5_10: &str = "\x1b[38;2;138;164;186m│\x1b[39m";
    const RIGHT_9_10: &str = "\x1b[38;2;45;212;191m│\x1b[39m";

    #[test]
    fn top_border_has_exact_layout_and_colors() {
        let borderer = Borderer::new();
        let mut output = Vec::new();

        borderer.top(&mut output, 10).unwrap();

        assert_eq!(String::from_utf8(output).unwrap(), EXPECTED_TOP);
    }

    #[test]
    fn bottom_border_has_exact_layout_and_colors() {
        let borderer = Borderer::new();
        let mut output = Vec::new();

        borderer.bottom(&mut output, 10).unwrap();

        assert_eq!(String::from_utf8(output).unwrap(), EXPECTED_BOTTOM);
    }

    #[test]
    fn side_borders_have_exact_character_and_colors() {
        let borderer = Borderer::new();
        let cases = [
            (0, 0, LEFT_0_1, RIGHT_0_1),
            (0, 1, LEFT_0_1, RIGHT_0_1),
            (1, 2, LEFT_1_2, RIGHT_1_2),
            (5, 10, LEFT_5_10, RIGHT_5_10),
            (9, 10, LEFT_9_10, RIGHT_9_10),
            (2, 2, LEFT_1_2, RIGHT_2_2),
        ];

        for (index, height, expected_left, expected_right) in cases {
            let mut left = Vec::new();
            borderer.left(&mut left, index, height).unwrap();

            assert_eq!(String::from_utf8(left).unwrap(), expected_left);

            let mut right = Vec::new();
            borderer.right(&mut right, index, height).unwrap();

            assert_eq!(String::from_utf8(right).unwrap(), expected_right);
        }
    }

    #[test]
    fn width_one_border_has_exact_layout_and_colors() {
        let borderer = Borderer::new();
        let mut top = Vec::new();
        let mut bottom = Vec::new();

        borderer.top(&mut top, 1).unwrap();
        borderer.bottom(&mut bottom, 1).unwrap();

        assert_eq!(
            String::from_utf8(top).unwrap(),
            concat!(
                "\x1b[38;2;96;165;250m╭\x1b[39m",
                "\x1b[38;2;122;155;238m─\x1b[39m",
                "\x1b[38;2;149;145;226m─\x1b[39m",
                "\x1b[38;2;175;135;215m \x1b[39m",
                "\x1b[38;2;202;125;203m󰮯\x1b[39m",
                "\x1b[38;2;228;115;191m \x1b[39m",
                "\x1b[38;2;255;105;180m╮\x1b[39m",
            )
        );
        assert_eq!(
            String::from_utf8(bottom).unwrap(),
            concat!(
                "\x1b[38;2;255;165;0m╰\x1b[39m",
                "\x1b[38;2;220;172;31m \x1b[39m",
                "\x1b[38;2;185;180;63m󰊠\x1b[39m",
                "\x1b[38;2;150;188;95m \x1b[39m",
                "\x1b[38;2;115;196;127m─\x1b[39m",
                "\x1b[38;2;80;204;159m─\x1b[39m",
                "\x1b[38;2;45;212;191m╯\x1b[39m",
            )
        );
    }
}
