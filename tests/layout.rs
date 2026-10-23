//! 针对横幅终端布局和 ANSI 装饰的契约测试。

pub mod common;

use common::layout::{BannerLayout, assert_exact_frame, display_width};
use common::output::{field_value, strip_ansi};
use common::process::run_binary_ok;

const SSH_CONNECTION: &str = "192.0.2.10 6119 198.51.100.20 22";

fn exact_layout(ssh_connection: Option<&str>) -> BannerLayout {
    let raw = run_binary_ok(ssh_connection);
    let visible = strip_ansi(&raw);

    assert_exact_frame(&visible)
}

fn identity_label(row: &str) -> &'static str {
    if row.starts_with("[ok]") {
        "[ok]"
    } else if row.starts_with("[fail]") {
        "[fail]"
    } else {
        panic!("identity row should start with a status label: {row:?}");
    }
}

fn assert_identity_layout(row: &str) {
    let value = field_value(row, identity_label(row));

    assert!(
        value.starts_with("SESSION "),
        "identity row should contain a SESSION value after the status column: {row:?}"
    );
}

#[test]
fn failed_login_has_exact_frame_and_row_order() {
    let layout = exact_layout(None);
    let contents = &layout.contents;

    assert_eq!(
        contents.len(),
        8,
        "the non-SSH banner should have exactly eight body rows: {contents:#?}"
    );

    assert_eq!(contents[0], "", "the first row should be blank");
    assert_eq!(field_value(&contents[1], "[fail]"), "SSH LOGIN FAILED");
    assert_identity_layout(&contents[2]);
    assert_eq!(
        contents[3], "",
        "identity and system fields should be separated"
    );
    assert!(!field_value(&contents[4], "OS").is_empty());
    assert!(!field_value(&contents[5], "TTY").is_empty());
    assert!(!field_value(&contents[6], "SESSIONS").is_empty());
    assert_eq!(contents[7], "", "the last row should be blank");
    assert!(
        contents
            .iter()
            .all(|row| !row.starts_with("FROM") && !row.starts_with("TO")),
        "FROM and TO should not occupy rows without SSH data: {contents:#?}"
    );
}

#[test]
fn successful_ssh_login_has_exact_frame_and_row_order() {
    let layout = exact_layout(Some(SSH_CONNECTION));
    let contents = &layout.contents;

    assert_eq!(
        contents.len(),
        10,
        "the SSH banner should have exactly ten body rows: {contents:#?}"
    );

    assert_eq!(contents[0], "", "the first row should be blank");
    assert_eq!(field_value(&contents[1], "[ok]"), "SSH LOGIN SUCCESSFUL");
    assert_identity_layout(&contents[2]);
    assert_eq!(
        contents[3], "",
        "identity and system fields should be separated"
    );
    assert!(!field_value(&contents[4], "OS").is_empty());
    assert!(!field_value(&contents[5], "TTY").is_empty());
    assert!(!field_value(&contents[6], "FROM").is_empty());
    assert!(!field_value(&contents[7], "TO").is_empty());
    assert!(!field_value(&contents[8], "SESSIONS").is_empty());
    assert_eq!(contents[9], "", "the last row should be blank");
}

#[test]
fn long_ssh_values_keep_the_frame_and_alignment_consistent() {
    let short = exact_layout(Some("192.0.2.10 6119 198.51.100.20 22"));
    let long = exact_layout(Some(
        "2001:0db8:85a3:0000:0000:8a2e:0370:7334 65535 \
         2001:0db8:85a3:0000:0000:8a2e:0370:7335 65535",
    ));

    assert_eq!(
        short.contents.len(),
        long.contents.len(),
        "longer FROM/TO values must not add or remove rows"
    );
    assert_eq!(long.contents.len(), 10);
    assert_eq!(
        field_value(&long.contents[6], "FROM"),
        "2001:0db8:85a3:0000:0000:8a2e:0370:7334:65535"
    );
    assert_eq!(
        field_value(&long.contents[7], "TO"),
        "2001:0db8:85a3:0000:0000:8a2e:0370:7335:65535"
    );

    let short_from_width = display_width(&short.contents[6]);
    let long_from_width = display_width(&long.contents[6]);

    assert!(
        long_from_width > short_from_width,
        "the controlled long value should make the FROM field wider"
    );

    // `assert_exact_frame` 会独立检查每个正文行、上边框和下边框是否都采用
    // 最长正文行推导出的宽度。行数保持不变也证明长值没有折行成额外的终端行。
}

#[test]
fn ansi_styles_are_present_without_changing_visible_layout() {
    let raw = run_binary_ok(Some(SSH_CONNECTION));
    let visible = strip_ansi(&raw);

    assert!(
        !visible.contains('\x1b'),
        "stripping ANSI should produce plain visible text"
    );
    assert!(
        raw.contains('\x1b'),
        "the real binary should emit ANSI color sequences"
    );
    assert_exact_frame(&visible);

    // 状态、消息、标签和渐变端点属于用户可见的样式契约；
    // 上面的可见布局验证了它们不占用终端宽度。
    for sequence in [
        "\x1b[92;1m[ok]      \x1b[0m",
        "\x1b[97;1mSSH LOGIN SUCCESSFUL\x1b[0m",
        "\x1b[96;1mFROM      \x1b[0m",
        "\x1b[38;2;96;165;250m╭\x1b[39m",
        "\x1b[38;2;255;105;180m╮\x1b[39m",
        "\x1b[38;2;255;165;0m╰\x1b[39m",
        "\x1b[38;2;45;212;191m╯\x1b[39m",
    ] {
        assert!(
            raw.contains(sequence),
            "raw output should contain the expected ANSI sequence {sequence:?}: {raw:?}"
        );
    }

    let failed_raw = run_binary_ok(None);

    for sequence in [
        "\x1b[91;1m[fail]    \x1b[0m",
        "\x1b[31;1mSSH LOGIN FAILED\x1b[0m",
    ] {
        assert!(
            failed_raw.contains(sequence),
            "failed-login output should contain the expected ANSI sequence {sequence:?}: \
             {failed_raw:?}"
        );
    }
}
