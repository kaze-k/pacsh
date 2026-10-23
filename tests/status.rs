use std::process::Command;

#[test]
fn binary_exit_status_matches_run_result() {
    let output = Command::new(env!("CARGO_BIN_EXE_pacsh"))
        .output()
        .expect("failed to execute the pacsh binary built by Cargo");

    assert!(output.status.success());
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
}

#[test]
fn lib_run_returns_success() {
    let result = pacsh::run();

    assert!(result.is_ok(), "pacsh::run() failed: {result:?}");
}
