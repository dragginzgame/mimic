use std::{collections::HashSet, sync::LazyLock};

// is_reserved
#[must_use]
pub fn is_reserved(word: &str) -> bool {
    is_reserved_word(word) || has_reserved_prefix(word)
}

///
/// PREFIXES
///

// has_reserved_prefix
fn has_reserved_prefix(s: &str) -> bool {
    let mut prefixes = HashSet::<&str>::default();

    prefixes.extend(BE_PREFIXES.iter().copied());
    prefixes.extend(FE_PREFIXES.iter().copied());

    prefixes.iter().any(|&prefix| s.starts_with(prefix))
}

static BE_PREFIXES: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut prefixes = Vec::new();

    // c#
    prefixes.extend(vec!["get_", "set_"]);

    prefixes.into_iter().collect()
});

static FE_PREFIXES: LazyLock<HashSet<&'static str>> = LazyLock::new(HashSet::default);

///
/// WORDS
///

// is_reserved_word
fn is_reserved_word(s: &str) -> bool {
    let mut words = HashSet::<&str>::default();

    words.extend(BE_WORDS.iter().copied());
    words.extend(FE_WORDS.iter().copied());

    words.contains(s)
}

static BE_WORDS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut words = Vec::new();

    // candid
    words.extend(vec![
        "blob",
        "bool",
        "composite_query",
        "empty",
        "float32",
        "float64",
        "func",
        "import",
        "int",
        "int8",
        "int16",
        "int32",
        "int64",
        "nat",
        "nat8",
        "nat16",
        "nat32",
        "nat64",
        "null",
        "oneway",
        "opt",
        "principal",
        "query",
        "record",
        "reserved",
        "service",
        "text",
        "type",
        "variant",
        "vec",
    ]);

    // rust
    // https://doc.rust-lang.org/reference/keywords.html
    words.extend(vec![
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do",
        "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
    ]);

    words.into_iter().collect()
});

static FE_WORDS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut words = Vec::new();

    // c# keywords
    // https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/keywords/
    words.extend(vec![
        "as",
        "base",
        "bool",
        "break",
        "byte",
        "case",
        "catch",
        "char",
        "checked",
        "class",
        "const",
        "class",
        "const",
        "continue",
        "decimal",
        "default",
        "delegate",
        "do",
        "double",
        "editor",
        "else",
        "enum",
        "event",
        "explicit",
        "extern",
        "false",
        "finally",
        "fixed",
        "float",
        "for",
        "foreach",
        "goto",
        "if",
        "implicit",
        "in",
        "int",
        "interface",
        "internal",
        "is",
        "lock",
        "long",
        "namespace",
        "new",
        "null",
        "object",
        "operator",
        "out",
        "override",
        "params",
        "private",
        "protected",
        "public",
        "readonly",
        "ref",
        "return",
        "sbyte",
        "sealed",
        "short",
        "sizeof",
        "stackalloc",
        "static",
        "string",
        "struct",
        "switch",
        "this",
        "throw",
        "true",
        "try",
        "typeof",
        "uint",
        "ulong",
        "unchecked",
        "unsafe",
        "ushort",
        "using",
        "virtual",
        "void",
        "volatile",
        "while",
    ]);

    // js
    // https://www.w3schools.com/js/js_reserved.asp
    words.extend(vec![
        "abstract",
        "arguments",
        "await",
        "boolean",
        "break",
        "byte",
        "case",
        "catch",
        "char",
        "class",
        "const",
        "continue",
        "debugger",
        "default",
        "delete",
        "do",
        "double",
        "else",
        "enum",
        "eval",
        "export",
        "extends",
        "false",
        "final",
        "finally",
        "float",
        "for",
        "function",
        "goto",
        "if",
        "implements",
        "import",
        "in",
        "instanceof",
        "int",
        "interface",
        "let",
        "long",
        "native",
        "new",
        "null",
        "package",
        "private",
        "protected",
        "public",
        "return",
        "short",
        "static",
        "super",
        "switch",
        "synchronized",
        "this",
        "throw",
        "throws",
        "transient",
        "true",
        "try",
        "typeof",
        "var",
        "void",
        "volatile",
        "while",
        "with",
        "yield",
    ]);

    words.into_iter().collect()
});
