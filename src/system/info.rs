use nix::unistd::{self, User};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub(crate) struct Info;

impl Info {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn username(&self) -> String {
        User::from_uid(unistd::getuid())
            .ok()
            .flatten()
            .map(|user| user.name)
            .unwrap_or_else(|| "unknown".to_string())
    }

    pub(crate) fn hostname(&self) -> String {
        unistd::gethostname()
            .map(|hostname| hostname.to_string_lossy().into_owned())
            .unwrap_or_else(|_| "unknown".to_string())
    }

    pub(crate) fn tty_name(&self) -> String {
        unistd::ttyname(io::stdin())
            .or_else(|_| unistd::ttyname(io::stdout()))
            .or_else(|_| unistd::ttyname(io::stderr()))
            .ok()
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_else(|| "unknown".to_string())
    }

    pub(crate) fn session_count(&self) -> String {
        let count = unsafe {
            libc::setutxent();

            let count = std::iter::from_fn(|| {
                let entry = libc::getutxent();

                (!entry.is_null()).then_some(entry)
            })
            .filter(|entry| (**entry).ut_type == libc::USER_PROCESS)
            .count();

            libc::endutxent();

            count
        };

        match count {
            0 => "unknown".into(),
            count => count.to_string(),
        }
    }

    pub(crate) fn os_name(&self) -> String {
        let Ok(file) = File::open("/etc/os-release") else {
            return "unknown".to_string();
        };

        let mut reader = BufReader::new(file);
        let mut line = String::new();

        loop {
            line.clear();

            let Ok(bytes) = reader.read_line(&mut line) else {
                return "unknown".to_string();
            };

            if bytes == 0 {
                return "unknown".to_string();
            }

            if let Some(value) = line.strip_prefix("PRETTY_NAME=") {
                return value.trim().trim_matches('"').to_string();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::{Command, Stdio};

    fn run_command(command: &str, args: &[&str], allowed_statuses: &[i32]) -> (String, i32) {
        let output = Command::new(command)
            .args(args)
            .stdin(Stdio::inherit())
            .env("LC_ALL", "C")
            .output()
            .unwrap_or_else(|error| panic!("failed to run {command}: {error}"));

        let status = output
            .status
            .code()
            .unwrap_or_else(|| panic!("{command} was terminated by a signal: {}", output.status));

        assert!(
            allowed_statuses.contains(&status),
            "{command} failed with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );

        let stdout = String::from_utf8(output.stdout)
            .unwrap_or_else(|error| panic!("{command} output was not valid UTF-8: {error}"));

        (stdout, status)
    }

    #[test]
    fn username_matches_id_un() {
        let (stdout, _) = run_command("id", &["-un"], &[0]);
        let expected = stdout.trim_end_matches(['\n', '\r']).to_string();

        assert_eq!(
            Info::new().username(),
            expected,
            "username() should match `id -un`"
        );
    }

    #[test]
    fn hostname_matches_uname_n() {
        let (stdout, _) = run_command("uname", &["-n"], &[0]);
        let expected = stdout.trim_end_matches(['\n', '\r']).to_string();

        assert_eq!(
            Info::new().hostname(),
            expected,
            "hostname() should match `uname -n`"
        );
    }

    #[test]
    fn tty_name_matches_tty_command() {
        use std::io::IsTerminal;

        let (stdout, status) = run_command("tty", &[], &[0, 1]);
        let expected = stdout.trim_end_matches(['\n', '\r']);
        match status {
            0 => assert_eq!(
                Info::new().tty_name(),
                expected,
                "tty_name() should match the `tty` path"
            ),
            1 if io::stdout().is_terminal() || io::stderr().is_terminal() => {
                assert_ne!(
                    Info::new().tty_name(),
                    "",
                    "tty_name() should fall back to stdout or stderr"
                );
            }
            1 => assert_eq!(
                Info::new().tty_name(),
                "unknown",
                "tty_name() should be unknown without a terminal"
            ),
            _ => unreachable!("run_command only allows exit codes 0 and 1"),
        }
    }

    #[test]
    fn session_count_matches_who_quick_count() {
        let (stdout, _) = run_command("who", &["-q"], &[0]);
        let expected_count = stdout
            .lines()
            .find_map(|line| line.strip_prefix("# users="))
            .unwrap_or_else(|| panic!("`who -q` output has no user count: {stdout:?}"))
            .parse::<usize>()
            .unwrap_or_else(|error| panic!("`who -q` user count is not a number: {error}"));

        let expected = match expected_count {
            0 => "unknown".to_string(),
            count => count.to_string(),
        };

        assert_eq!(
            Info::new().session_count(),
            expected,
            "session_count() should match the count from `who -q`"
        );
    }

    #[test]
    fn os_name_matches_pretty_name_from_os_release() {
        let expected = std::fs::read_to_string("/etc/os-release")
            .ok()
            .and_then(|contents| {
                contents.lines().find_map(|line| {
                    let (key, value) = line.split_once('=')?;
                    (key == "PRETTY_NAME").then(|| value.trim().trim_matches('"').to_string())
                })
            })
            .unwrap_or_else(|| panic!("failed to read PRETTY_NAME from /etc/os-release"));

        assert_eq!(
            Info::new().os_name(),
            expected,
            "os_name() should match PRETTY_NAME from /etc/os-release"
        );
    }
}
