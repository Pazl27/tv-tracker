pub mod tokens;
mod lexer;

pub use lexer::Lexer;
pub use tokens::Token;

pub fn tokenize(input: &str) -> Result<Vec<Token>, crate::error::LexerError> {
    let mut lexer = Lexer::new(input);
    lexer.tokenize()
}
