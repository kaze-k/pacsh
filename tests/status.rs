use std::process::{Command, Output, Stdio};

fn binary() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_pacsh"));
    command
        .env_remove("SSH_CONNECTION")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    command
}

fn assert_success(output: &Output) {
    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty(), "stderr: {:?}", output.stderr);
}

fn without_styles(text: &str) -> String {
    let mut plain = String::new();
    let mut chars = text.chars();
    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            assert_eq!(chars.next(), Some('['), "unexpected escape sequence");
            assert!(chars.by_ref().any(|ch| ('@'..='~').contains(&ch)));
        } else {
            plain.push(ch);
        }
    }
    plain
}

fn banner_rows(output: &Output) -> Vec<String> {
    assert_success(output);
    let text = without_styles(std::str::from_utf8(&output.stdout).expect("UTF-8 banner"));
    let banner = text
        .strip_prefix("\n\n")
        .and_then(|text| text.strip_suffix("\n\n"))
        .expect("banner should have two leading and trailing newlines");
    let mut lines: Vec<_> = banner.lines().collect();
    assert!(lines.len() >= 2, "missing banner borders: {text:?}");
    let top = lines.remove(0);
    let bottom = lines.pop().unwrap();
    assert!(top.starts_with("    ╭── 󰮯 ") && top.ends_with('╮'));
    assert!(bottom.starts_with("    ╰") && bottom.ends_with(" 󰊠 ──╯"));

    lines
        .into_iter()
        .map(|line| {
            let content = line
                .strip_prefix("    │")
                .and_then(|line| line.strip_suffix('│'))
                .expect("every banner row should have side borders");
            content.split_whitespace().collect::<Vec<_>>().join(" ")
        })
        .collect()
}

fn assert_common_rows(rows: &[String]) {
    assert_eq!(rows[0], "");
    let identity = rows[2]
        .strip_prefix("[ok] SESSION ")
        .or_else(|| rows[2].strip_prefix("[fail] SESSION "))
        .expect("identity row should include a status and SESSION");
    let (user, host) = identity.split_once('@').expect("user@host identity");
    assert!(!user.is_empty() && !host.is_empty());
    assert_eq!(rows[3], "");
    assert!(rows[4].strip_prefix("OS ").is_some_and(|os| !os.is_empty()));
    assert_eq!(rows[5], "TTY unknown");
    let sessions = rows[rows.len() - 2]
        .strip_prefix("SESSIONS ")
        .expect("sessions should be the last content row");
    assert!(sessions == "unknown" || sessions.parse::<usize>().is_ok());
    assert_eq!(rows.last().unwrap(), "");
}

#[test]
fn binary_renders_ssh_banner_with_client_and_server_in_order() {
    let output = binary()
        .env("SSH_CONNECTION", "192.0.2.10 54321 192.0.2.20 22")
        .output()
        .expect("failed to execute the pacsh binary built by Cargo");

    let rows = banner_rows(&output);
    assert_eq!(rows.len(), 10, "unexpected banner rows: {rows:?}");
    assert_common_rows(&rows);
    assert_eq!(rows[1], "[ok] SSH LOGIN SUCCESSFUL");
    assert_eq!(rows[6], "FROM 192.0.2.10:54321");
    assert_eq!(rows[7], "TO 192.0.2.20:22");
}

#[test]
fn binary_renders_non_ssh_banner_without_address_rows() {
    let output = binary().output().expect("failed to execute pacsh");

    let rows = banner_rows(&output);
    assert_eq!(rows.len(), 8, "unexpected banner rows: {rows:?}");
    assert_common_rows(&rows);
    assert_eq!(rows[1], "[fail] SSH LOGIN FAILED");
    assert!(
        rows.iter()
            .all(|row| !row.starts_with("FROM ") && !row.starts_with("TO "))
    );
}

#[test]
fn lib_run_returns_success() {
    let result = pacsh::run();

    assert!(result.is_ok(), "pacsh::run() failed: {result:?}");
}
