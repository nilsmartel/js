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
        "break",
        // following are not used yet
        "do",
        "switch",
        "typeof",
        "of",
        "in",
        "const",
        "var",
        "class",
        "constructor",
    ]
    .contains(&input)
}
