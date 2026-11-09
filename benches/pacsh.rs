use std::env;
use std::io;
use std::process::{Command, ExitCode, Stdio};

const CLEAN_ARGS_ENV: &str = "PACSH_BENCH_CLEAN_ARGS";

fn main() -> ExitCode {
    if env::var_os(CLEAN_ARGS_ENV).is_some() {
        divan::Divan::default().main();
        return ExitCode::SUCCESS;
    }

    if env::args_os().any(|arg| arg == "--bench") {
        let executable = env::current_exe().expect("failed to get benchmark executable");

        let status = Command::new(executable)
            .env(CLEAN_ARGS_ENV, "1")
            .status()
            .expect("failed to re-execute benchmark");

        return if status.success() {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        };
    }

    divan::main();
    ExitCode::SUCCESS
}

#[divan::bench(name = "1_run")]
fn run() {
    pacsh::run().unwrap()
}

#[divan::bench(name = "2_run_to_sink")]
fn run_to_sink() {
    let sink = io::sink();
    pacsh::run_to(sink).unwrap()
}

#[divan::bench(name = "3_execute")]
fn execute() {
    let status = Command::new(env!("CARGO_BIN_EXE_pacsh"))
        .stdout(Stdio::null())
        .status()
        .expect("failed to run pacsh");

    assert!(status.success());
}

#[divan::bench(name = "4_pacman")]
fn pacman() {
    let status = Command::new(env!("CARGO_BIN_EXE_pacsh"))
        .arg("--pacman")
        .stdout(Stdio::null())
        .status()
        .expect("failed to run pacsh --pacman");

    assert!(status.success());
}

#[divan::bench(name = "5_help")]
fn help() {
    let status = Command::new(env!("CARGO_BIN_EXE_pacsh"))
        .arg("--help")
        .stdout(Stdio::null())
        .status()
        .expect("failed to run pacsh --help");

    assert!(status.success());
}
