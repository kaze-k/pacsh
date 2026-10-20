use std::io;

use crate::colors;
use crate::constants::Layout;

use super::Module;

use unicode_width::UnicodeWidthStr;

pub struct Access {
    label: &'static str,
    is_ssh: bool,
}

impl Access {
    pub fn new(is_ssh: bool) -> Self {
        Self {
            label: if is_ssh { "[ok]" } else { "[fail]" },
            is_ssh,
        }
    }

    fn message(&self) -> &str {
        if self.is_ssh {
            "SSH LOGIN SUCCESSFUL"
        } else {
            "SSH LOGIN FAILED"
        }
    }
}

impl Module for Access {
    fn render<W: io::Write>(&self, writer: &mut W, gap: usize) -> io::Result<()> {
        let message = self.message();

        let padding = Layout::PADDING;
        let status = colors::status(self.label, self.is_ssh);
        let message = colors::message(message, self.is_ssh);

        write!(writer, "{:padding$}{:<gap$}{message}", "", status)
    }

    fn width(&self, gap: usize) -> usize {
        Layout::PADDING + gap.max(self.label.width()) + self.message().width()
    }

    fn label(&self) -> Option<&str> {
        Some(self.label)
    }
}
