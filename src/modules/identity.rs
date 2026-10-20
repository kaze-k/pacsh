use std::io;

use crate::colors;
use crate::constants::Layout;
use crate::utils;

use super::Module;

use unicode_width::UnicodeWidthStr;

pub struct Identity {
    label: &'static str,
    username_ok: bool,
    hostname_ok: bool,
    username: String,
    hostname: String,
}

impl Identity {
    pub fn new(username: String, hostname: String) -> Self {
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
