/// to_snake_case
/// Converts a string to snake case by inserting underscores before uppercase
/// characters and spaces, then lowercasing all characters.
pub fn to_snake_case(s: &str) -> String {
    let mut snake_case = String::new();
    let mut prev_was_uppercase = false;
    let mut prev_was_underscore_or_space = false;

    for (i, ch) in s.trim().chars().enumerate() {
        if ch.is_uppercase() {
            if i != 0 && !prev_was_uppercase && !prev_was_underscore_or_space {
                snake_case.push('_');
            }
            snake_case.extend(ch.to_lowercase());
            prev_was_uppercase = true;
            prev_was_underscore_or_space = false;
        } else if ch == ' ' || ch == '_' {
            if !prev_was_underscore_or_space {
                snake_case.push('_');
            }
            prev_was_uppercase = false;
            prev_was_underscore_or_space = true;
        } else {
            snake_case.push(ch);
            prev_was_uppercase = false;
            prev_was_underscore_or_space = false;
        }
    }

    snake_case
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
            ("CAPiTALS", "capi_tals"),
            ("CAPiTaLS", "capi_ta_ls"),
            ("utf8", "utf8"),
            ("Utf8", "utf8"),
            ("UTF8", "utf8"),
            ("UTF8___UTF8", "utf8_utf8"),
            (" the the the ", "the_the_the"),
            ("MyExampleString123", "my_example_string123"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(to_snake_case(input), expected);
        }
    }
}
