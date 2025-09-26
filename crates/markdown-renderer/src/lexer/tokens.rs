#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Basic content
    Text(String),
    Newline,
    Eof,

    // Markdown markers
    Hash(u8),          // # ## ### etc. - store level directly
    Asterisk(u8),      // * ** - store count
    Underscore(u8),    // _ __ - store count
    LeftBracket,       // [
    RightBracket,      // ]
    LeftParen,         // (
    RightParen,        // )
    Hyphen,            // -

    Number(u32),
    Dot,
    Url(String)
}
