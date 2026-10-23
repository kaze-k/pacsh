use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

const INDENT: usize = 4;
const PADDING: usize = 4;
const PACMAN: char = '󰮯';
const GHOST: char = '󰊠';

pub fn display_width(text: &str) -> usize {
    text.width()
}

pub struct BannerLayout {
    pub contents: Vec<String>,
}

/// 断言精确的可见边框契约，并返回正文内容行。
///
/// 边框宽度由实际正文行推导，但所有结构属性（缩进、内边距、边框、图标、
/// 宽度一致性）都会独立检查。这样既能发现边框/内边距回归，又不会把
/// 当前机器的用户名、主机名、OS 或会话数固化进测试。
pub fn assert_exact_frame(visible: &str) -> BannerLayout {
    assert!(
        !visible.contains('\x1b'),
        "visible output still contains ANSI escapes"
    );
    assert!(
        visible.starts_with("\n\n"),
        "output should start with two newlines: {visible:?}"
    );
    assert!(
        visible.ends_with("\n\n"),
        "output should end with two newlines: {visible:?}"
    );

    let frame = visible
        .strip_prefix("\n\n")
        .and_then(|frame| frame.strip_suffix("\n\n"))
        .expect("output should contain a frame surrounded by blank lines");
    let lines: Vec<&str> = frame.split('\n').collect();

    assert!(
        lines.len() >= 3,
        "frame should have a top, body, and bottom"
    );

    let top = lines[0];
    let bottom = lines[lines.len() - 1];
    let body = &lines[1..lines.len() - 1];
    let indent = " ".repeat(INDENT);
    let leading_padding = " ".repeat(PADDING);
    let left_prefix = format!("{indent}│");

    let mut contents = Vec::with_capacity(body.len());

    for line in body {
        let rest = line.strip_prefix(&left_prefix).unwrap_or_else(|| {
            panic!("body row should start with indent and left border: {line:?}")
        });
        let inner = rest
            .strip_suffix('│')
            .unwrap_or_else(|| panic!("body row should end with right border: {line:?}"));
        let body = inner.strip_prefix(&leading_padding).unwrap_or_else(|| {
            panic!("body row should have {PADDING} spaces after the left border: {line:?}")
        });
        let content = body.trim_end_matches(' ');
        contents.push(content.to_string());
    }

    let max_content_width = contents
        .iter()
        .map(|content| display_width(content))
        .max()
        .expect("banner should have at least one body row");
    let expected_inner_width = max_content_width + PADDING * 2;
    let frame_width = INDENT + 1 + expected_inner_width + 1;

    for line in body {
        let inner = line
            .strip_prefix(&left_prefix)
            .and_then(|rest| rest.strip_suffix('│'))
            .expect("body row should be surrounded by borders");
        assert_eq!(
            display_width(inner),
            expected_inner_width,
            "all body rows should have the same inner width: {line:?}"
        );
        assert_eq!(
            display_width(line),
            frame_width,
            "body row width should match the frame width: {line:?}"
        );
    }

    let pacman_width = PACMAN.width().expect("PACMAN icon should have a width");
    let ghost_width = GHOST.width().expect("GHOST icon should have a width");
    let top_dashes = frame_width
        .checked_sub(INDENT + 6 + pacman_width)
        .expect("frame is too narrow for the top border");
    let bottom_dashes = frame_width
        .checked_sub(INDENT + 6 + ghost_width)
        .expect("frame is too narrow for the bottom border");
    let expected_top = format!("{indent}╭── {PACMAN} {}╮", "─".repeat(top_dashes));
    let expected_bottom = format!("{indent}╰{} {GHOST} ──╯", "─".repeat(bottom_dashes));

    assert_eq!(top, expected_top, "top border should match the frame width");
    assert_eq!(
        bottom, expected_bottom,
        "bottom border should match the frame width"
    );
    assert_eq!(
        display_width(top),
        frame_width,
        "top border width should match the frame width"
    );
    assert_eq!(
        display_width(bottom),
        frame_width,
        "bottom border width should match the frame width"
    );

    BannerLayout { contents }
}
