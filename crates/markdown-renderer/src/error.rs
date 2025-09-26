use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum LexerError {
    #[error("Unexpected character '{char}' at line {line}, column {column}")]
    UnexpectedCharacter {
        char: char,
        line: usize,
        column: usize
    },

    #[error("Unexpected end of file at line {line}, column {column}")]
    UnexpectedEof {
        line: usize,
        column: usize
    },

    #[error("Invalid number '{text}' at line {line}, column {column}")]
    InvalidNumber {
        text: String,
        line: usize,
        column: usize
    },

    #[error("Invalid UTF-8 sequence at position {position}")]
    InvalidUtf8 {
        position: usize
    },

    #[error("Maximum nesting depth exceeded at line {line}, column {column}")]
    MaxDepthExceeded {
        line: usize,
        column: usize
    },
}

// You can also define parser errors here
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ParseError {
    #[error("Lexer error: {0}")]
    LexerError(#[from] LexerError),

    #[error("Unexpected token {token:?} at line {line}, column {column}")]
    UnexpectedToken {
        token: String,  // Use String instead of Token to avoid circular deps
        line: usize,
        column: usize
    },

    #[error("Unclosed delimiter '{delimiter}' opened at line {open_line}, column {open_column}")]
    UnclosedDelimiter {
        delimiter: char,
        open_line: usize,
        open_column: usize,
    },

    #[error("Invalid heading level {level} at line {line}, column {column} (must be 1-6)")]
    InvalidHeadingLevel {
        level: u8,
        line: usize,
        column: usize
    },
}

// General error type for the entire markdown renderer
#[derive(Error, Debug)]
pub enum MarkdownError {
    #[error("Parse error: {0}")]
    ParseError(#[from] ParseError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Rendering error: {message}")]
    RenderError { message: String },
}
