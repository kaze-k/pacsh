use nix::unistd::{self, User};
use std::{
    fs,
    io::{stderr, stdin, stdout},
};

pub struct Info;

impl Info {
    pub fn new() -> Self {
        Self
    }

    pub fn username(&self) -> String {
        User::from_uid(unistd::getuid())
            .ok()
            .flatten()
            .map(|user| user.name)
            .unwrap_or_else(|| "unknown".to_string())
    }

    pub fn hostname(&self) -> String {
        unistd::gethostname()
            .map(|hostname| hostname.to_string_lossy().into_owned())
            .unwrap_or_else(|_| "unknown".to_string())
    }

    pub fn tty_name(&self) -> String {
        unistd::ttyname(stdin())
            .or_else(|_| unistd::ttyname(stdout()))
            .or_else(|_| unistd::ttyname(stderr()))
            .ok()
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_else(|| "unknown".to_string())
    }

    pub fn session_count(&self) -> String {
        let count = unsafe {
            libc::setutxent();

            let count = std::iter::from_fn(|| {
                let entry = libc::getutxent();

                (!entry.is_null()).then_some(entry)
            })
            .filter(|entry| (**entry).ut_type == libc::USER_PROCESS as i16)
            .count();

            libc::endutxent();

            count
        };

        match count {
            0 => "unknown".into(),
            count => count.to_string(),
        }
    }

    pub fn os_name(&self) -> String {
        fs::read_to_string("/etc/os-release")
            .map(|content| {
                content
                    .lines()
                    .find_map(|line| line.strip_prefix("PRETTY_NAME="))
                    .map(|v| v.trim_matches('"').to_string())
                    .unwrap_or_else(|| "unknown".to_string())
            })
            .unwrap_or_else(|_| "unknown".to_string())
    }
}
