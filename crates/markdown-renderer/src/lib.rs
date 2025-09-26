pub mod error;
pub mod parser;

mod lexer;

// Re-export commonly used items
pub use error::{LexerError, ParseError, MarkdownError};
pub use lexer::{Lexer, Token, tokenize};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_api() {
        let tokens = tokenize("# Hello **World**").unwrap();
        assert_eq!(tokens[0], Token::Hash(1));
        assert!(matches!(tokens[1], Token::Text(_)));
        assert_eq!(tokens[2], Token::Asterisk(2));
    }
}
