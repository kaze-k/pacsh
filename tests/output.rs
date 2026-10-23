//! 针对用户可见横幅字段及其数据来源的契约测试。

pub mod common;

use std::fs::File;
use std::io::{BufRead, BufReader};

use nix::unistd::{self, User};

use common::output::{field_value, parse_banner};
use common::process::run_binary_ok;

fn expected_identity() -> (String, String) {
    let username = User::from_uid(unistd::getuid())
        .ok()
        .flatten()
        .map(|user| user.name)
        .unwrap_or_else(|| "unknown".to_string());
    let hostname = unistd::gethostname()
        .map(|hostname| hostname.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "unknown".to_string());

    (username, hostname)
}

fn identity_status(username: &str, hostname: &str) -> &'static str {
    if username == "unknown" || hostname == "unknown" {
        "[fail]"
    } else {
        "[ok]"
    }
}

fn row_status(row: &str) -> &'static str {
    if row.starts_with("[ok]") {
        "[ok]"
    } else if row.starts_with("[fail]") {
        "[fail]"
    } else {
        panic!("row should start with a status label: {row:?}");
    }
}

fn expected_os() -> String {
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

#[test]
fn failed_login_reports_system_fields_without_from_or_to() {
    let contents = parse_banner(&run_binary_ok(None));

    assert_eq!(
        contents.len(),
        8,
        "the non-SSH banner should have exactly eight content rows: {contents:#?}"
    );

    assert_eq!(contents[0], "");
    assert_eq!(
        field_value(&contents[1], "[fail]"),
        "SSH LOGIN FAILED",
        "a missing SSH_CONNECTION should produce the failed-login status pair"
    );

    let (username, hostname) = expected_identity();
    let label = identity_status(&username, &hostname);

    assert_eq!(
        field_value(&contents[2], label),
        format!("SESSION {username}@{hostname}"),
        "SESSION should be built from the current username and hostname"
    );

    assert_eq!(contents[3], "");
    assert_eq!(
        field_value(&contents[4], "OS"),
        expected_os(),
        "OS should be the PRETTY_NAME from /etc/os-release"
    );
    assert_eq!(
        field_value(&contents[5], "TTY"),
        "unknown",
        "a binary with null stdin and piped output should not expose a TTY"
    );
    let sessions = field_value(&contents[6], "SESSIONS");

    // 实时会话数不是可控输入。这里只验证约定形态
    //（0 表示为 "unknown"，否则为正整数），而不是与登录/登出竞争，
    // 也不硬编码当前机器的会话数。
    assert!(
        sessions == "unknown" || sessions.parse::<usize>().is_ok_and(|count| count > 0),
        "SESSIONS should be unknown or a positive decimal count, got {sessions:?}"
    );
    assert_eq!(contents[7], "");

    assert!(
        contents
            .iter()
            .all(|row| !row.starts_with("FROM") && !row.starts_with("TO")),
        "FROM and TO must not be rendered without SSH connection data: {contents:#?}"
    );
}

const SSH_CONNECTION: &str = "192.0.2.10 6119 198.51.100.20 22";

#[test]
fn successful_ssh_login_reports_controlled_endpoints() {
    let contents = parse_banner(&run_binary_ok(Some(SSH_CONNECTION)));

    assert_eq!(
        contents.len(),
        10,
        "the SSH banner should have exactly ten content rows: {contents:#?}"
    );

    assert_eq!(contents[0], "");
    assert_eq!(
        field_value(&contents[1], "[ok]"),
        "SSH LOGIN SUCCESSFUL",
        "SSH_CONNECTION should produce the success status pair"
    );
    let identity = &contents[2];
    assert!(
        field_value(identity, row_status(identity)).starts_with("SESSION "),
        "the SSH banner should still contain a SESSION value"
    );
    assert_eq!(contents[3], "");
    assert!(!field_value(&contents[4], "OS").is_empty());
    assert!(!field_value(&contents[5], "TTY").is_empty());
    assert_eq!(
        field_value(&contents[6], "FROM"),
        "192.0.2.10:6119",
        "FROM should map client IP and port from SSH_CONNECTION"
    );
    assert_eq!(
        field_value(&contents[7], "TO"),
        "198.51.100.20:22",
        "TO should map server IP and port from SSH_CONNECTION"
    );
    assert!(!field_value(&contents[8], "SESSIONS").is_empty());
    assert_eq!(contents[9], "");
}

#[test]
fn ssh_connection_edge_cases_preserve_field_mapping() {
    // 这些用例覆盖“非 SSH”和“带部分数据的 SSH”之间的边界。
    // 解析器按空白分割，不校验地址或端口格式。
    let cases = [
        ("", "unknown:unknown", "unknown:unknown"),
        (
            " 192.0.2.10\t54321\n198.51.100.20 22 ",
            "192.0.2.10:54321",
            "198.51.100.20:22",
        ),
        ("192.0.2.10", "192.0.2.10:unknown", "unknown:unknown"),
        (
            "2001:db8::1 54321 2001:db8::2 22",
            "2001:db8::1:54321",
            "2001:db8::2:22",
        ),
        (
            "192.0.2.10 54321 198.51.100.20 22 ignored",
            "192.0.2.10:54321",
            "198.51.100.20:22",
        ),
    ];

    for (connection, expected_from, expected_to) in cases {
        let contents = parse_banner(&run_binary_ok(Some(connection)));

        assert_eq!(
            contents.len(),
            10,
            "SSH_CONNECTION={connection:?} should still use the SSH layout: {contents:#?}"
        );
        assert_eq!(
            field_value(&contents[1], "[ok]"),
            "SSH LOGIN SUCCESSFUL",
            "SSH_CONNECTION={connection:?} should be treated as an SSH session"
        );
        assert_eq!(
            field_value(&contents[6], "FROM"),
            expected_from,
            "unexpected FROM value for SSH_CONNECTION={connection:?}"
        );
        assert_eq!(
            field_value(&contents[7], "TO"),
            expected_to,
            "unexpected TO value for SSH_CONNECTION={connection:?}"
        );
    }
}
