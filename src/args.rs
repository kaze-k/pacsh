use std::ffi::OsStr;
use std::io::{self, Write};
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};

const PACMAN_ARG: &str = "--pacman";
const HELP_ARG: &str = "--help";
const SHORT_HELP_ARG: &str = "-h";

const HELP_TEXT: &[&str] = &[
    "Usage: pacsh [OPTIONS]",
    "",
    "Options:",
    "      --pacman    Use pacman icon (requires Nerd Font)",
    "  -h, --help      Print help information",
];

static PACMAN: AtomicBool = AtomicBool::new(false);
static HELP: AtomicBool = AtomicBool::new(false);

fn write_help<W: Write>(mut writer: W) {
    for line in HELP_TEXT {
        writeln!(writer, "{line}").ok();
    }
}

fn exit_with_help() -> ! {
    let stdout = io::stdout();
    let writer = stdout.lock();
    write_help(writer);
    process::exit(0);
}

fn exit_with_error(arg: &OsStr) -> ! {
    let stderr = io::stderr();
    let mut writer = stderr.lock();

    writeln!(
        writer,
        "Error: unknown argument '{}'",
        arg.to_string_lossy()
    )
    .ok();

    write_help(&mut writer);

    drop(writer);
    process::exit(2);
}

pub(crate) struct Args;

impl Args {
    pub(crate) fn parse() {
        PACMAN.store(false, Ordering::Relaxed);
        HELP.store(false, Ordering::Relaxed);

        for arg in std::env::args_os().skip(1) {
            match arg.as_os_str() {
                a if a == PACMAN_ARG => PACMAN.store(true, Ordering::Relaxed),
                a if a == HELP_ARG || a == SHORT_HELP_ARG => {
                    HELP.store(true, Ordering::Relaxed);
                }
                _ => exit_with_error(&arg),
            }
        }

        if HELP.load(Ordering::Relaxed) {
            exit_with_help();
        }
    }

    pub(crate) fn has_pacman() -> bool {
        PACMAN.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_help_outputs_every_usage_line() {
        let mut output = Vec::new();

        write_help(&mut output);

        let expected = HELP_TEXT
            .iter()
            .map(|line| format!("{line}\n"))
            .collect::<String>();
        assert_eq!(String::from_utf8(output).unwrap(), expected);
    }

    #[test]
    fn has_pacman_defaults_to_false() {
        assert!(!Args::has_pacman());
    }
}
