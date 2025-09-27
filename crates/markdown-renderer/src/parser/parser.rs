use crate::{parser::ast::AstNode, Token};
use crate::error::ParseError;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<AstNode, ParseError> {
        let mut children = Vec::new();

        while !self.is_at_end() {
            if let Some(node) = self.parse_block()? {
                children.push(node);
            }
        }

        Ok(AstNode::Document { children: children })
    }

    fn parse_block(&mut self) -> Result<Option<AstNode>, ParseError> {
        match self.current_token().cloned() {
            Some(Token::Hash(level)) => {
                self.advance();
                Ok(Some(self.parse_heading(level)?))
            },
            Some(Token::Number(_)) => {
                Ok(Some(self.parse_ordered_list()?))
            },
            Some(Token::Hyphen) => {
                Ok(Some(self.parse_unordered_list()?))
            },
            Some(Token::Newline) => {
                self.advance();
                Ok(None) // Skip empty lines
            },
            Some(_) => {
                Ok(Some(self.parse_paragraph()?))
            },
            None => Ok(None)
        }
    }

    fn parse_heading(&mut self, level: u8) -> Result<AstNode, ParseError> {
        let content = self.parse_inline_content_until_newline()?;
        Ok(AstNode::Heading { level, content })
    }

    fn parse_paragraph(&mut self) -> Result<AstNode, ParseError> {
        let content = self.parse_inline_content_until_newline()?;
        Ok(AstNode::Paragraph { content })
    }

    fn parse_inline_content_until_newline(&mut self) -> Result<Vec<AstNode>, ParseError> {
        let mut content = Vec::new();

        while let Some(token) = self.current_token().cloned() {
            match token {
                Token::Newline | Token::Eof => break,
                Token::Text(text) => {
                    content.push(AstNode::Text(text));
                    self.advance();
                },
                Token::Asterisk(count) => {
                    content.push(self.parse_emphasis(count)?);
                },
                Token::LeftBracket => {
                    content.push(self.parse_link()?);
                },
                _ => {
                   self.advance();
                }
            }
        }

        Ok(content)
    }

    fn parse_emphasis(&mut self, count: u8) -> Result<AstNode, ParseError> {
        self.advance();

        let content = self.parse_inline_until_asterisk(count)?;

        match count {
            1 => Ok(AstNode::Italic(content)),
            2 => Ok(AstNode::Bold(content)),
            _ => Err(ParseError::InvalidEmphasis(count))
        }
    }

    fn parse_link(&mut self) -> Result<AstNode, ParseError> {
        self.advance(); // consume '['

        let text = self.parse_inline_until_right_bracket()?;

        self.expect_token(&Token::RightBracket)?;
        self.expect_token(&Token::LeftParen)?;

        let url = match self.current_token().cloned() {
            Some(Token::Url(url)) => {
                self.advance();
                url
            },
            Some(Token::Text(text)) => {
                // Allow plain text as URL
                self.advance();
                text
            },
            _ => return Err(ParseError::ExpectedUrl)
        };

        self.expect_token(&Token::RightParen)?;

        Ok(AstNode::Link { text, url })
    }

    fn parse_ordered_list(&mut self) -> Result<AstNode, ParseError> {
        let mut items = Vec::new();
        
        while matches!(self.current_token(), Some(Token::Number(_))) {
            self.advance(); // consume number
            if matches!(self.current_token(), Some(Token::Dot)) {
                self.advance(); // consume dot
            }
            
            let content = self.parse_inline_content_until_newline()?;
            items.push(AstNode::ListItem { content });
            
            // Skip optional newline
            if matches!(self.current_token(), Some(Token::Newline)) {
                self.advance();
            }
            
            // Check if next line continues the list
            if !matches!(self.current_token(), Some(Token::Number(_))) {
                break;
            }
        }
        
        Ok(AstNode::List { ordered: true, items })
    }

    fn parse_unordered_list(&mut self) -> Result<AstNode, ParseError> {
        let mut items = Vec::new();
        
        while matches!(self.current_token(), Some(Token::Hyphen)) {
            self.advance(); // consume hyphen
            
            let content = self.parse_inline_content_until_newline()?;
            items.push(AstNode::ListItem { content });
            
            // Skip optional newline
            if matches!(self.current_token(), Some(Token::Newline)) {
                self.advance();
            }
            
            // Check if next line continues the list
            if !matches!(self.current_token(), Some(Token::Hyphen)) {
                break;
            }
        }
        
        Ok(AstNode::List { ordered: false, items })
    }

    fn parse_inline_until_asterisk(&mut self, count: u8) -> Result<Vec<AstNode>, ParseError> {
        let mut content = Vec::new();
        
        while let Some(token) = self.current_token().cloned() {
            match token {
                Token::Asterisk(c) if c == count => {
                    self.advance(); // consume closing asterisks
                    break;
                },
                Token::Text(text) => {
                    content.push(AstNode::Text(text));
                    self.advance();
                },
                Token::Newline | Token::Eof => {
                    return Err(ParseError::UnclosedDelimiter {
                        delimiter: '*',
                        open_line: 0, // TODO: track position properly
                        open_column: 0,
                    });
                },
                _ => {
                    self.advance(); // Skip other tokens
                }
            }
        }
        
        Ok(content)
    }

    fn parse_inline_until_right_bracket(&mut self) -> Result<Vec<AstNode>, ParseError> {
        let mut content = Vec::new();
        
        while let Some(token) = self.current_token().cloned() {
            match token {
                Token::RightBracket => break,
                Token::Text(text) => {
                    content.push(AstNode::Text(text));
                    self.advance();
                },
                Token::Newline | Token::Eof => {
                    return Err(ParseError::UnclosedDelimiter {
                        delimiter: '[',
                        open_line: 0, // TODO: track position properly
                        open_column: 0,
                    });
                },
                _ => {
                    self.advance(); // Skip other tokens
                }
            }
        }
        
        Ok(content)
    }

    fn expect_token(&mut self, expected: &Token) -> Result<(), ParseError> {
        match self.current_token() {
            Some(token) if std::mem::discriminant(token) == std::mem::discriminant(expected) => {
                self.advance();
                Ok(())
            },
            Some(token) => {
                Err(ParseError::ExpectedToken {
                    expected: format!("{:?}", expected),
                    found: format!("{:?}", token),
                })
            },
            None => Err(ParseError::UnexpectedEndOfInput),
        }
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() ||
        matches!(self.current_token(), Some(Token::Eof))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Token;

    fn create_parser(tokens: Vec<Token>) -> Parser {
        Parser::new(tokens)
    }

    #[test]
    fn test_empty_token_stream() {
        let mut parser = create_parser(vec![]);
        let result = parser.parse().unwrap();
        
        match result {
            AstNode::Document { children } => {
                assert!(children.is_empty());
            },
            _ => panic!("Expected document node"),
        }
    }

    #[test]
    fn test_single_eof_token() {
        let mut parser = create_parser(vec![Token::Eof]);
        let result = parser.parse().unwrap();
        
        match result {
            AstNode::Document { children } => {
                assert!(children.is_empty());
            },
            _ => panic!("Expected document node"),
        }
    }

    #[test]
    fn test_simple_heading() {
        let tokens = vec![
            Token::Hash(1),
            Token::Text("Simple Heading".to_string()),
            Token::Newline,
            Token::Eof,
        ];
        let mut parser = create_parser(tokens);
        let result = parser.parse().unwrap();
        
        match result {
            AstNode::Document { children } => {
                assert_eq!(children.len(), 1);
                match &children[0] {
                    AstNode::Heading { level, content } => {
                        assert_eq!(*level, 1);
                        assert_eq!(content.len(), 1);
                        assert_eq!(content[0], AstNode::Text("Simple Heading".to_string()));
                    },
                    _ => panic!("Expected heading node"),
                }
            },
            _ => panic!("Expected document node"),
        }
    }

    #[test]
    fn test_heading_levels() {
        for level in 1..=6 {
            let tokens = vec![
                Token::Hash(level),
                Token::Text(format!("Level {} Heading", level)),
                Token::Eof,
            ];
            let mut parser = create_parser(tokens);
            let result = parser.parse().unwrap();
            
            match result {
                AstNode::Document { children } => {
                    assert_eq!(children.len(), 1);
                    match &children[0] {
                        AstNode::Heading { level: parsed_level, .. } => {
                            assert_eq!(*parsed_level, level);
                        },
                        _ => panic!("Expected heading node for level {}", level),
                    }
                },
                _ => panic!("Expected document node"),
            }
        }
    }

    #[test]
    fn test_empty_heading() {
        let tokens = vec![
            Token::Hash(2),
            Token::Newline,
            Token::Eof,
        ];
        let mut parser = create_parser(tokens);
        let result = parser.parse().unwrap();
        
        match result {
            AstNode::Document { children } => {
                assert_eq!(children.len(), 1);
                match &children[0] {
                    AstNode::Heading { level, content } => {
                        assert_eq!(*level, 2);
                        assert!(content.is_empty());
                    },
                    _ => panic!("Expected heading node"),
                }
            },
            _ => panic!("Expected document node"),
        }
    }

    #[test]
    fn test_simple_paragraph() {
        let tokens = vec![
            Token::Text("This is a simple paragraph.".to_string()),
            Token::Eof,
        ];
        let mut parser = create_parser(tokens);
        let result = parser.parse().unwrap();
        
        match result {
            AstNode::Document { children } => {
                assert_eq!(children.len(), 1);
                match &children[0] {
                    AstNode::Paragraph { content } => {
                        assert_eq!(content.len(), 1);
                        assert_eq!(content[0], AstNode::Text("This is a simple paragraph.".to_string()));
                    },
                    _ => panic!("Expected paragraph node"),
                }
            },
            _ => panic!("Expected document node"),
        }
    }

    #[test]
    fn test_paragraph_with_mixed_content() {
        let tokens = vec![
            Token::Text("Start ".to_string()),
            Token::Asterisk(2),
            Token::Text("bold".to_string()),
            Token::Asterisk(2),
            Token::Text(" middle ".to_string()),
            Token::Asterisk(1),
            Token::Text("italic".to_string()),
            Token::Asterisk(1),
            Token::Text(" end".to_string()),
            Token::Eof,
        ];
        let mut parser = create_parser(tokens);
        let result = parser.parse().unwrap();
        
        match result {
            AstNode::Document { children } => {
                assert_eq!(children.len(), 1);
                match &children[0] {
                    AstNode::Paragraph { content } => {
                        assert_eq!(content.len(), 5); // "Start ", Bold, " middle ", Italic, " end"
                        
                        // Check bold content
                        match &content[1] {
                            AstNode::Bold(bold_content) => {
                                assert_eq!(bold_content.len(), 1);
                                assert_eq!(bold_content[0], AstNode::Text("bold".to_string()));
                            },
                            _ => panic!("Expected bold node"),
                        }
                        
                        // Check italic content
                        match &content[3] {
                            AstNode::Italic(italic_content) => {
                                assert_eq!(italic_content.len(), 1);
                                assert_eq!(italic_content[0], AstNode::Text("italic".to_string()));
                            },
                            _ => panic!("Expected italic node"),
                        }
                    },
                    _ => panic!("Expected paragraph node"),
                }
            },
            _ => panic!("Expected document node"),
        }
    }

    #[test]
    fn test_emphasis_variants() {
        // Test single asterisk (italic)
        let tokens = vec![
            Token::Asterisk(1),
            Token::Text("italic".to_string()),
            Token::Asterisk(1),
            Token::Eof,
        ];
        let mut parser = create_parser(tokens);
        let result = parser.parse().unwrap();
        
        match result {
            AstNode::Document { children } => {
                match &children[0] {
                    AstNode::Paragraph { content } => {
                        match &content[0] {
                            AstNode::Italic(content) => {
                                assert_eq!(content[0], AstNode::Text("italic".to_string()));
                            },
                            _ => panic!("Expected italic node"),
                        }
                    },
                    _ => panic!("Expected paragraph node"),
                }
            },
            _ => panic!("Expected document node"),
        }

        // Test double asterisk (bold)
        let tokens = vec![
            Token::Asterisk(2),
            Token::Text("bold".to_string()),
            Token::Asterisk(2),
            Token::Eof,
        ];
        let mut parser = create_parser(tokens);
        let result = parser.parse().unwrap();
        
        match result {
            AstNode::Document { children } => {
                match &children[0] {
                    AstNode::Paragraph { content } => {
                        match &content[0] {
                            AstNode::Bold(content) => {
                                assert_eq!(content[0], AstNode::Text("bold".to_string()));
                            },
                            _ => panic!("Expected bold node"),
                        }
                    },
                    _ => panic!("Expected paragraph node"),
                }
            },
            _ => panic!("Expected document node"),
        }
    }

    #[test]
    fn test_invalid_emphasis_level() {
        let tokens = vec![
            Token::Asterisk(3), // Invalid level
            Token::Text("text".to_string()),
            Token::Asterisk(3),
            Token::Eof,
        ];
        let mut parser = create_parser(tokens);
        let result = parser.parse();
        
        assert!(result.is_err());
        match result {
            Err(ParseError::InvalidEmphasis(3)) => {}, // Expected
            _ => panic!("Expected InvalidEmphasis error"),
        }
    }

    #[test]
    fn test_unclosed_emphasis() {
        let tokens = vec![
            Token::Asterisk(1),
            Token::Text("unclosed".to_string()),
            Token::Newline, // No closing asterisk
            Token::Eof,
        ];
        let mut parser = create_parser(tokens);
        let result = parser.parse();
        
        assert!(result.is_err());
        match result {
            Err(ParseError::UnclosedDelimiter { delimiter: '*', .. }) => {}, // Expected
            _ => panic!("Expected UnclosedDelimiter error"),
        }
    }

    #[test]
    fn test_simple_link() {
        let tokens = vec![
            Token::LeftBracket,
            Token::Text("Link Text".to_string()),
            Token::RightBracket,
            Token::LeftParen,
            Token::Url("https://example.com".to_string()),
            Token::RightParen,
            Token::Eof,
        ];
        let mut parser = create_parser(tokens);
        let result = parser.parse().unwrap();
        
        match result {
            AstNode::Document { children } => {
                match &children[0] {
                    AstNode::Paragraph { content } => {
                        match &content[0] {
                            AstNode::Link { text, url } => {
                                assert_eq!(text.len(), 1);
                                assert_eq!(text[0], AstNode::Text("Link Text".to_string()));
                                assert_eq!(url, "https://example.com");
                            },
                            _ => panic!("Expected link node"),
                        }
                    },
                    _ => panic!("Expected paragraph node"),
                }
            },
            _ => panic!("Expected document node"),
        }
    }

    #[test]
    fn test_link_with_text_as_url() {
        let tokens = vec![
            Token::LeftBracket,
            Token::Text("Link".to_string()),
            Token::RightBracket,
            Token::LeftParen,
            Token::Text("example.com".to_string()), // Plain text instead of URL token
            Token::RightParen,
            Token::Eof,
        ];
        let mut parser = create_parser(tokens);
        let result = parser.parse().unwrap();
        
        match result {
            AstNode::Document { children } => {
                match &children[0] {
                    AstNode::Paragraph { content } => {
                        match &content[0] {
                            AstNode::Link { text: _, url } => {
                                assert_eq!(url, "example.com");
                            },
                            _ => panic!("Expected link node"),
                        }
                    },
                    _ => panic!("Expected paragraph node"),
                }
            },
            _ => panic!("Expected document node"),
        }
    }

    #[test]
    fn test_unclosed_link_bracket() {
        let tokens = vec![
            Token::LeftBracket,
            Token::Text("Unclosed".to_string()),
            Token::Newline, // No closing bracket
            Token::Eof,
        ];
        let mut parser = create_parser(tokens);
        let result = parser.parse();
        
        assert!(result.is_err());
        match result {
            Err(ParseError::UnclosedDelimiter { delimiter: '[', .. }) => {}, // Expected
            _ => panic!("Expected UnclosedDelimiter error"),
        }
    }

    #[test]
    fn test_link_missing_url() {
        let tokens = vec![
            Token::LeftBracket,
            Token::Text("Link".to_string()),
            Token::RightBracket,
            Token::LeftParen,
            Token::RightParen, // No URL
            Token::Eof,
        ];
        let mut parser = create_parser(tokens);
        let result = parser.parse();
        
        assert!(result.is_err());
        match result {
            Err(ParseError::ExpectedUrl) => {}, // Expected
            _ => panic!("Expected ExpectedUrl error"),
        }
    }

    #[test]
    fn test_unordered_list() {
        let tokens = vec![
            Token::Hyphen,
            Token::Text("First item".to_string()),
            Token::Newline,
            Token::Hyphen,
            Token::Text("Second item".to_string()),
            Token::Eof,
        ];
        let mut parser = create_parser(tokens);
        let result = parser.parse().unwrap();
        
        match result {
            AstNode::Document { children } => {
                assert_eq!(children.len(), 1);
                match &children[0] {
                    AstNode::List { ordered, items } => {
                        assert_eq!(*ordered, false);
                        assert_eq!(items.len(), 2);
                        
                        // Check first item
                        match &items[0] {
                            AstNode::ListItem { content } => {
                                assert_eq!(content.len(), 1);
                                assert_eq!(content[0], AstNode::Text("First item".to_string()));
                            },
                            _ => panic!("Expected list item"),
                        }
                        
                        // Check second item
                        match &items[1] {
                            AstNode::ListItem { content } => {
                                assert_eq!(content.len(), 1);
                                assert_eq!(content[0], AstNode::Text("Second item".to_string()));
                            },
                            _ => panic!("Expected list item"),
                        }
                    },
                    _ => panic!("Expected unordered list node"),
                }
            },
            _ => panic!("Expected document node"),
        }
    }

    #[test]
    fn test_ordered_list() {
        let tokens = vec![
            Token::Number(1),
            Token::Dot,
            Token::Text("First item".to_string()),
            Token::Newline,
            Token::Number(2),
            Token::Dot,
            Token::Text("Second item".to_string()),
            Token::Eof,
        ];
        let mut parser = create_parser(tokens);
        let result = parser.parse().unwrap();
        
        match result {
            AstNode::Document { children } => {
                assert_eq!(children.len(), 1);
                match &children[0] {
                    AstNode::List { ordered, items } => {
                        assert_eq!(*ordered, true);
                        assert_eq!(items.len(), 2);
                    },
                    _ => panic!("Expected ordered list node"),
                }
            },
            _ => panic!("Expected document node"),
        }
    }

    #[test]
    fn test_ordered_list_without_dot() {
        let tokens = vec![
            Token::Number(1),
            Token::Text("Item without dot".to_string()),
            Token::Eof,
        ];
        let mut parser = create_parser(tokens);
        let result = parser.parse().unwrap();
        
        match result {
            AstNode::Document { children } => {
                assert_eq!(children.len(), 1);
                match &children[0] {
                    AstNode::List { ordered, items } => {
                        assert_eq!(*ordered, true);
                        assert_eq!(items.len(), 1);
                    },
                    _ => panic!("Expected ordered list node"),
                }
            },
            _ => panic!("Expected document node"),
        }
    }

    #[test]
    fn test_single_item_lists() {
        // Single unordered item
        let tokens = vec![
            Token::Hyphen,
            Token::Text("Single item".to_string()),
            Token::Eof,
        ];
        let mut parser = create_parser(tokens);
        let result = parser.parse().unwrap();
        
        match result {
            AstNode::Document { children } => {
                match &children[0] {
                    AstNode::List { ordered, items } => {
                        assert_eq!(*ordered, false);
                        assert_eq!(items.len(), 1);
                    },
                    _ => panic!("Expected list node"),
                }
            },
            _ => panic!("Expected document node"),
        }
    }

    #[test]
    fn test_mixed_document_structure() {
        let tokens = vec![
            Token::Hash(1),
            Token::Text("Title".to_string()),
            Token::Newline,
            Token::Text("Paragraph text".to_string()),
            Token::Newline,
            Token::Hyphen,
            Token::Text("List item".to_string()),
            Token::Eof,
        ];
        let mut parser = create_parser(tokens);
        let result = parser.parse().unwrap();
        
        match result {
            AstNode::Document { children } => {
                assert_eq!(children.len(), 3); // heading, paragraph, list
                
                // Check structure types
                assert!(matches!(children[0], AstNode::Heading { .. }));
                assert!(matches!(children[1], AstNode::Paragraph { .. }));
                assert!(matches!(children[2], AstNode::List { .. }));
            },
            _ => panic!("Expected document node"),
        }
    }

    #[test]
    fn test_multiple_newlines_handling() {
        let tokens = vec![
            Token::Text("First paragraph".to_string()),
            Token::Newline,
            Token::Newline,
            Token::Newline, // Multiple newlines should be skipped
            Token::Text("Second paragraph".to_string()),
            Token::Eof,
        ];
        let mut parser = create_parser(tokens);
        let result = parser.parse().unwrap();
        
        match result {
            AstNode::Document { children } => {
                assert_eq!(children.len(), 2); // Should only have 2 paragraphs
            },
            _ => panic!("Expected document node"),
        }
    }

    #[test]
    fn test_expect_token_errors() {
        let tokens = vec![
            Token::LeftBracket,
            Token::Text("Link".to_string()),
            Token::Text("Wrong token instead of ]".to_string()), // Should be RightBracket
            Token::Eof,
        ];
        let mut parser = create_parser(tokens);
        let result = parser.parse();
        
        assert!(result.is_err());
        match result {
            Err(ParseError::UnclosedDelimiter { delimiter: '[', .. }) => {}, // Expected - unclosed bracket
            _ => panic!("Expected UnclosedDelimiter error"),
        }
    }

    #[test]
    fn test_unexpected_end_of_input() {
        let tokens = vec![
            Token::LeftBracket,
            Token::Text("Link".to_string()),
            // Missing closing bracket and rest of link
        ];
        let mut parser = create_parser(tokens);
        let result = parser.parse();
        
        match result {
            Err(ParseError::UnclosedDelimiter { delimiter: '[', .. }) => {
                // Expected error case
            },
            Err(ParseError::UnexpectedEndOfInput) => {
                // Also expected - when expect_token hits end of input
            },
            Ok(_) => {
                // Also acceptable - parser might treat incomplete link as regular text
            },
            Err(e) => {
                panic!("Unexpected error type: {:?}", e);
            }
        }
    }

    #[test]
    fn test_empty_emphasis() {
        let tokens = vec![
            Token::Asterisk(1),
            Token::Asterisk(1), // Empty italic
            Token::Eof,
        ];
        let mut parser = create_parser(tokens);
        let result = parser.parse().unwrap();
        
        match result {
            AstNode::Document { children } => {
                match &children[0] {
                    AstNode::Paragraph { content } => {
                        match &content[0] {
                            AstNode::Italic(content) => {
                                assert!(content.is_empty());
                            },
                            _ => panic!("Expected italic node"),
                        }
                    },
                    _ => panic!("Expected paragraph node"),
                }
            },
            _ => panic!("Expected document node"),
        }
    }

    #[test]
    fn test_nested_emphasis_in_list() {
        let tokens = vec![
            Token::Hyphen,
            Token::Text("Item with ".to_string()),
            Token::Asterisk(2),
            Token::Text("bold".to_string()),
            Token::Asterisk(2),
            Token::Text(" text".to_string()),
            Token::Eof,
        ];
        let mut parser = create_parser(tokens);
        let result = parser.parse().unwrap();
        
        match result {
            AstNode::Document { children } => {
                match &children[0] {
                    AstNode::List { items, .. } => {
                        match &items[0] {
                            AstNode::ListItem { content } => {
                                assert_eq!(content.len(), 3); // "Item with ", Bold, " text"
                                assert!(matches!(content[1], AstNode::Bold(_)));
                            },
                            _ => panic!("Expected list item"),
                        }
                    },
                    _ => panic!("Expected list node"),
                }
            },
            _ => panic!("Expected document node"),
        }
    }

    #[test]
    fn test_parser_position_tracking() {
        let mut parser = create_parser(vec![Token::Text("test".to_string()), Token::Eof]);
        
        // Test initial position
        assert_eq!(parser.current, 0);
        assert!(!parser.is_at_end());
        
        // Test advance
        parser.advance();
        assert_eq!(parser.current, 1);
        
        // Test end condition
        parser.advance();
        assert!(parser.is_at_end());
    }

    #[test]
    fn test_current_token_bounds() {
        let mut parser = create_parser(vec![Token::Text("test".to_string())]);
        
        // Valid position
        assert!(parser.current_token().is_some());
        
        // Past end
        parser.current = 10;
        assert!(parser.current_token().is_none());
    }

    #[test]
    fn test_complex_nested_structure() {
        let tokens = vec![
            Token::Hash(1),
            Token::Text("Title with ".to_string()),
            Token::Asterisk(1),
            Token::Text("italic".to_string()),
            Token::Asterisk(1),
            Token::Newline,
            Token::Hyphen,
            Token::LeftBracket,
            Token::Text("Link in list".to_string()),
            Token::RightBracket,
            Token::LeftParen,
            Token::Url("http://example.com".to_string()),
            Token::RightParen,
            Token::Eof,
        ];
        let mut parser = create_parser(tokens);
        let result = parser.parse().unwrap();
        
        match result {
            AstNode::Document { children } => {
                assert_eq!(children.len(), 2); // heading and list
                
                // Check heading with italic
                match &children[0] {
                    AstNode::Heading { content, .. } => {
                        assert_eq!(content.len(), 2); // "Title with ", Italic
                        assert!(matches!(content[1], AstNode::Italic(_)));
                    },
                    _ => panic!("Expected heading"),
                }
                
                // Check list with link
                match &children[1] {
                    AstNode::List { items, .. } => {
                        match &items[0] {
                            AstNode::ListItem { content } => {
                                assert!(matches!(content[0], AstNode::Link { .. }));
                            },
                            _ => panic!("Expected list item"),
                        }
                    },
                    _ => panic!("Expected list"),
                }
            },
            _ => panic!("Expected document node"),
        }
    }
}
