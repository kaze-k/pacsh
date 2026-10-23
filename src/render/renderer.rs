use std::io::{self, Write};

use crate::constants::Layout;
use crate::modules::Module;

use super::borderer::Borderer;

use unicode_width::UnicodeWidthStr;

struct RenderBuffer {
    buf: Vec<u8>,
}

impl RenderBuffer {
    fn new() -> Self {
        Self { buf: Vec::new() }
    }

    fn spaces(&mut self, count: usize) -> &mut Self {
        self.buf.resize(self.buf.len() + count, b' ');

        self
    }

    fn newline(&mut self, count: usize) -> &mut Self {
        self.buf.resize(self.buf.len() + count, b'\n');

        self
    }

    fn write_with<F>(&mut self, func: F) -> io::Result<&mut Self>
    where
        F: FnOnce(&mut RenderBuffer) -> io::Result<()>,
    {
        func(self)?;
        Ok(self)
    }

    fn as_bytes(&self) -> &[u8] {
        &self.buf
    }
}

impl Write for RenderBuffer {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.buf.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub(crate) struct Renderer {
    borderer: Borderer,
}

impl Renderer {
    pub(crate) fn new() -> Self {
        Self {
            borderer: Borderer::new(),
        }
    }

    pub(crate) fn render<M: Module>(&self, lines: Vec<M>) -> io::Result<()> {
        self.render_to(lines, &mut io::stdout().lock())
    }

    fn render_to<M: Module, W: Write>(&self, lines: Vec<M>, output: &mut W) -> io::Result<()> {
        let gap = self.gap(&lines);
        let width = self.width(&lines, gap);

        let mut buffer = RenderBuffer::new();

        buffer
            .newline(2)
            .spaces(Layout::INDENT)
            .write_with(|buf| self.borderer.top(buf, width))?
            .newline(1);

        for (index, line) in lines.iter().enumerate() {
            let line_width = line.width(gap);
            let padding = width.saturating_sub(line_width) + Layout::PADDING;

            buffer
                .spaces(Layout::INDENT)
                .write_with(|buf| self.borderer.left(buf, index, lines.len()))?
                .write_with(|buf| line.render(buf, gap))?
                .spaces(padding)
                .write_with(|buf| self.borderer.right(buf, index, lines.len()))?
                .newline(1);
        }

        buffer
            .spaces(Layout::INDENT)
            .write_with(|buf| self.borderer.bottom(buf, width))?
            .newline(2);

        output.write_all(buffer.as_bytes())?;
        output.flush()
    }

    fn width<M: Module>(&self, lines: &[M], gap: usize) -> usize {
        lines
            .iter()
            .map(|line| line.width(gap))
            .max()
            .expect("failed to get maximum lines width: should not be empty")
    }

    fn gap<M: Module>(&self, lines: &[M]) -> usize {
        let max_gap = lines
            .iter()
            .map(|line| line.label().unwrap_or_default().width())
            .max()
            .expect("failed to get maximum label width: should not be empty");

        max_gap + Layout::STATUS_GAP
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write};

    use crate::modules::{Access, Blank, From, Identity, ModuleItem, Os, Sessions, To, Tty};

    #[test]
    fn render_buffer_spaces_appends_spaces() {
        let mut buffer = RenderBuffer::new();

        buffer.spaces(3).spaces(0);

        assert_eq!(buffer.as_bytes(), b"   ");
    }

    #[test]
    fn render_buffer_newline_appends_newlines() {
        let mut buffer = RenderBuffer::new();

        buffer.newline(1).newline(2);

        assert_eq!(buffer.as_bytes(), b"\n\n\n");
    }

    #[test]
    fn render_buffer_zero_length_operations_do_nothing() {
        let mut buffer = RenderBuffer::new();

        buffer.spaces(0).newline(0);

        assert!(buffer.as_bytes().is_empty());
        assert_eq!(buffer.write(b"").unwrap(), 0);
        assert!(buffer.as_bytes().is_empty());
    }

    #[test]
    fn render_buffer_write_appends_bytes() {
        let mut buffer = RenderBuffer::new();

        assert_eq!(buffer.write(b"abc").unwrap(), 3);
        assert_eq!(buffer.as_bytes(), b"abc");
        assert!(buffer.flush().is_ok());
    }

    #[test]
    fn render_buffer_write_with_runs_closure_and_propagates_errors() {
        let mut buffer = RenderBuffer::new();

        buffer
            .write_with(|buffer| {
                buffer.write_all(b"ok")?;
                Ok(())
            })
            .unwrap();
        assert_eq!(buffer.as_bytes(), b"ok");

        let error = buffer
            .write_with(|_| Err(io::Error::other("boom")))
            .err()
            .expect("write_with should fail");
        assert_eq!(error.to_string(), "boom");
    }

    #[test]
    fn gap_uses_widest_label_plus_status_gap() {
        let renderer = Renderer::new();
        let lines = [
            ModuleItem::Access(Access::new(true)),
            ModuleItem::Sessions(Sessions::new("2".to_string())),
        ];

        assert_eq!(renderer.gap(&lines), 10);
    }

    #[test]
    fn width_uses_widest_rendered_line() {
        let renderer = Renderer::new();
        let lines = [
            ModuleItem::Access(Access::new(true)),
            ModuleItem::Sessions(Sessions::new("2".to_string())),
        ];

        assert_eq!(renderer.width(&lines, 10), 34);
    }

    #[test]
    fn blank_lines_use_status_gap_and_zero_width() {
        let renderer = Renderer::new();
        let lines = [ModuleItem::Blank(Blank::new())];

        assert_eq!(renderer.gap(&lines), 2);
        assert_eq!(renderer.width(&lines, 2), 0);
    }

    #[test]
    #[should_panic(expected = "should not be empty")]
    fn gap_panics_on_empty_input() {
        Renderer::new().gap::<ModuleItem>(&[]);
    }

    #[test]
    #[should_panic(expected = "should not be empty")]
    fn width_panics_on_empty_input() {
        Renderer::new().width::<ModuleItem>(&[], 0);
    }

    #[test]
    fn render_to_writes_expected_banner_and_styles() {
        const EXPECTED_RENDER: &str = concat!(
            "\n\n",
            "    ╭── 󰮯 ─────────────────────────────────╮\n",
            "    │                                      │\n",
            "    │    [ok]      SSH LOGIN SUCCESSFUL    │\n",
            "    │    [ok]      SESSION user@host       │\n",
            "    │                                      │\n",
            "    │    OS        Arch Linux              │\n",
            "    │    TTY       /dev/pts/0              │\n",
            "    │    FROM      192.168.1.10:6119       │\n",
            "    │    TO        10.0.0.5:22             │\n",
            "    │    SESSIONS  2                       │\n",
            "    │                                      │\n",
            "    ╰───────────────────────────────── 󰊠 ──╯\n",
            "\n",
        );

        fn without_styles(text: &str) -> String {
            let mut output = String::with_capacity(text.len());
            let mut chars = text.chars().peekable();

            while let Some(ch) = chars.next() {
                if ch == '\x1b' && chars.peek() == Some(&'[') {
                    chars.next();

                    for ch in chars.by_ref() {
                        if ch.is_ascii_alphabetic() {
                            break;
                        }
                    }
                } else {
                    output.push(ch);
                }
            }

            output
        }

        let renderer = Renderer::new();
        let mut output = Vec::new();

        renderer
            .render_to(
                vec![
                    ModuleItem::Blank(Blank::new()),
                    ModuleItem::Access(Access::new(true)),
                    ModuleItem::Identity(Identity::new("user".to_string(), "host".to_string())),
                    ModuleItem::Blank(Blank::new()),
                    ModuleItem::Os(Os::new("Arch Linux".to_string())),
                    ModuleItem::Tty(Tty::new("/dev/pts/0".to_string())),
                    ModuleItem::From(From::new("192.168.1.10".to_string(), "6119".to_string())),
                    ModuleItem::To(To::new("10.0.0.5".to_string(), "22".to_string())),
                    ModuleItem::Sessions(Sessions::new("2".to_string())),
                    ModuleItem::Blank(Blank::new()),
                ],
                &mut output,
            )
            .unwrap();

        let text = String::from_utf8(output).unwrap();

        assert_eq!(without_styles(&text), EXPECTED_RENDER);

        let status_style = "\x1b[92;1m[ok]      \x1b[0m";
        let expected_styles = [
            ("top-left border", "\x1b[38;2;96;165;250m╭\x1b[39m", 6, 0),
            (
                "top-right border",
                "\x1b[38;2;255;105;180m╮\x1b[39m",
                1055,
                0,
            ),
            (
                "bottom-left border",
                "\x1b[38;2;255;165;0m╰\x1b[39m",
                2195,
                0,
            ),
            (
                "bottom-right border",
                "\x1b[38;2;45;212;191m╯\x1b[39m",
                3211,
                0,
            ),
            ("access status", status_style, 1214, 0),
            (
                "success message",
                "\x1b[97;1mSSH LOGIN SUCCESSFUL\x1b[0m",
                1235,
                0,
            ),
            ("identity status", status_style, 1333, 1),
            ("session label", "\x1b[37;1mSESSION\x1b[0m", 1354, 0),
            ("username", "\x1b[38;2;255;165;0;1muser\x1b[0m", 1373, 0),
            ("hostname", "\x1b[38;2;255;105;180;1mhost\x1b[0m", 1401, 0),
            ("os label", "\x1b[96;1mOS        \x1b[0m", 1597, 0),
            ("tty label", "\x1b[96;1mTTY       \x1b[0m", 1705, 0),
            ("from label", "\x1b[96;1mFROM      \x1b[0m", 1812, 0),
            ("to label", "\x1b[96;1mTO        \x1b[0m", 1919, 0),
            ("sessions label", "\x1b[96;1mSESSIONS  \x1b[0m", 2025, 0),
        ];

        for (name, style, expected_index, occurrence) in expected_styles {
            let actual_index = text
                .match_indices(style)
                .nth(occurrence)
                .map(|(index, _)| index)
                .unwrap_or_else(|| panic!("missing styled {name}"));

            assert_eq!(actual_index, expected_index, "wrong styled {name} index");
        }
    }
}
