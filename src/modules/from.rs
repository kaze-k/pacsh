use std::io::{self, Write};

use crate::colors;
use crate::constants::Layout;
use crate::utils::unknown;

use super::Module;

use unicode_width::UnicodeWidthStr;

pub struct From {
    label: &'static str,
    ip_ok: bool,
    port_ok: bool,
    ip: String,
    port: String,
}

impl From {
    pub fn new(ip: String, port: String) -> Self {
        Self {
            label: "FROM",
            ip_ok: unknown(&ip).is_none(),
            port_ok: unknown(&port).is_none(),
            ip: ip,
            port: port,
        }
    }
}

impl Module for From {
    fn render<W: Write>(&self, writer: &mut W, gap: usize) -> io::Result<()> {
        let padding = Layout::PADDING;
        let label = colors::label(self.label, self.ip_ok && self.port_ok);
        let ip = colors::info(&self.ip, self.ip_ok);
        let port = colors::info(&self.port, self.port_ok);

        write!(writer, "{:padding$}{:<gap$}{ip}:{port}", "", label)
    }

    fn width(&self, gap: usize) -> usize {
        Layout::PADDING
            + gap.max(self.label.width())
            + self.ip.width()
            + 1 // ":"
            + self.port.width()
    }

    fn label(&self) -> Option<&str> {
        Some(self.label)
    }
}
