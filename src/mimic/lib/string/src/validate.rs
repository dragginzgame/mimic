// max_char_repeat
//
// name chosen because it is looking for the max, plus it could possibly work
// well with max_string_repeat if we needed to check for arbitrary length string repeats
#[must_use]
pub fn max_char_repeat<S: AsRef<str>>(s: S) -> usize {
    let s = s.as_ref();
    if s.len() <= 1 {
        return s.len();
    }

    let mut previous_char: Option<char> = None;
    let mut count = 0;
    let mut max = 1; // non-empty string

    for c in s.chars() {
        if Some(c) == previous_char {
            count += 1;
            if count > max {
                max = count;
            }
        } else {
            previous_char = Some(c);
            count = 1;
        }
    }

    max
}

//
// TESTS
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_char_repeat() {
        let test_cases = [
            ("", 0),
            ("a", 1),
            ("aa", 2),
            ("123454321", 1),
            ("baaa aaab", 3),
            ("baaaaaab", 6),
            ("x xx xxx xxxx xxx xx x", 4),
            ("AaAaAaAaAaAa", 1),
            ("abcdef", 1),
            ("aaabbbccc", 3),
            ("aabbccddeee", 3),
        ];

        for (input, expected) in &test_cases {
            let count = max_char_repeat(input);
            assert_eq!(count, *expected, "testing: '{input}'");
        }
    }
}
