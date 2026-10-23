use std::env;

pub(crate) struct Ssh {
    pub(crate) is_ssh: bool,
    pub(crate) client_ip: String,
    pub(crate) client_port: String,
    pub(crate) server_ip: String,
    pub(crate) server_port: String,
}

impl Ssh {
    pub(crate) fn new() -> Self {
        Self {
            is_ssh: false,
            client_ip: "unknown".to_string(),
            client_port: "unknown".to_string(),
            server_ip: "unknown".to_string(),
            server_port: "unknown".to_string(),
        }
    }

    pub(crate) fn parse(mut self) -> Self {
        let Some(connection) = env::var_os("SSH_CONNECTION") else {
            return self;
        };

        let connection = connection.to_string_lossy().into_owned();
        let mut parts = connection.split_whitespace();

        self.is_ssh = true;
        self.client_ip = parts
            .next()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        self.client_port = parts
            .next()
            .map(|port| port.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        self.server_ip = parts
            .next()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        self.server_port = parts
            .next()
            .map(|port| port.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        self
    }
}

#[cfg(test)]
mod tests {
    use super::Ssh;
    use std::env;

    #[test]
    fn new_starts_disconnected_with_unknown_details() {
        let ssh = Ssh::new();

        assert!(!ssh.is_ssh);
        assert_eq!(ssh.client_ip, "unknown");
        assert_eq!(ssh.client_port, "unknown");
        assert_eq!(ssh.server_ip, "unknown");
        assert_eq!(ssh.server_port, "unknown");
    }

    #[test]
    fn parse_covers_ssh_connection_cases() {
        fn set_connection(value: Option<&str>) {
            unsafe {
                match value {
                    Some(value) => env::set_var("SSH_CONNECTION", value),
                    None => env::remove_var("SSH_CONNECTION"),
                }
            }
        }

        let cases = [
            (None, false, ("unknown", "unknown", "unknown", "unknown")),
            (Some(""), true, ("unknown", "unknown", "unknown", "unknown")),
            (
                Some(" \t\n "),
                true,
                ("unknown", "unknown", "unknown", "unknown"),
            ),
            (
                Some("192.0.2.10"),
                true,
                ("192.0.2.10", "unknown", "unknown", "unknown"),
            ),
            (
                Some("192.0.2.10 54321"),
                true,
                ("192.0.2.10", "54321", "unknown", "unknown"),
            ),
            (
                Some("192.0.2.10 54321 192.0.2.20"),
                true,
                ("192.0.2.10", "54321", "192.0.2.20", "unknown"),
            ),
            (
                Some("192.0.2.10 54321 192.0.2.20 22"),
                true,
                ("192.0.2.10", "54321", "192.0.2.20", "22"),
            ),
            (
                Some("192.0.2.10 54321 192.0.2.20 22 ignored"),
                true,
                ("192.0.2.10", "54321", "192.0.2.20", "22"),
            ),
            (
                Some("  192.0.2.10\t54321   192.0.2.20 22  "),
                true,
                ("192.0.2.10", "54321", "192.0.2.20", "22"),
            ),
            (
                Some("192.0.2.10\n54321\n192.0.2.20\n22"),
                true,
                ("192.0.2.10", "54321", "192.0.2.20", "22"),
            ),
            (
                Some("2001:db8::1 54321 2001:db8::2 22"),
                true,
                ("2001:db8::1", "54321", "2001:db8::2", "22"),
            ),
        ];

        for (connection, is_ssh, expected) in cases {
            set_connection(connection);
            let ssh = Ssh::new().parse();

            assert_eq!(ssh.is_ssh, is_ssh, "SSH_CONNECTION={connection:?}");
            assert_eq!(
                (
                    ssh.client_ip.as_str(),
                    ssh.client_port.as_str(),
                    ssh.server_ip.as_str(),
                    ssh.server_port.as_str(),
                ),
                expected,
                "SSH_CONNECTION={connection:?}"
            );
        }

        set_connection(None);
    }
}
