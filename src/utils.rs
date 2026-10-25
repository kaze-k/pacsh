use std::iter;

pub(crate) fn repeat(ch: char, count: usize) -> String {
    let mut text = String::new();

    text.reserve(count);
    text.extend(iter::repeat_n(ch, count));

    text
}

pub(crate) fn unknown(value: &str) -> Option<()> {
    if value == "unknown" { Some(()) } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeat_builds_requested_characters() {
        assert_eq!(repeat('x', 0), "");
        assert_eq!(repeat('x', 1), "x");
        assert_eq!(repeat('x', 5), "xxxxx");
        assert_eq!(repeat(' ', 4), "    ");
        assert_eq!(repeat(' ', 4).len(), 4);
        assert_eq!(repeat('─', 3), "───");
        assert_eq!(repeat('─', 3).len(), 9);
    }

    #[test]
    fn repeat_handles_multibyte_and_control_characters() {
        assert_eq!(repeat('é', 2), "éé");
        assert_eq!(repeat('é', 2).len(), 4);
        assert_eq!(repeat('🙂', 2), "🙂🙂");
        assert_eq!(repeat('🙂', 2).len(), 8);
        assert_eq!(repeat('\u{0301}', 2), "\u{0301}\u{0301}");
        assert_eq!(repeat('\n', 3), "\n\n\n");
        assert_eq!(repeat('\0', 2), "\0\0");
    }

    #[test]
    fn repeat_handles_large_counts() {
        let repeated = repeat('x', 1000);

        assert_eq!(repeated.len(), 1000);
        assert_eq!(repeated, "x".repeat(1000));
    }

    #[test]
    fn unknown_only_matches_exact_marker() {
        let cases = [
            ("unknown", Some(())),
            ("Unknown", None),
            ("UNKNOWN", None),
            ("unknowns", None),
            ("xunknown", None),
            ("unknown!", None),
            (" unknown", None),
            ("unknown ", None),
            ("unknown\n", None),
            ("unknown\t", None),
            ("unknown\0", None),
            ("true", None),
            ("false", None),
            ("", None),
        ];

        for (value, expected) in cases {
            assert_eq!(unknown(value), expected, "unknown({value:?})");
        }
    }

    #[test]
    fn unknown_is_none_covers_both_boolean_outcomes() {
        assert!(!unknown("unknown").is_none());
        assert!(unknown("known").is_none());
    }
}
