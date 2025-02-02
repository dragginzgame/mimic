use crate::schema::build::schema_builder;

// validate_ident
pub fn validate_ident(ident: &str) -> Result<(), String> {
    if ident.is_empty() {
        return Err("ident is empty".to_string());
    }

    // reserved?
    is_reserved(ident)?;

    Ok(())
}

// is_reserved
pub fn is_reserved(word: &str) -> Result<(), String> {
    if has_reserved_prefix(word) {
        return Err(format!("the word '{word}' has a reserved prefix"));
    }

    if is_reserved_word(word) {
        return Err(format!("the word '{word}' is reserved"));
    }

    Ok(())
}

// has_reserved_prefix
fn has_reserved_prefix(s: &str) -> bool {
    schema_builder()
        .reserved_prefixes
        .iter()
        .any(|&prefix| s.starts_with(prefix))
}

// is_reserved_word
fn is_reserved_word(s: &str) -> bool {
    schema_builder().reserved_words.contains(s)
}
