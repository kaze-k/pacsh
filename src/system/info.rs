use nix::{
    sys::utsname,
    unistd::{self, User},
};
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
            return self.get_sysname("unknown");
        };

        let mut reader = BufReader::new(file);
        let mut line = String::new();

        loop {
            line.clear();

            let Ok(bytes) = reader.read_line(&mut line) else {
                return self.get_sysname("unknown");
            };

            if bytes == 0 {
                return self.get_sysname("unknown");
            }

            if let Some(value) = line.strip_prefix("PRETTY_NAME=") {
                return value.trim().trim_matches('"').to_string();
            }
        }
    }

    fn get_sysname(&self, fallback: &str) -> String {
        utsname::uname()
            .map(|info| info.sysname().to_string_lossy().into_owned())
            .unwrap_or_else(|_| fallback.to_string())
    }
}
