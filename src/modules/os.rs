use std::io;

use crate::colors;
use crate::constants::Layout;
use crate::utils;

use super::Module;

use unicode_width::UnicodeWidthStr;

pub(crate) struct Os {
    label: &'static str,
    ok: bool,
    name: String,
}

impl Os {
    pub(crate) fn new(name: String) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::render_to_string;

    #[test]
    fn renders_os_name() {
        let module = Os::new("Arch Linux".to_string());
        let expected = format!(
            "    {:<10}{}",
            colors::label("OS", true),
            colors::info("Arch Linux", true),
        );

        assert_eq!(module.label(), Some("OS"));
        assert_eq!(module.width(10), 24);
        assert_eq!(render_to_string(&module, 10), expected);
    }

    #[test]
    fn marks_unknown_os_as_failed() {
        let module = Os::new("unknown".to_string());
        let expected = format!(
            "    {:<10}{}",
            colors::label("OS", false),
            colors::info("unknown", false),
        );

        assert_eq!(module.width(10), 21);
        assert_eq!(render_to_string(&module, 10), expected);
    }

    #[test]
    fn empty_name_and_zero_gap_keep_label_only() {
        let module = Os::new(String::new());
        let expected = format!("    {}", colors::label("OS", true));

        assert_eq!(module.width(0), 6);
        assert_eq!(render_to_string(&module, 0), expected);
    }

    #[test]
    fn wide_unicode_name_uses_display_width() {
        let module = Os::new("测试".to_string());
        let expected = format!(
            "    {:<10}{}",
            colors::label("OS", true),
            colors::info("测试", true),
        );

        assert_eq!(module.width(10), 18);
        assert_eq!(render_to_string(&module, 10), expected);
    }
}
