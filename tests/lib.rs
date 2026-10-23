//! `pacsh` 公共库 API 的集成测试。
//!
//! `pacsh::run()` 直接写入进程 stdout/stderr。为了避免与 libtest 自身输出竞争，
//! 所有场景都在同一个测试中执行；该测试临时重定向 fd 1/2，并在返回前恢复。

pub mod common;

use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::{self, Read};
use std::os::fd::{AsRawFd, RawFd};
use std::thread;

use common::layout::assert_exact_frame;
use common::output::{field_value, strip_ansi};
use common::process::pipe;

const SSH_CONNECTION: &str = "192.0.2.10 6119 198.51.100.20 22";

struct EnvGuard {
    key: &'static str,
    original: Option<OsString>,
}

impl EnvGuard {
    fn set(key: &'static str, value: Option<&str>) -> Self {
        let original = env::var_os(key);

        unsafe {
            match value {
                Some(value) => env::set_var(key, value),
                None => env::remove_var(key),
            }
        }

        Self { key, original }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        unsafe {
            match &self.original {
                Some(value) => env::set_var(self.key, value),
                None => env::remove_var(self.key),
            }
        }
    }
}

struct FdRedirect {
    fd: RawFd,
    original: RawFd,
}

impl FdRedirect {
    fn new(fd: RawFd, target: RawFd) -> Self {
        let original = unsafe { libc::dup(fd) };

        assert!(
            original >= 0,
            "failed to duplicate fd {fd}: {}",
            io::Error::last_os_error()
        );

        let result = unsafe { libc::dup2(target, fd) };

        if result < 0 {
            unsafe {
                libc::close(original);
            }

            panic!("failed to redirect fd {fd}: {}", io::Error::last_os_error());
        }

        Self { fd, original }
    }
}

impl Drop for FdRedirect {
    fn drop(&mut self) {
        unsafe {
            libc::dup2(self.original, self.fd);
            libc::close(self.original);
        }
    }
}

fn capture_run(ssh_connection: Option<&str>) -> (io::Result<()>, String, String) {
    let _env = EnvGuard::set("SSH_CONNECTION", ssh_connection);
    let (stdout_read, stdout_write) = pipe();
    let (stderr_read, stderr_write) = pipe();
    let stdout_reader = thread::spawn(move || {
        let mut file = File::from(stdout_read);
        let mut bytes = Vec::new();

        file.read_to_end(&mut bytes)
            .expect("failed to read captured stdout");

        bytes
    });
    let stderr_reader = thread::spawn(move || {
        let mut file = File::from(stderr_read);
        let mut bytes = Vec::new();

        file.read_to_end(&mut bytes)
            .expect("failed to read captured stderr");

        bytes
    });

    let stdout_redirect = FdRedirect::new(libc::STDOUT_FILENO, stdout_write.as_raw_fd());
    let stderr_redirect = FdRedirect::new(libc::STDERR_FILENO, stderr_write.as_raw_fd());
    let result = pacsh::run();

    drop(stdout_redirect);
    drop(stderr_redirect);
    drop(stdout_write);
    drop(stderr_write);

    let stdout = stdout_reader.join().expect("stdout reader thread panicked");
    let stderr = stderr_reader.join().expect("stderr reader thread panicked");

    (
        result,
        String::from_utf8(stdout).expect("pacsh stdout should be valid UTF-8"),
        String::from_utf8(stderr).expect("pacsh stderr should be valid UTF-8"),
    )
}

fn run_with_broken_pipe(ssh_connection: Option<&str>) -> (io::Result<()>, String) {
    let _env = EnvGuard::set("SSH_CONNECTION", ssh_connection);
    let (read_fd, write_fd) = pipe();
    let (stderr_read, stderr_write) = pipe();
    let stderr_reader = thread::spawn(move || {
        let mut file = File::from(stderr_read);
        let mut bytes = Vec::new();

        file.read_to_end(&mut bytes)
            .expect("failed to read captured stderr");

        bytes
    });

    // 在运行前关闭读端，使对 fd 1 的每次写入都返回 EPIPE。
    drop(read_fd);

    let stdout_redirect = FdRedirect::new(libc::STDOUT_FILENO, write_fd.as_raw_fd());
    let stderr_redirect = FdRedirect::new(libc::STDERR_FILENO, stderr_write.as_raw_fd());
    let result = pacsh::run();

    drop(stdout_redirect);
    drop(stderr_redirect);
    drop(write_fd);
    drop(stderr_write);

    let stderr = stderr_reader.join().expect("stderr reader thread panicked");

    (
        result,
        String::from_utf8(stderr).expect("pacsh stderr should be valid UTF-8"),
    )
}

fn assert_identity_row(row: &str) {
    let label = if row.starts_with("[ok]") {
        "[ok]"
    } else if row.starts_with("[fail]") {
        "[fail]"
    } else {
        panic!("identity row should start with a status label: {row:?}");
    };

    let value = field_value(row, label);

    assert!(
        value.starts_with("SESSION "),
        "identity value should be a SESSION value: {value:?}"
    );
    assert!(
        value.len() > "SESSION ".len(),
        "identity value should include username and hostname: {value:?}"
    );
}

#[test]
fn run_writes_the_banner_and_propagates_broken_pipe() {
    // 普通的不带 SSH 信息的库调用应写出完整的失败横幅。
    let (result, raw, stderr) = capture_run(None);
    result.expect("run() should succeed when stdout is writable");
    assert!(
        stderr.is_empty(),
        "run() should leave stderr to the binary caller: {stderr:?}"
    );

    let visible = strip_ansi(&raw);
    let layout = assert_exact_frame(&visible);

    assert_eq!(
        layout.contents.len(),
        8,
        "the non-SSH banner should contain eight body rows"
    );
    assert_eq!(layout.contents[0], "");
    assert_eq!(
        field_value(&layout.contents[1], "[fail]"),
        "SSH LOGIN FAILED"
    );
    assert_identity_row(&layout.contents[2]);
    assert_eq!(layout.contents[3], "");
    assert!(!field_value(&layout.contents[4], "OS").is_empty());
    assert!(!field_value(&layout.contents[5], "TTY").is_empty());
    assert!(!field_value(&layout.contents[6], "SESSIONS").is_empty());
    assert_eq!(layout.contents[7], "");
    assert!(
        !visible.contains("SSH LOGIN SUCCESSFUL"),
        "the non-SSH banner must not claim a successful SSH login"
    );

    // 提供 SSH 信息时，run() 按文档顺序添加 FROM 和 TO。
    let (result, raw, stderr) = capture_run(Some(SSH_CONNECTION));
    result.expect("run() should succeed when stdout is writable");
    assert!(
        stderr.is_empty(),
        "run() should leave stderr to the binary caller: {stderr:?}"
    );

    let visible = strip_ansi(&raw);
    let layout = assert_exact_frame(&visible);

    assert_eq!(
        layout.contents.len(),
        10,
        "the SSH banner should contain ten body rows"
    );
    assert_eq!(
        field_value(&layout.contents[1], "[ok]"),
        "SSH LOGIN SUCCESSFUL"
    );
    assert_identity_row(&layout.contents[2]);
    assert_eq!(layout.contents[3], "");
    assert!(!field_value(&layout.contents[4], "OS").is_empty());
    assert!(!field_value(&layout.contents[5], "TTY").is_empty());
    assert_eq!(field_value(&layout.contents[6], "FROM"), "192.0.2.10:6119");
    assert_eq!(field_value(&layout.contents[7], "TO"), "198.51.100.20:22");
    assert!(!field_value(&layout.contents[8], "SESSIONS").is_empty());
    assert_eq!(layout.contents[9], "");

    // run() 必须传播写入错误；main 把 BrokenPipe 映射为成功属于
    // 独立的二进制行为，在 binary.rs 中测试。
    let (error, stderr) = run_with_broken_pipe(None);
    let error = error.expect_err("run() should fail on a broken stdout");

    assert_eq!(
        error.kind(),
        io::ErrorKind::BrokenPipe,
        "run() should preserve the underlying write error: {error}"
    );
    assert!(
        stderr.is_empty(),
        "run() should propagate the error without printing it: {stderr:?}"
    );
}
