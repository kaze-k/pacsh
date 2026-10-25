use std::io;

use crate::colors;
use crate::constants::Layout;
use crate::utils;

use super::Module;

use unicode_width::UnicodeWidthStr;

pub(crate) struct Tty {
    label: &'static str,
    ok: bool,
    name: String,
}

impl Tty {
    pub(crate) fn new(name: String) -> Self {
        Self {
            label: "TTY",
            ok: utils::unknown(&name).is_none(),
            name,
        }
    }
}

impl Module for Tty {
    fn render<W: io::Write>(&self, writer: &mut W, gap: usize) -> io::Result<()> {
        let padding = Layout::PADDING;
        let label = colors::label(self.label, self.ok);
        let tty = colors::info(&self.name, self.ok);

        write!(writer, "{:padding$}{:<gap$}{tty}", "", label)
    }

    fn width(&self, gap: usize) -> usize {
        Layout::PADDING + gap.max(self.label.width()) + self.name.width()
    }

    fn label(&self) -> Option<&str> {
        Some(self.label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::render_to_string;

    #[test]
    fn renders_tty_name() {
        let module = Tty::new("/dev/pts/0".to_string());
        let expected = format!(
            "    {:<10}{}",
            colors::label("TTY", true),
            colors::info("/dev/pts/0", true),
        );

        assert_eq!(module.label(), Some("TTY"));
        assert_eq!(module.width(10), 24);
        assert_eq!(render_to_string(&module, 10), expected);
    }

    #[test]
    fn marks_unknown_tty_as_failed() {
        let module = Tty::new("unknown".to_string());
        let expected = format!(
            "    {:<10}{}",
            colors::label("TTY", false),
            colors::info("unknown", false),
        );

        assert_eq!(module.width(10), 21);
        assert_eq!(render_to_string(&module, 10), expected);
    }

    #[test]
    fn empty_name_and_zero_gap_keep_label_only() {
        let module = Tty::new(String::new());
        let expected = format!("    {}", colors::label("TTY", true));

        assert_eq!(module.width(0), 7);
        assert_eq!(render_to_string(&module, 0), expected);
    }
}
