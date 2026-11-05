mod colors;
mod constants;
mod modules;
mod render;
mod system;
mod types;
mod utils;

#[cfg(test)]
mod tests;

use std::io::{self, Write};

use modules::{Access, Blank, From, Identity, ModuleItem, Os, Sessions, To, Tty};
use render::Renderer;
use system::{Info, Ssh};

fn get_lines() -> Vec<ModuleItem> {
    let info = Info::new();
    let ssh = Ssh::new().parse();

    let mut lines = vec![
        ModuleItem::Blank(Blank::new()),
        ModuleItem::Access(Access::new(ssh.is_ssh)),
        ModuleItem::Identity(Identity::new(info.username(), info.hostname())),
        ModuleItem::Blank(Blank::new()),
        ModuleItem::Os(Os::new(info.os_name())),
        ModuleItem::Tty(Tty::new(info.tty_name())),
    ];

    if ssh.is_ssh {
        lines.extend([
            ModuleItem::From(From::new(ssh.client_ip, ssh.client_port)),
            ModuleItem::To(To::new(ssh.server_ip, ssh.server_port)),
        ]);
    }

    lines.extend([
        ModuleItem::Sessions(Sessions::new(info.session_count())),
        ModuleItem::Blank(Blank::new()),
    ]);

    lines
}

pub fn run() -> io::Result<()> {
    let lines = get_lines();
    Renderer::new().render(&lines)
}

pub fn run_to<W: Write>(output: W) -> io::Result<()> {
    let lines = get_lines();
    Renderer::new().render_to(&lines, output)
}
