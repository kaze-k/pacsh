use std::io;
use std::os::fd::{FromRawFd, OwnedFd};
use std::process::{Command, Output, Stdio};

fn run_binary(ssh_connection: Option<&str>) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_pacsh"));
    command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    match ssh_connection {
        Some(value) => command.env("SSH_CONNECTION", value),
        None => command.env_remove("SSH_CONNECTION"),
    };

    command
        .output()
        .expect("failed to execute the pacsh binary built by Cargo")
}

/// 运行 Cargo 为本包构建的二进制文件，并要求满足正常成功流程的进程契约。
///
/// 这里刻意把 stdin 设为 `/dev/null`，并把 stdout/stderr 设为管道，
/// 因此无论测试自身运行在什么环境中，`Info::tty_name` 都只能走非 TTY 回退分支。
pub fn run_binary_ok(ssh_connection: Option<&str>) -> String {
    let output = run_binary(ssh_connection);

    assert!(
        output.status.success(),
        "pacsh should exit successfully, got {:?}; stderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stderr.is_empty(),
        "pacsh should not write to stderr on success: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("pacsh stdout should be valid UTF-8")
}

/// 创建一个管道，不修改进程级信号或描述符状态。
pub fn pipe() -> (OwnedFd, OwnedFd) {
    let mut fds = [-1; 2];
    let result = unsafe { libc::pipe(fds.as_mut_ptr()) };

    assert_eq!(
        result,
        0,
        "failed to create a pipe: {}",
        io::Error::last_os_error()
    );

    unsafe { (OwnedFd::from_raw_fd(fds[0]), OwnedFd::from_raw_fd(fds[1])) }
}
