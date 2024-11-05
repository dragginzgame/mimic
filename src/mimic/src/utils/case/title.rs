// to_title_case
pub fn to_title_case(input: &str) -> String {
    // These are common small words that are not usually capitalized in titles
    let small_words = [
        "a", "and", "as", "at", "by", "for", "in", "nor", "of", "on", "or", "the", "to", "with",
    ];

    // Split the input into words and map them to a new vector with proper capitalization
    let words: Vec<String> = input
        .split_whitespace()
        .enumerate()
        .map(|(i, word)| {
            // Always capitalize the first and last word or words not in small_words
            if i == 0
                || i == input.split_whitespace().count() - 1
                || !small_words.contains(&word.to_lowercase().as_str())
            {
                capitalize_first(word)
            } else {
                word.to_lowercase()
            }
        })
        .collect();

    words.join(" ")
}

// Helper function to capitalize the first letter of a word
fn capitalize_first(word: &str) -> String {
    let mut chars = word.chars();
    if let Some(first) = chars.next() {
        first.to_uppercase().collect::<String>() + chars.as_str()
    } else {
        String::new()
    }
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
            ("Group Of Green Sacks", "Group of Green Sacks"),
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
