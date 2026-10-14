use crate::types::RGB;

pub struct Layout;
impl Layout {
    pub const INDENT: usize = 4;
    pub const PADDING: usize = 4;

    pub const STATUS_GAP: usize = 2;
}

pub struct Icon;
impl Icon {
    pub const PACMAN: char = '󰮯';
    pub const GHOST: char = '󰊠';
}

pub struct Color;
impl Color {
    pub const BLUE: RGB = (96, 165, 250);
    pub const CYAN: RGB = (45, 212, 191);
    pub const ORANGE: RGB = (255, 165, 0);
    pub const PINK: RGB = (255, 105, 180);
}

pub struct Border;
impl Border {
    pub const TOP_LEFT: char = '╭';
    pub const TOP_RIGHT: char = '╮';
    pub const BOTTOM_RIGHT: char = '╯';
    pub const BOTTOM_LEFT: char = '╰';

    pub const HORIZONTAL: char = '─';
    pub const VERTICAL: char = '│';
}
