use std::io::{self, Write};

use crate::constants::Layout;
use crate::modules::Module;

use super::borderer::Borderer;

use unicode_width::UnicodeWidthStr;

pub struct RenderBuffer {
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

pub struct Renderer {
    borderer: Borderer,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            borderer: Borderer::new(),
        }
    }

    pub fn render<M: Module>(&self, lines: Vec<M>) -> io::Result<()> {
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

    fn width<M: Module>(&self, lines: &Vec<M>, gap: usize) -> usize {
        lines
            .iter()
            .map(|line| line.width(gap))
            .max()
            .unwrap_or_default()
    }

    fn gap<M: Module>(&self, lines: &Vec<M>) -> usize {
        let max_gap = lines
            .iter()
            .map(|line| line.label().unwrap_or_default().width())
            .max()
            .unwrap_or_default();

        max_gap + Layout::STATUS_GAP
    }
}
