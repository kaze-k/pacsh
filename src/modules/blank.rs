use std::io;

use super::Module;

#[derive(Clone, Copy)]
pub(crate) struct Blank;

impl Blank {
    pub(crate) fn new() -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::render_to_string;

    #[test]
    fn blank_renders_nothing() {
        let module = Blank::new();

        assert_eq!(module.label(), None);
        assert_eq!(module.width(10), 0);
        assert_eq!(render_to_string(&module, 10), "");
    }
}
