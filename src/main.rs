use std::{io, process::ExitCode};

fn main() -> ExitCode {
    match pacsh::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("pacsh: {error}");
            ExitCode::FAILURE
        }
    }
}
