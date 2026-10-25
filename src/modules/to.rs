use std::io;

use crate::colors;
use crate::constants::Layout;
use crate::utils;

use super::Module;

use unicode_width::UnicodeWidthStr;

pub(crate) struct To {
    label: &'static str,
    ip_ok: bool,
    port_ok: bool,
    ip: String,
    port: String,
}

impl To {
    pub(crate) fn new(ip: String, port: String) -> Self {
        Self {
            label: "TO",
            ip_ok: utils::unknown(&ip).is_none(),
            port_ok: utils::unknown(&port).is_none(),
            ip,
            port,
        }
    }
}

impl Module for To {
    fn render<W: io::Write>(&self, writer: &mut W, gap: usize) -> io::Result<()> {
        let padding = Layout::PADDING;
        let label = colors::label(&self.label, self.ip_ok && self.port_ok);
        let ip = colors::info(&self.ip, self.ip_ok);
        let port = colors::info(&self.port, self.port_ok);

        write!(writer, "{:padding$}{:<gap$}{ip}:{port}", "", label)
    }

    fn width(&self, gap: usize) -> usize {
        Layout::PADDING
            + gap.max(self.label.len())
            + self.ip.width()
            + 1 // ":"
            + self.port.width()
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
    fn renders_server_address() {
        let module = To::new("10.0.0.5".to_string(), "22".to_string());
        let expected = format!(
            "    {:<10}{}:{}",
            colors::label("TO", true),
            colors::info("10.0.0.5", true),
            colors::info("22", true),
        );

        assert_eq!(module.label(), Some("TO"));
        assert_eq!(module.width(10), 25);
        assert_eq!(render_to_string(&module, 10), expected);
    }

    #[test]
    fn marks_unknown_ip_as_failed() {
        let module = To::new("unknown".to_string(), "22".to_string());
        let expected = format!(
            "    {:<10}{}:{}",
            colors::label("TO", false),
            colors::info("unknown", false),
            colors::info("22", true),
        );

        assert_eq!(module.width(10), 24);
        assert_eq!(render_to_string(&module, 10), expected);
    }

    #[test]
    fn marks_unknown_port_as_failed() {
        let module = To::new("10.0.0.5".to_string(), "unknown".to_string());
        let expected = format!(
            "    {:<10}{}:{}",
            colors::label("TO", false),
            colors::info("10.0.0.5", true),
            colors::info("unknown", false),
        );

        assert_eq!(module.width(10), 30);
        assert_eq!(render_to_string(&module, 10), expected);
    }

    #[test]
    fn marks_both_unknown_parts_as_failed() {
        let module = To::new("unknown".to_string(), "unknown".to_string());
        let expected = format!(
            "    {:<10}{}:{}",
            colors::label("TO", false),
            colors::info("unknown", false),
            colors::info("unknown", false),
        );

        assert_eq!(module.width(10), 29);
        assert_eq!(render_to_string(&module, 10), expected);
    }

    #[test]
    fn zero_gap_keeps_label_and_address_separate() {
        let module = To::new("10.0.0.5".to_string(), "22".to_string());
        let expected = format!(
            "    {}{}:{}",
            colors::label("TO", true),
            colors::info("10.0.0.5", true),
            colors::info("22", true),
        );

        assert_eq!(module.width(0), 17);
        assert_eq!(render_to_string(&module, 0), expected);
    }
}
