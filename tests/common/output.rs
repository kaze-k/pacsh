const LABEL_COLUMN: usize = 10;

/// 去除 ANSI CSI 序列，得到终端中实际可见的文本。
pub fn strip_ansi(input: &str) -> String {
    let mut visible = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' && chars.peek() == Some(&'[') {
            chars.next();

            // 消费 CSI 序列：参数/中间字节之后是终止字节。
            for next in chars.by_ref() {
                if ('@'..='~').contains(&next) {
                    break;
                }
            }
        } else {
            visible.push(ch);
        }
    }

    visible
}

/// 提取正文行，但不施加完整边框契约。
///
/// 布局测试使用 `assert_exact_frame`；内容测试只需要行内字段，
/// 不应在这里隐式重复布局断言。
pub fn parse_banner(raw: &str) -> Vec<String> {
    let visible = strip_ansi(raw);

    visible
        .lines()
        .filter_map(|line| {
            let rest = line.strip_prefix("    │")?;
            let inner = rest.strip_suffix('│')?;
            let body = inner.strip_prefix("    ")?;

            Some(body.trim_end_matches(' ').to_string())
        })
        .collect()
}

/// 断言内容行具有固定标签列，并返回其值。
pub fn field_value<'a>(row: &'a str, label: &str) -> &'a str {
    let expected_prefix = format!("{label:<LABEL_COLUMN$}");

    assert!(
        row.starts_with(&expected_prefix),
        "row should start with {label:?} in the fixed label column: {row:?}"
    );

    let value = row
        .get(LABEL_COLUMN..)
        .unwrap_or_else(|| panic!("row is too short for a value: {row:?}"));

    assert!(!value.is_empty(), "row should have a value after {label:?}");
    value
}
