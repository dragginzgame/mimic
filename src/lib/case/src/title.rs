// to_title_case
pub fn to_title_case(input: &str) -> String {
    // These are common small words that are not usually capitalized in titles
    let small_words = [
        "a", "and", "as", "at", "by", "for", "in", "nor", "of", "on", "or", "the", "to", "with",
    ];
    let mut result = String::new();

    // Split the input into words
    let words: Vec<&str> = input.split_whitespace().collect();

    for (i, &word) in words.iter().enumerate() {
        // Always capitalize the first and last word
        // For other words, capitalize them unless they are small words
        if i == 0 || i == words.len() - 1 || !small_words.contains(&word) {
            let mut c = word.chars();
            let upper = c.next().unwrap().to_uppercase().collect::<String>();
            result.push_str(&upper);
            result.push_str(c.as_str());
        } else {
            result.push_str(word);
        }

        // Append a space after each word
        result.push(' ');
    }

    // Remove the trailing space and return the result
    result.trim().to_owned()
}

//
// TESTS
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_title() {
        let test_cases = vec![
            ("Come by", "Come By"),
            ("test me", "Test Me"),
            ("Test Me", "Test Me"),
            ("Spaces ", "Spaces"),
            ("Spaces   ", "Spaces"),
            ("   Spaces", "Spaces"),
            ("   Spaces   ", "Spaces"),
            ("    non title text ", "Non Title Text"),
            (" the   book    of peas ", "The Book of Peas"),
            ("I'm loving it", "I'm Loving It"), // Short forms
            ("war and peace", "War and Peace"), // Short words
        ];

        for (input, expected) in test_cases {
            assert_eq!(to_title_case(input), expected);
        }
    }
}
