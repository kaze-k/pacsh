use std::ffi::OsStr;
use std::io::{self, Write};
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};

use owo_colors::OwoColorize;

static PACMAN: AtomicBool = AtomicBool::new(false);
static HELP: AtomicBool = AtomicBool::new(false);

pub(crate) struct Args;

impl Args {
    const PACMAN_ARG: &str = "--pacman";
    const HELP_ARG: &str = "--help";
    const SHORT_HELP_ARG: &str = "-h";

    const HELP_TEXT: &[&str] = &[
        "",
        "Usage: pacsh [OPTIONS]",
        "",
        "Options:",
        "      --pacman    Use pacman icon (requires Nerd Font)",
        "  -h, --help      Print help information",
    ];
}

impl Args {
    pub(crate) fn parse() {
        PACMAN.store(false, Ordering::Relaxed);
        HELP.store(false, Ordering::Relaxed);

        for arg in std::env::args_os().skip(1) {
            match arg.as_os_str() {
                a if a == Self::PACMAN_ARG => PACMAN.store(true, Ordering::Relaxed),
                a if a == Self::HELP_ARG || a == Self::SHORT_HELP_ARG => {
                    HELP.store(true, Ordering::Relaxed);
                }
                _ => Self::exit_with_error(&arg),
            }
        }

        if HELP.load(Ordering::Relaxed) {
            Self::exit_with_help();
        }
    }

    pub(crate) fn has_pacman() -> bool {
        PACMAN.load(Ordering::Relaxed)
    }

    fn write_help<W: Write>(mut writer: W) {
        for line in Self::HELP_TEXT {
            writeln!(writer, "{line}").ok();
        }
    }

    fn exit_with_help() -> ! {
        let stdout = io::stdout();
        let writer = stdout.lock();
        Self::write_help(writer);
        process::exit(0);
    }

    fn exit_with_error(arg: &OsStr) -> ! {
        let stderr = io::stderr();
        let mut writer = stderr.lock();

        writeln!(
            writer,
            "{}: unknown argument '{}'",
            "Error".red(),
            arg.to_string_lossy()
        )
        .ok();

        Self::write_help(&mut writer);

        drop(writer);
        process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_pacman_defaults_to_false() {
        assert!(!Args::has_pacman());
    }

    #[test]
    fn write_help_outputs_every_usage_line() {
        let mut output = Vec::new();

        Args::write_help(&mut output);

        let expected = Args::HELP_TEXT
            .iter()
            .map(|line| format!("{line}\n"))
            .collect::<String>();

        assert_eq!(String::from_utf8(output).unwrap(), expected);
    }

    #[test]
    fn help_text_documents_supported_flags() {
        let help = Args::HELP_TEXT.join("\n");

        assert!(help.contains(Args::PACMAN_ARG));
        assert!(help.contains(Args::HELP_ARG));
        assert!(help.contains(Args::SHORT_HELP_ARG));
    }

    #[test]
    fn help_text_has_exact_content() {
        let expected: &[&str] = &[
            "",
            "Usage: pacsh [OPTIONS]",
            "",
            "Options:",
            "      --pacman    Use pacman icon (requires Nerd Font)",
            "  -h, --help      Print help information",
        ];

        assert_eq!(Args::HELP_TEXT, expected);
    }

    #[test]
    fn argument_constants_match_public_cli() {
        assert_eq!(Args::PACMAN_ARG, "--pacman");
        assert_eq!(Args::HELP_ARG, "--help");
        assert_eq!(Args::SHORT_HELP_ARG, "-h");
    }
}
