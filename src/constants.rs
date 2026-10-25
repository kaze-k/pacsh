use crate::types::Rgb;

pub(crate) struct Layout;
impl Layout {
    pub(crate) const INDENT: usize = 4;
    pub(crate) const PADDING: usize = 4;

    pub(crate) const STATUS_GAP: usize = 2;
}

pub(crate) struct Icon;
impl Icon {
    pub(crate) const PACMAN: char = '󰮯';
    pub(crate) const GHOST: char = '󰊠';
}

pub(crate) struct Color;
impl Color {
    pub(crate) const BLUE: Rgb = (96, 165, 250);
    pub(crate) const CYAN: Rgb = (45, 212, 191);
    pub(crate) const ORANGE: Rgb = (255, 165, 0);
    pub(crate) const PINK: Rgb = (255, 105, 180);
}

pub(crate) struct Border;
impl Border {
    pub(crate) const TOP_LEFT: char = '╭';
    pub(crate) const TOP_RIGHT: char = '╮';
    pub(crate) const BOTTOM_RIGHT: char = '╯';
    pub(crate) const BOTTOM_LEFT: char = '╰';

    pub(crate) const HORIZONTAL: char = '─';
    pub(crate) const VERTICAL: char = '│';
}
