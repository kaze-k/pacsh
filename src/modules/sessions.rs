use std::io::{self, Write};

use crate::colors;
use crate::constants::Layout;
use crate::utils::unknown;

use super::Module;

use unicode_width::UnicodeWidthStr;

pub struct Sessions {
    label: &'static str,
    ok: bool,
    count: String,
}

impl Sessions {
    pub fn new(count: String) -> Self {
        Self {
            label: "SESSIONS",
            ok: unknown(&count).is_none(),
            count,
        }
    }
}

impl Module for Sessions {
    fn render<W: Write>(&self, writer: &mut W, gap: usize) -> io::Result<()> {
        let padding = Layout::PADDING;
        let label = colors::label(self.label, self.ok);
        let count = colors::info(&self.count, self.ok);

        write!(writer, "{:padding$}{:<gap$}{count}", "", label)
    }

    fn width(&self, gap: usize) -> usize {
        Layout::PADDING + gap.max(self.label.width()) + self.count.width()
    }

    fn label(&self) -> Option<&str> {
        Some(self.label)
    }
}
