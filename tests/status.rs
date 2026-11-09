use std::process::{Command, Output, Stdio};

const HELP_TEXT: &str = "\
Usage: pacsh [OPTIONS]

Options:
      --pacman    Use pacman icon (requires Nerd Font)
  -h, --help      Print help information
";

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
    banner_rows_with_icons(output, '●', '◉')
}

fn banner_rows_with_icons(output: &Output, top_icon: char, bottom_icon: char) -> Vec<String> {
    assert_success(output);
    let text = without_styles(std::str::from_utf8(&output.stdout).expect("UTF-8 banner"));
    let banner = text
        .strip_prefix("\n\n")
        .and_then(|text| text.strip_suffix("\n\n"))
        .expect("banner should have two leading and trailing newlines");
    let mut lines: Vec<&str> = banner.lines().collect();
    assert!(lines.len() >= 2, "missing banner borders: {text:?}");
    let top = lines.remove(0);
    let bottom = lines.pop().unwrap();
    assert!(
        top.starts_with(&format!("    ╭── {top_icon} ")) && top.ends_with('╮'),
        "unexpected top border: {top:?}"
    );
    assert!(
        bottom.starts_with("    ╰") && bottom.ends_with(&format!(" {bottom_icon} ──╯")),
        "unexpected bottom border: {bottom:?}"
    );

    lines
        .into_iter()
        .map(|line| {
            let content = line
                .strip_prefix("    │")
                .and_then(|line| line.strip_suffix('│'))
                .expect("every banner row should have side borders");
            content.split_whitespace().collect::<Vec<&str>>().join(" ")
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
fn binary_pacman_flag_uses_pacman_icon() {
    let output = binary()
        .arg("--pacman")
        .output()
        .expect("failed to execute pacsh with --pacman");

    let rows = banner_rows_with_icons(&output, '󰮯', '󰊠');
    assert_eq!(rows.len(), 8, "unexpected banner rows: {rows:?}");
    assert_common_rows(&rows);
}

#[test]
fn binary_help_flags_print_help_without_rendering_a_banner() {
    for flag in ["--help", "-h"] {
        let output = binary()
            .arg(flag)
            .output()
            .unwrap_or_else(|_| panic!("failed to execute pacsh with {flag}"));

        assert_eq!(output.status.code(), Some(0), "flag: {flag}");
        assert_eq!(
            String::from_utf8(output.stdout).expect("help output should be UTF-8"),
            HELP_TEXT,
            "flag: {flag}"
        );
        assert!(
            output.stderr.is_empty(),
            "flag: {flag}, stderr: {:?}",
            output.stderr
        );
    }
}

#[test]
fn binary_help_flag_takes_precedence_over_pacman() {
    let output = binary()
        .args(["--pacman", "--help"])
        .output()
        .expect("failed to execute pacsh with --pacman --help");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(
        String::from_utf8(output.stdout).expect("help output should be UTF-8"),
        HELP_TEXT
    );
    assert!(output.stderr.is_empty(), "stderr: {:?}", output.stderr);
}

#[test]
fn binary_unknown_arguments_exit_with_usage_error() {
    for arg in ["--unknown", "-x"] {
        let output = binary()
            .arg(arg)
            .output()
            .unwrap_or_else(|_| panic!("failed to execute pacsh with {arg}"));

        assert_eq!(output.status.code(), Some(2), "arg: {arg}");
        assert!(
            output.stdout.is_empty(),
            "arg: {arg}, stdout: {:?}",
            output.stdout
        );
        assert_eq!(
            String::from_utf8(output.stderr).expect("error output should be UTF-8"),
            format!("Error: unknown argument '{arg}'\n{HELP_TEXT}"),
            "arg: {arg}"
        );
    }
}
