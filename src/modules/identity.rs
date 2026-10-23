use std::io;

use crate::colors;
use crate::constants::Layout;
use crate::utils;

use super::Module;

use unicode_width::UnicodeWidthStr;

pub(crate) struct Identity {
    label: &'static str,
    username_ok: bool,
    hostname_ok: bool,
    username: String,
    hostname: String,
}

impl Identity {
    pub(crate) fn new(username: String, hostname: String) -> Self {
        let username_ok = utils::unknown(&username).is_none();
        let hostname_ok = utils::unknown(&hostname).is_none();

        Self {
            label: if username_ok && hostname_ok {
                "[ok]"
            } else {
                "[fail]"
            },
            username_ok,
            hostname_ok,
            username,
            hostname,
        }
    }
}

impl Module for Identity {
    fn render<W: io::Write>(&self, writer: &mut W, gap: usize) -> io::Result<()> {
        let padding = Layout::PADDING;
        let status = colors::status(self.label, self.username_ok && self.hostname_ok);
        let session = colors::session("SESSION", self.username_ok && self.hostname_ok);
        let username = colors::username(&self.username, self.username_ok);
        let hostname = colors::hostname(&self.hostname, self.hostname_ok);

        write!(
            writer,
            "{:padding$}{:<gap$}{session} {username}@{hostname}",
            "", status
        )
    }

    fn width(&self, gap: usize) -> usize {
        Layout::PADDING
            + gap.max(self.label.len())
            + 7 // "SESSION"
            + 1 // " "
            + self.username.width()
            + 1 // "@"
            + self.hostname.width()
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
    fn renders_known_identity() {
        let module = Identity::new("user".to_string(), "host".to_string());
        let expected = format!(
            "    {:<10}{} {}@{}",
            colors::status("[ok]", true),
            colors::session("SESSION", true),
            colors::username("user", true),
            colors::hostname("host", true),
        );

        assert_eq!(module.label(), Some("[ok]"));
        assert_eq!(module.width(10), 31);
        assert_eq!(render_to_string(&module, 10), expected);
    }

    #[test]
    fn marks_unknown_username_as_failed() {
        let module = Identity::new("unknown".to_string(), "host".to_string());
        let expected = format!(
            "    {:<10}{} {}@{}",
            colors::status("[fail]", false),
            colors::session("SESSION", false),
            colors::username("unknown", false),
            colors::hostname("host", true),
        );

        assert_eq!(module.label(), Some("[fail]"));
        assert_eq!(module.width(10), 34);
        assert_eq!(render_to_string(&module, 10), expected);
    }

    #[test]
    fn marks_unknown_hostname_as_failed() {
        let module = Identity::new("user".to_string(), "unknown".to_string());
        let expected = format!(
            "    {:<10}{} {}@{}",
            colors::status("[fail]", false),
            colors::session("SESSION", false),
            colors::username("user", true),
            colors::hostname("unknown", false),
        );

        assert_eq!(module.label(), Some("[fail]"));
        assert_eq!(module.width(10), 34);
        assert_eq!(render_to_string(&module, 10), expected);
    }

    #[test]
    fn marks_unknown_identity_as_failed() {
        let module = Identity::new("unknown".to_string(), "unknown".to_string());
        let expected = format!(
            "    {:<10}{} {}@{}",
            colors::status("[fail]", false),
            colors::session("SESSION", false),
            colors::username("unknown", false),
            colors::hostname("unknown", false),
        );

        assert_eq!(module.label(), Some("[fail]"));
        assert_eq!(module.width(10), 37);
        assert_eq!(render_to_string(&module, 10), expected);
    }

    #[test]
    fn zero_gap_keeps_status_and_session_separate() {
        let module = Identity::new("user".to_string(), "host".to_string());
        let expected = format!(
            "    {}{} {}@{}",
            colors::status("[ok]", true),
            colors::session("SESSION", true),
            colors::username("user", true),
            colors::hostname("host", true),
        );

        assert_eq!(module.width(0), 25);
        assert_eq!(render_to_string(&module, 0), expected);
    }

    #[test]
    fn empty_parts_are_valid_and_have_zero_width() {
        let module = Identity::new(String::new(), String::new());
        let expected = format!(
            "    {:<10}{} {}@{}",
            colors::status("[ok]", true),
            colors::session("SESSION", true),
            colors::username("", true),
            colors::hostname("", true),
        );

        assert_eq!(module.label(), Some("[ok]"));
        assert_eq!(module.width(10), 23);
        assert_eq!(render_to_string(&module, 10), expected);
    }

    #[test]
    fn wide_unicode_parts_use_display_width() {
        let module = Identity::new("用户".to_string(), "主机".to_string());
        let expected = format!(
            "    {:<10}{} {}@{}",
            colors::status("[ok]", true),
            colors::session("SESSION", true),
            colors::username("用户", true),
            colors::hostname("主机", true),
        );

        assert_eq!(module.width(10), 31);
        assert_eq!(render_to_string(&module, 10), expected);
    }
}
