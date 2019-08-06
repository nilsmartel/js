///
/// Keywords
///
/// JavaScript contains several keywords.
/// This will only be a subset.

pub fn is_keyword(input: &str) -> bool {
    vec![
        "let",
        "for",
        "while",
        "if",
        "else",
        "function",
        "return",
        "in",
        // following are not used yet
        "do",
        "switch",
        "typeof",
        "of",
        "const",
        "var",
        "class",
        "constructor",
        "break",
    ]
    .contains(&input)
}
