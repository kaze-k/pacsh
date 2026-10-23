use super::{Access, Blank, From, Identity, Os, Sessions, To, Tty};
use std::io;

pub(crate) trait Module {
    fn render<W: io::Write>(&self, writer: &mut W, gap: usize) -> io::Result<()>;
    fn width(&self, gap: usize) -> usize;
    fn label(&self) -> Option<&str>;
}

pub(crate) enum ModuleItem {
    Access(Access),
    Blank(Blank),
    From(From),
    Identity(Identity),
    Os(Os),
    To(To),
    Tty(Tty),
    Sessions(Sessions),
}

impl Module for ModuleItem {
    fn render<W: io::Write>(&self, writer: &mut W, gap: usize) -> io::Result<()> {
        match self {
            Self::Access(module) => module.render(writer, gap),
            Self::Blank(module) => module.render(writer, gap),
            Self::From(module) => module.render(writer, gap),
            Self::Identity(module) => module.render(writer, gap),
            Self::Os(module) => module.render(writer, gap),
            Self::To(module) => module.render(writer, gap),
            Self::Tty(module) => module.render(writer, gap),
            Self::Sessions(module) => module.render(writer, gap),
        }
    }

    fn width(&self, gap: usize) -> usize {
        match self {
            Self::Access(module) => module.width(gap),
            Self::Blank(module) => module.width(gap),
            Self::From(module) => module.width(gap),
            Self::Identity(module) => module.width(gap),
            Self::Os(module) => module.width(gap),
            Self::To(module) => module.width(gap),
            Self::Tty(module) => module.width(gap),
            Self::Sessions(module) => module.width(gap),
        }
    }

    fn label(&self) -> Option<&str> {
        match self {
            Self::Access(module) => module.label(),
            Self::Blank(module) => module.label(),
            Self::From(module) => module.label(),
            Self::Identity(module) => module.label(),
            Self::Os(module) => module.label(),
            Self::To(module) => module.label(),
            Self::Tty(module) => module.label(),
            Self::Sessions(module) => module.label(),
        }
    }
}
