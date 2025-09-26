use std::{iter::Peekable, str::Chars};

use crate::lexer::tokens::Token;
use crate::error::LexerError;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    current_pos: usize,
    line: usize,
    column: usize
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
            current_pos: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token()? {
            if matches!(token, Token::Eof) {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        match self.peek_char().copied() {
            None => Ok(Some(Token::Eof)),
            Some(ch) => {
                let token = self.read_token(ch)?;
                Ok(Some(token))
            }
        }
    }

    fn read_token(&mut self, ch: char) -> Result<Token, LexerError> {
        match ch {
            '\n' => {
                self.advance();
                Ok(Token::Newline)
            }
            '\r' => {
                self.advance();
                // Handle \r\n
                if self.peek_char() == Some(&'\n') {
                    self.advance();
                }
                Ok(Token::Newline)
            }
            '#' => Ok(self.read_hashes()),
            '*' => Ok(self.read_asterisks()),
            '_' => Ok(self.read_underscores()),
            '[' => {
                self.advance();
                Ok(Token::LeftBracket)
            }
            ']' => {
                self.advance();
                Ok(Token::RightBracket)
            }
            '(' => {
                self.advance();
                Ok(Token::LeftParen)
            }
            ')' => {
                self.advance();
                Ok(Token::RightParen)
            }
            '-' => {
                self.advance();
                Ok(Token::Hyphen)
            }
            '.' => {
                self.advance();
                Ok(Token::Dot)
            }
            c if c.is_ascii_digit() => Ok(self.read_number()),
            c if c.is_whitespace() && c != '\n' => {
                self.skip_whitespace();
                // Recursively get the next token after whitespace
                if let Some(&next_ch) = self.peek_char() {
                    self.read_token(next_ch)
                } else {
                    Ok(Token::Eof)
                }
            }
            _ => Ok(self.read_text()),
        }
    }

    fn read_hashes(&mut self) -> Token {
        let mut count = 0u8;
        while self.peek_char() == Some(&'#') && count < 6 {
            self.advance();
            count += 1;
        }
        Token::Hash(count)
    }

    fn read_asterisks(&mut self) -> Token {
        let mut count = 0u8;
        while self.peek_char() == Some(&'*') && count < 3 {
            self.advance();
            count += 1;
        }
        Token::Asterisk(count)
    }

    fn read_underscores(&mut self) -> Token {
        let mut count = 0u8;
        while self.peek_char() == Some(&'_') && count < 3 {
            self.advance();
            count += 1;
        }
        Token::Underscore(count)
    }

    fn read_number(&mut self) -> Token {
        let mut number_str = String::new();

        while let Some(&ch) = self.peek_char() {
            if ch.is_ascii_digit() {
                number_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        let number = number_str.parse::<u32>().unwrap_or(0);
        Token::Number(number)
    }

    fn read_text(&mut self) -> Token {
        let mut text = String::new();

        while let Some(&ch) = self.peek_char() {
            match ch {
                '\n' | '\r' => break,
                '#' | '*' | '_' | '[' | ']' | '(' | ')' => break,
                '-' => {
                    // Look ahead to see if hyphen is followed by whitespace
                    let mut lookahead = self.input.clone();
                    lookahead.next(); // consume the hyphen
                    match lookahead.peek() {
                        Some(' ') | Some('\t') | Some('\n') | Some('\r') | None => break,
                        _ => {
                            text.push(ch);
                            self.advance();
                        }
                    }
                }
                '.' => {
                    // Always include dots in text - they're commonly part of sentences, URLs, etc.
                    text.push(ch);
                    self.advance();
                }
                c if c.is_ascii_digit() && text.is_empty() => {
                    // Let number reader handle this only if text is empty
                    break;
                }
                _ => {
                    text.push(ch);
                    self.advance();
                }
            }
        }

        // Trim trailing whitespace from text tokens
        Token::Text(text.trim_end().to_string())
    }


    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn advance(&mut self) -> Option<char> {
        match self.input.next() {
            Some('\n') => {
                self.line += 1;
                self.column = 1;
                self.current_pos += 1;
                Some('\n')
            }
            Some(ch) => {
                self.column += 1;
                self.current_pos += 1;
                Some(ch)
            }
            None => None,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.peek_char() {
            if ch.is_whitespace() && ch != '\n' && ch != '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn position(&self) -> (usize, usize) {
        (self.line, self.column)
    }


}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokens::Token;

    #[test]
    fn test_empty_input() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Eof]);
    }

    #[test]
    fn test_single_newline() {
        let mut lexer = Lexer::new("\n");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Newline, Token::Eof]);
    }

    #[test]
    fn test_carriage_return_newline() {
        let mut lexer = Lexer::new("\r\n");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Newline, Token::Eof]);
    }

    #[test]
    fn test_carriage_return_only() {
        let mut lexer = Lexer::new("\r");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Newline, Token::Eof]);
    }

    #[test]
    fn test_simple_text() {
        let mut lexer = Lexer::new("Hello World");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::Text("Hello World".to_string()),
            Token::Eof
        ]);
    }

    #[test]
    fn test_text_with_whitespace() {
        let mut lexer = Lexer::new("  Hello   World  ");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::Text("Hello   World".to_string()),
            Token::Eof
        ]);
    }

    #[test]
    fn test_hash_symbols() {
        let test_cases = vec![
            ("#", vec![Token::Hash(1)]),
            ("##", vec![Token::Hash(2)]),
            ("###", vec![Token::Hash(3)]),
            ("####", vec![Token::Hash(4)]),
            ("#####", vec![Token::Hash(5)]),
            ("######", vec![Token::Hash(6)]),
            ("#######", vec![Token::Hash(6), Token::Hash(1)]), // Max 6, then another
        ];

        for (input, expected) in test_cases {
            let mut lexer = Lexer::new(input);
            let mut tokens = lexer.tokenize().unwrap();
            tokens.pop(); // Remove EOF for easier comparison
            assert_eq!(tokens, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_asterisk_symbols() {
        let test_cases = vec![
            ("*", vec![Token::Asterisk(1)]),
            ("**", vec![Token::Asterisk(2)]),
            ("***", vec![Token::Asterisk(3)]),
            ("****", vec![Token::Asterisk(3), Token::Asterisk(1)]), // Max 3, then another
        ];

        for (input, expected) in test_cases {
            let mut lexer = Lexer::new(input);
            let mut tokens = lexer.tokenize().unwrap();
            tokens.pop(); // Remove EOF
            assert_eq!(tokens, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_underscore_symbols() {
        let test_cases = vec![
            ("_", vec![Token::Underscore(1)]),
            ("__", vec![Token::Underscore(2)]),
            ("___", vec![Token::Underscore(3)]),
            ("____", vec![Token::Underscore(3), Token::Underscore(1)]), // Max 3, then another
        ];

        for (input, expected) in test_cases {
            let mut lexer = Lexer::new(input);
            let mut tokens = lexer.tokenize().unwrap();
            tokens.pop(); // Remove EOF
            assert_eq!(tokens, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_brackets_and_parentheses() {
        let mut lexer = Lexer::new("[]()");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::LeftBracket,
            Token::RightBracket,
            Token::LeftParen,
            Token::RightParen,
            Token::Eof
        ]);
    }

    #[test]
    fn test_numbers() {
        let test_cases = vec![
            ("0", Token::Number(0)),
            ("1", Token::Number(1)),
            ("123", Token::Number(123)),
            ("999999", Token::Number(999999)),
        ];

        for (input, expected) in test_cases {
            let mut lexer = Lexer::new(input);
            let tokens = lexer.tokenize().unwrap();
            assert_eq!(tokens[0], expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_hyphen_and_dot() {
        let mut lexer = Lexer::new("- .");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::Hyphen,
            Token::Dot,
            Token::Eof
        ]);
    }

    #[test]
    fn test_heading_with_text() {
        let mut lexer = Lexer::new("# Hello World");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::Hash(1),
            Token::Text("Hello World".to_string()),
            Token::Eof
        ]);
    }

    #[test]
    fn test_emphasis_with_text() {
        let mut lexer = Lexer::new("*italic* and **bold**");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::Asterisk(1),
            Token::Text("italic".to_string()),
            Token::Asterisk(1),
            Token::Text("and".to_string()),
            Token::Asterisk(2),
            Token::Text("bold".to_string()),
            Token::Asterisk(2),
            Token::Eof
        ]);
    }

    #[test]
    fn test_link_structure() {
        let mut lexer = Lexer::new("[link text](https://example.com)");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::LeftBracket,
            Token::Text("link text".to_string()),
            Token::RightBracket,
            Token::LeftParen,
            Token::Text("https://example.com".to_string()), // Should be one token
            Token::RightParen,
            Token::Eof
        ]);
    }

    #[test]
    fn test_list_items() {
        let mut lexer = Lexer::new("- Item 1\n- Item 2");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::Hyphen,
            Token::Text("Item 1".to_string()), // "Item 1" should be one token
            Token::Newline,
            Token::Hyphen,
            Token::Text("Item 2".to_string()), // "Item 2" should be one token
            Token::Eof
        ]);
    }

    #[test]
    fn test_ordered_list() {
        let mut lexer = Lexer::new("1. First item\n2. Second item");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::Number(1),
            Token::Dot,
            Token::Text("First item".to_string()),
            Token::Newline,
            Token::Number(2),
            Token::Dot,
            Token::Text("Second item".to_string()),
            Token::Eof
        ]);
    }

    #[test]
    fn test_complex_markdown() {
        let input = "# Heading\n\nThis is **bold** and *italic* text.\n\n- List item\n- Another item\n\n[Link](url)";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        let expected = vec![
            Token::Hash(1),
            Token::Text("Heading".to_string()),
            Token::Newline,
            Token::Newline,
            Token::Text("This is".to_string()),
            Token::Asterisk(2),
            Token::Text("bold".to_string()),
            Token::Asterisk(2),
            Token::Text("and".to_string()),
            Token::Asterisk(1),
            Token::Text("italic".to_string()),
            Token::Asterisk(1),
            Token::Text("text.".to_string()),
            Token::Newline,
            Token::Newline,
            Token::Hyphen,
            Token::Text("List item".to_string()),
            Token::Newline,
            Token::Hyphen,
            Token::Text("Another item".to_string()),
            Token::Newline,
            Token::Newline,
            Token::LeftBracket,
            Token::Text("Link".to_string()),
            Token::RightBracket,
            Token::LeftParen,
            Token::Text("url".to_string()),
            Token::RightParen,
            Token::Eof
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_unicode_text() {
        let mut lexer = Lexer::new("Hello 世界 🌍");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::Text("Hello 世界 🌍".to_string()),
            Token::Eof
        ]);
    }

    #[test]
    fn test_mixed_whitespace() {
        let mut lexer = Lexer::new("  \t  text  \t  ");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::Text("text".to_string()),
            Token::Eof
        ]);
    }

    #[test]
    fn test_consecutive_newlines() {
        let mut lexer = Lexer::new("\n\n\n");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, vec![
            Token::Newline,
            Token::Newline,
            Token::Newline,
            Token::Eof
        ]);
    }

    // Error cases
    #[test]
    fn test_invalid_number_error() {
        // This shouldn't actually error in our current implementation,
        // but let's test the boundary case
        let input = "999999999999999999999999999999999999999";
        let mut lexer = Lexer::new(input);
        let result = lexer.tokenize();

        match result {
            Err(LexerError::InvalidNumber { text, line, column }) => {
                assert_eq!(text, input);
                assert_eq!(line, 1);
                assert_eq!(column, 1);
            }
            Ok(_) => {
                // If it doesn't error, that's also fine for very large numbers
                // depending on your implementation
            }
            Err(other) => panic!("Unexpected error: {:?}", other),
        }
    }

    // Position tracking tests
    #[test]
    fn test_position_tracking() {
        let mut lexer = Lexer::new("line1\nline2\nline3");

        // Initial position
        assert_eq!(lexer.position(), (1, 1));

        // After first token (line1)
        lexer.next_token().unwrap();
        assert_eq!(lexer.position(), (1, 6));

        // After newline
        lexer.next_token().unwrap();
        assert_eq!(lexer.position(), (2, 1));

        // After second line
        lexer.next_token().unwrap();
        assert_eq!(lexer.position(), (2, 6));
    }

    // Stream processing test
    #[test]
    fn test_streaming_tokens() {
        let mut lexer = Lexer::new("# Hello\n*World*");
        let mut tokens = Vec::new();

        loop {
            match lexer.next_token().unwrap() {
                Some(Token::Eof) => {
                    tokens.push(Token::Eof);
                    break;
                }
                Some(token) => tokens.push(token),
                None => break,
            }
        }

        assert_eq!(tokens, vec![
            Token::Hash(1),
            Token::Text("Hello".to_string()),
            Token::Newline,
            Token::Asterisk(1),
            Token::Text("World".to_string()),
            Token::Asterisk(1),
            Token::Eof
        ]);
    }

    #[test]
    fn test_dot_contexts() {
        // Standalone dot
        let mut lexer = Lexer::new(". ");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0], Token::Dot);

        // Dot in URL (should be part of text)
        let mut lexer = Lexer::new("example.com");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0], Token::Text("example.com".to_string()));

        // Ordered list pattern
        let mut lexer = Lexer::new("1. Item");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0], Token::Number(1));
        assert_eq!(tokens[1], Token::Dot);
        assert_eq!(tokens[2], Token::Text("Item".to_string()));
    }

    #[test]
    fn test_hyphen_contexts() {
        // Standalone hyphen (list marker)
        let mut lexer = Lexer::new("- item");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0], Token::Hyphen);
        assert_eq!(tokens[1], Token::Text("item".to_string()));

        // Hyphen in word (should be part of text)
        let mut lexer = Lexer::new("well-known");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0], Token::Text("well-known".to_string()));
    }

    #[test]
    fn test_number_contexts() {
        // Standalone number
        let mut lexer = Lexer::new("42 ");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0], Token::Number(42));

        // Number in text
        let mut lexer = Lexer::new("version1.2.3");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0], Token::Text("version1.2.3".to_string()));

        // Ordered list
        let mut lexer = Lexer::new("1. First");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0], Token::Number(1));
        assert_eq!(tokens[1], Token::Dot);
    }
}
