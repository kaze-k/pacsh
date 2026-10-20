use std::io;

use crate::colors;
use crate::constants::Layout;
use crate::utils;

use super::Module;

use unicode_width::UnicodeWidthStr;

pub struct Os {
    label: &'static str,
    ok: bool,
    name: String,
}

impl Os {
    pub fn new(name: String) -> Self {
        Self {
            label: "OS",
            ok: utils::unknown(&name).is_none(),
            name,
        }
    }
}

impl Module for Os {
    fn render<W: io::Write>(&self, writer: &mut W, gap: usize) -> io::Result<()> {
        let padding = Layout::PADDING;
        let label = colors::label(self.label, self.ok);
        let name = colors::info(&self.name, self.ok);

        write!(writer, "{:padding$}{:<gap$}{name}", "", label)
    }

    fn width(&self, gap: usize) -> usize {
        Layout::PADDING + gap.max(self.label.width()) + self.name.width()
    }

    fn label(&self) -> Option<&str> {
        Some(self.label)
    }
}
