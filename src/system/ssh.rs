use std::env;

pub struct Ssh {
    pub is_ssh: bool,
    pub client_ip: String,
    pub client_port: String,
    pub server_ip: String,
    pub server_port: String,
}

impl Ssh {
    pub fn new() -> Self {
        Self {
            is_ssh: false,
            client_ip: "unknown".to_string(),
            client_port: "unknown".to_string(),
            server_ip: "unknown".to_string(),
            server_port: "unknown".to_string(),
        }
    }

    pub fn parse(mut self) -> Self {
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
