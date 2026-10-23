use std::io;

use crate::colors;
use crate::constants::Layout;
use crate::utils;

use super::Module;

use unicode_width::UnicodeWidthStr;

pub(crate) struct Sessions {
    label: &'static str,
    ok: bool,
    count: String,
}

impl Sessions {
    pub(crate) fn new(count: String) -> Self {
        Self {
            label: "SESSIONS",
            ok: utils::unknown(&count).is_none(),
            count,
        }
    }
}

impl Module for Sessions {
    fn render<W: io::Write>(&self, writer: &mut W, gap: usize) -> io::Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::render_to_string;

    #[test]
    fn renders_session_count() {
        let module = Sessions::new("2".to_string());
        let expected = format!(
            "    {:<10}{}",
            colors::label("SESSIONS", true),
            colors::info("2", true),
        );

        assert_eq!(module.label(), Some("SESSIONS"));
        assert_eq!(module.width(10), 15);
        assert_eq!(render_to_string(&module, 10), expected);
    }

    #[test]
    fn renders_unknown_session_count() {
        let module = Sessions::new("unknown".to_string());
        let expected = format!(
            "    {:<10}{}",
            colors::label("SESSIONS", false),
            colors::info("unknown", false),
        );

        assert_eq!(module.label(), Some("SESSIONS"));
        assert_eq!(module.width(10), 21);
        assert_eq!(render_to_string(&module, 10), expected);
    }

    #[test]
    fn renders_zero_multi_digit_and_empty_counts() {
        let cases = [("0", 15), ("42", 16), ("", 14)];

        for (count, expected_width) in cases {
            let module = Sessions::new(count.to_string());
            let expected = format!(
                "    {:<10}{}",
                colors::label("SESSIONS", true),
                colors::info(count, true),
            );

            assert_eq!(module.width(10), expected_width);
            assert_eq!(render_to_string(&module, 10), expected);
        }
    }

    #[test]
    fn zero_gap_keeps_label_and_count_separate() {
        let module = Sessions::new("2".to_string());
        let expected = format!(
            "    {}{}",
            colors::label("SESSIONS", true),
            colors::info("2", true),
        );

        assert_eq!(module.width(0), 13);
        assert_eq!(render_to_string(&module, 0), expected);
    }
}
