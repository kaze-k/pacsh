use std::io;

use crate::colors;
use crate::constants::Layout;

use super::Module;

use unicode_width::UnicodeWidthStr;

pub(crate) struct Access {
    label: &'static str,
    is_ssh: bool,
}

impl Access {
    pub(crate) fn new(is_ssh: bool) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::render_to_string;

    #[test]
    fn ssh_login_reports_success() {
        let module = Access::new(true);
        let expected = format!(
            "    {:<10}{}",
            colors::status("[ok]", true),
            colors::message("SSH LOGIN SUCCESSFUL", true),
        );

        assert_eq!(module.label(), Some("[ok]"));
        assert_eq!(module.width(10), 34);
        assert_eq!(render_to_string(&module, 10), expected);
    }

    #[test]
    fn non_ssh_login_reports_failure() {
        let module = Access::new(false);
        let expected = format!(
            "    {:<10}{}",
            colors::status("[fail]", false),
            colors::message("SSH LOGIN FAILED", false),
        );

        assert_eq!(module.label(), Some("[fail]"));
        assert_eq!(module.width(10), 30);
        assert_eq!(render_to_string(&module, 10), expected);
    }

    #[test]
    fn zero_gap_keeps_status_and_message_separate() {
        let module = Access::new(true);
        let expected = format!(
            "    {}{}",
            colors::status("[ok]", true),
            colors::message("SSH LOGIN SUCCESSFUL", true),
        );

        assert_eq!(module.width(0), 28);
        assert_eq!(render_to_string(&module, 0), expected);
    }
}
