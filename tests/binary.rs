//! 针对编译后的 `pacsh` 可执行文件的集成测试。

use std::process::{Command, ExitStatus};

trait ExitStatusExt {
    fn fail(&self) -> bool;
}

impl ExitStatusExt for ExitStatus {
    fn fail(&self) -> bool {
        !self.success()
    }
}

#[test]
fn binary_exit_status_is_success() {
    let output = Command::new(env!("CARGO_BIN_EXE_pacsh"))
        .output()
        .expect("failed to execute the pacsh binary built by Cargo");

    assert!(output.status.success());
    assert!(!output.status.fail());
}
