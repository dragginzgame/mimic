// collapse_whitespace
pub fn collapse_whitespace<S: AsRef<str>>(s: S) -> String {
    s.as_ref()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

// fix_ms_word
pub fn fix_ms_word<S: AsRef<str>>(s: S) -> String {
    s.as_ref()
        .replace('\u{2026}', "...") // ellipsis
        .replace('\u{00BC}', "1/4") // 1/4
        .replace('\u{00BD}', "1/2") // 1/2
        .replace('\u{00BE}', "3/4") // 3/4
        .replace(['\u{201C}', '\u{201D}'], "\"") // double smart quotes
        .replace(['\u{2018}', '\u{2019}'], "'") // single smart quotes
        .replace(['\u{2013}', '\u{2014}'], "-") // en and em-dash
}

//
// TESTS
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collapse_whitespace() {
        let test_cases = [
            ("", ""),
            ("    ", ""),
            ("\n", ""),
            ("\n\n", ""),
            (" \n \n ", ""),
            ("   hello   ", "hello"),
            ("hello   ", "hello"),
            ("   hello", "hello"),
            ("hello world", "hello world"),
            ("  hello   world  ", "hello world"),
            ("hello   world  \n", "hello world"),
            ("hello\tworld", "hello world"),
            ("\t\na\n\n\nc        \t   \t", "a c"),
        ];

        for (input, expected) in &test_cases {
            let trimmed = collapse_whitespace(input);
            assert_eq!(trimmed, *expected, "testing: {input}");
        }
    }

    #[test]
    fn test_fix_ms_word() {
        let test_cases = vec![
            ("This is an example…", "This is an example..."),
            (
                "“Smart quotes” can be ‘annoying’ sometimes.",
                "\"Smart quotes\" can be 'annoying' sometimes.",
            ),
            (
                "“Replace both… ‘Ellipsis’ and ‘smart quotes’.”",
                "\"Replace both... 'Ellipsis' and 'smart quotes'.\"",
            ),
            ("No special characters", "No special characters"),
            (
                "Em dash— and en dash – test.",
                "Em dash- and en dash - test.",
            ),
            ("Fractions: ½, ¼, and ¾.", "Fractions: 1/2, 1/4, and 3/4."),
        ];

        for (input, expected) in test_cases {
            assert_eq!(fix_ms_word(input), expected);
        }
    }
}
