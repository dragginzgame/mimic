/// Converts a string to snake_case format.
///
/// This function inserts underscores between word boundaries and lowercases all characters.
/// Word boundaries are detected between:
/// - lowercase/digit → uppercase (e.g. `"fooBar"` → `"foo_bar"`)
/// - digit → uppercase (e.g. `"Shape2D"` → `"shape2_d"`)
/// - acronym + word (e.g. `"HTTPServer"` → `"http_server"`)
/// - spaces or underscores are normalized as separators
/// - non-alphanumeric characters are removed
///
/// Leading/trailing underscores and whitespace are trimmed.
///
/// # Examples
/// ```
/// assert_eq!(to_snake_case("PascalCase"), "pascal_case");
/// assert_eq!(to_snake_case("XMLHttpRequest"), "xml_http_request");
/// assert_eq!(to_snake_case("  _Hello World_  "), "hello_world");
/// assert_eq!(to_snake_case("Shape2D"), "shape2_d");
/// assert_eq!(to_snake_case("!@#$"), "");
/// ```
pub fn to_snake_case(s: &str) -> String {
    let mut snake_case = String::new();
    let chars: Vec<char> = s.trim().chars().collect();

    for i in 0..chars.len() {
        let ch = chars[i];
        let prev = chars.get(i.wrapping_sub(1)).copied();
        let next = chars.get(i + 1).copied();

        if ch.is_uppercase() {
            let prev_is_lower_or_digit =
                prev.is_some_and(|p| p.is_lowercase() || p.is_ascii_digit());
            let next_is_lower = next.is_some_and(char::is_lowercase);

            if i != 0 && !snake_case.ends_with('_') && (prev_is_lower_or_digit || next_is_lower) {
                snake_case.push('_');
            }

            snake_case.extend(ch.to_lowercase());
        } else if ch == ' ' || ch == '_' {
            if !snake_case.ends_with('_') {
                snake_case.push('_');
            }
        } else if ch.is_alphanumeric() {
            snake_case.push(ch);
        }
    }

    snake_case.trim_matches('_').to_string()
}

//
// TESTS
//

#[cfg(test)]
/// Tests converting strings to snake case. Runs through a series of test cases,
/// converting input strings to snake case and comparing the result to the
/// expected output.
mod tests {
    use super::*;

    #[test]
    fn test_to_snake() {
        let test_cases = vec![
            ("PascalCase", "pascal_case"),
            ("camelCase", "camel_case"),
            ("Shape2D", "shape2_d"),
            ("Shape2d", "shape2d"),
            ("a a", "a_a"),
            ("a       a", "a_a"),
            ("CAPITALS", "capitals"),
            ("utf8", "utf8"),
            ("Utf8", "utf8"),
            ("UTF8", "utf8"),
            ("UTF8___UTF8", "utf8_utf8"),
            (" the the the ", "the_the_the"),
            ("MyExampleString123", "my_example_string123"),
            ("HTTPServer", "http_server"),
            ("XMLHttpRequest", "xml_http_request"),
            ("Already_Snake", "already_snake"),
            (" Mixed_Case And  Spacing ", "mixed_case_and_spacing"),
            ("TokenID", "token_id"),
            ("SomeURL123", "some_url123"),
            ("!@#$", ""),
            ("  _Hello_World_  ", "hello_world"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(to_snake_case(input), expected);
        }
    }
}
