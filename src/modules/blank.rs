use std::io;

use super::Module;

#[derive(Clone, Copy)]
pub struct Blank;

impl Blank {
    pub fn new() -> Self {
        Self
    }
}

impl Module for Blank {
    fn render<W: io::Write>(&self, _writer: &mut W, _gap: usize) -> io::Result<()> {
        Ok(())
    }

    fn width(&self, _gap: usize) -> usize {
        0
    }

    fn label(&self) -> Option<&str> {
        None
    }
}
