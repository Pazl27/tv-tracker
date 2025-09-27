pub mod ast;
pub mod parser;

pub use ast::AstNode;
pub use parser::Parser;

use crate::error::ParseError;
use crate::Token;

/// Parse a vector of tokens into an AST
pub fn parse(tokens: Vec<Token>) -> Result<AstNode, ParseError> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}
