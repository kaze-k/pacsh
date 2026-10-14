mod colors;
mod constants;
mod modules;
mod render;
mod system;
mod types;
mod utils;

use std::{io, process::ExitCode};

use modules::{Access, Blank, From, Identity, ModuleItem, Os, Sessions, To, Tty};
use render::Renderer;
use system::{Info, Ssh};

fn run() -> io::Result<()> {
    let info = Info::new();
    let ssh = Ssh::new().parse();

    let username = info.username();
    let hostname = info.hostname();
    let tty_name = info.tty_name();
    let session_count = info.session_count();
    let os_name = info.os_name();

    let blank = Blank::new();
    let access = Access::new(ssh.is_ssh);
    let identity = Identity::new(username, hostname);
    let tty = Tty::new(tty_name);
    let session = Sessions::new(session_count);
    let os = Os::new(os_name);

    let renderer = Renderer::new();

    if ssh.is_ssh {
        let from = From::new(ssh.client_ip, ssh.client_port);
        let to = To::new(ssh.server_ip, ssh.server_port);

        let lines = vec![
            ModuleItem::Blank(blank),
            ModuleItem::Access(access),
            ModuleItem::Identity(identity),
            ModuleItem::Blank(blank),
            ModuleItem::Os(os),
            ModuleItem::Tty(tty),
            ModuleItem::From(from),
            ModuleItem::To(to),
            ModuleItem::Sessions(session),
            ModuleItem::Blank(blank),
        ];

        renderer.render(lines)
    } else {
        let lines = vec![
            ModuleItem::Blank(blank),
            ModuleItem::Access(access),
            ModuleItem::Identity(identity),
            ModuleItem::Blank(blank),
            ModuleItem::Os(os),
            ModuleItem::Tty(tty),
            ModuleItem::Sessions(session),
            ModuleItem::Blank(blank),
        ];

        renderer.render(lines)
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("pacsh: {error}");
            ExitCode::FAILURE
        }
    }
}
