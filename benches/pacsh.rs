use std::env;
use std::io;
use std::process::{Command, ExitCode};

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

#[divan::bench]
fn run() {
    pacsh::run().unwrap()
}

#[divan::bench]
fn run_to_sink() {
    let sink = io::sink();
    pacsh::run_to(sink).unwrap()
}
