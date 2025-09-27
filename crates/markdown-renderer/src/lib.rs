pub mod error;
pub mod parser;
pub mod renderer;

mod lexer;

// Re-export commonly used items
pub use error::{LexerError, ParseError, MarkdownError};
pub use lexer::{Lexer, Token, tokenize};
pub use parser::{Parser, AstNode, parse};

// Re-export renderer items when feature is enabled
#[cfg(feature = "html")]
pub use renderer::{render_html, render_html_with_options, HtmlRenderer, Renderer};

#[cfg(feature = "html")]
pub use renderer::html::{HtmlOptions, CssClasses, CustomAttributes};

/// Parse markdown text into an AST
pub fn parse_markdown(input: &str) -> Result<AstNode, MarkdownError> {
    let tokens = tokenize(input).map_err(ParseError::LexerError)?;
    let ast = parse(tokens)?;
    Ok(ast)
}

/// Convert markdown text directly to HTML (only available with "html" feature)
#[cfg(feature = "html")]
pub fn markdown_to_html(input: &str) -> Result<String, MarkdownError> {
    let ast = parse_markdown(input)?;
    render_html(&ast)
}

/// Convert markdown text to HTML with custom options (only available with "html" feature)
#[cfg(feature = "html")]
pub fn markdown_to_html_with_options(input: &str, options: renderer::html::HtmlOptions) -> Result<String, MarkdownError> {
    let ast = parse_markdown(input)?;
    render_html_with_options(&ast, options)
}

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

    #[test]
    fn test_parse_markdown() {
        let ast = parse_markdown("# Hello **World**").unwrap();
        
        if let AstNode::Document { children } = ast {
            assert_eq!(children.len(), 1);
            if let AstNode::Heading { level, content } = &children[0] {
                assert_eq!(*level, 1);
                // The content should be: "Hello ", Bold("World")
                assert_eq!(content.len(), 2);
            } else {
                panic!("Expected heading node");
            }
        } else {
            panic!("Expected document node");
        }
    }

    #[test]
    fn test_parse_heading() {
        let ast = parse_markdown("## Second Level").unwrap();
        
        if let AstNode::Document { children } = ast {
            assert_eq!(children.len(), 1);
            if let AstNode::Heading { level, content } = &children[0] {
                assert_eq!(*level, 2);
                assert_eq!(content.len(), 1);
                assert_eq!(content[0], AstNode::Text("Second Level".to_string()));
            } else {
                panic!("Expected heading node");
            }
        }

        #[cfg(feature = "html")]
        #[test]
        fn test_html_rendering() {
            let ast = parse_markdown("# Hello **World**").unwrap();
            let html = render_html(&ast).unwrap();
        
            assert!(html.contains("<h1>Hello <strong>World</strong></h1>"));
        }

        #[cfg(feature = "html")]
        #[test]
        fn test_markdown_to_html() {
            let html = markdown_to_html("## Test\n\nParagraph with *italic* text.").unwrap();
        
            assert!(html.contains("<h2>Test</h2>"));
            assert!(html.contains("<p>Paragraph with <em>italic</em> text.</p>"));
        }
    }

    #[test]
    fn test_parse_paragraph() {
        let ast = parse_markdown("This is a simple paragraph.").unwrap();
        
        if let AstNode::Document { children } = ast {
            assert_eq!(children.len(), 1);
            if let AstNode::Paragraph { content } = &children[0] {
                assert_eq!(content.len(), 1);
                assert_eq!(content[0], AstNode::Text("This is a simple paragraph.".to_string()));
            } else {
                panic!("Expected paragraph node");
            }
        }
    }

    #[test]
    fn test_parse_emphasis() {
        let ast = parse_markdown("*italic* and **bold** text").unwrap();
        
        if let AstNode::Document { children } = ast {
            assert_eq!(children.len(), 1);
            if let AstNode::Paragraph { content } = &children[0] {
                assert_eq!(content.len(), 4); // "italic", " and ", "bold", " text"
                
                // Check italic
                if let AstNode::Italic(italic_content) = &content[0] {
                    assert_eq!(italic_content.len(), 1);
                    assert_eq!(italic_content[0], AstNode::Text("italic".to_string()));
                } else {
                    panic!("Expected italic node");
                }
                
                // Check bold
                if let AstNode::Bold(bold_content) = &content[2] {
                    assert_eq!(bold_content.len(), 1);
                    assert_eq!(bold_content[0], AstNode::Text("bold".to_string()));
                } else {
                    panic!("Expected bold node");
                }
            }
        }
    }

    #[test]
    fn test_parse_unordered_list() {
        let ast = parse_markdown("- First item\n- Second item").unwrap();
        
        if let AstNode::Document { children } = ast {
            assert_eq!(children.len(), 1);
            if let AstNode::List { ordered, items } = &children[0] {
                assert_eq!(*ordered, false);
                assert_eq!(items.len(), 2);
                
                // Check first item
                if let AstNode::ListItem { content } = &items[0] {
                    assert_eq!(content.len(), 1);
                    assert_eq!(content[0], AstNode::Text("First item".to_string()));
                }
                
                // Check second item
                if let AstNode::ListItem { content } = &items[1] {
                    assert_eq!(content.len(), 1);
                    assert_eq!(content[0], AstNode::Text("Second item".to_string()));
                }
            } else {
                panic!("Expected unordered list node");
            }
        }
    }

    #[test]
    fn test_parse_ordered_list() {
        let ast = parse_markdown("1. First item\n2. Second item").unwrap();
        
        if let AstNode::Document { children } = ast {
            assert_eq!(children.len(), 1);
            if let AstNode::List { ordered, items } = &children[0] {
                assert_eq!(*ordered, true);
                assert_eq!(items.len(), 2);
                
                // Check first item
                if let AstNode::ListItem { content } = &items[0] {
                    assert_eq!(content.len(), 1);
                    assert_eq!(content[0], AstNode::Text("First item".to_string()));
                }
                
                // Check second item
                if let AstNode::ListItem { content } = &items[1] {
                    assert_eq!(content.len(), 1);
                    assert_eq!(content[0], AstNode::Text("Second item".to_string()));
                }
            } else {
                panic!("Expected ordered list node");
            }
        }
    }

    #[test]
    fn test_parse_link() {
        let ast = parse_markdown("[OpenAI](https://openai.com)").unwrap();
        
        if let AstNode::Document { children } = ast {
            assert_eq!(children.len(), 1);
            if let AstNode::Paragraph { content } = &children[0] {
                assert_eq!(content.len(), 1);
                if let AstNode::Link { text, url } = &content[0] {
                    assert_eq!(text.len(), 1);
                    assert_eq!(text[0], AstNode::Text("OpenAI".to_string()));
                    assert_eq!(url, "https://openai.com");
                } else {
                    panic!("Expected link node");
                }
            }
        }
    }

    #[test]
    fn test_parse_complex_markdown() {
        let markdown = "# Title\n\nThis is a **bold** paragraph with *italic* text.\n\n- Item 1\n- Item 2";
        let ast = parse_markdown(markdown).unwrap();
        
        if let AstNode::Document { children } = ast {
            assert_eq!(children.len(), 3); // heading, paragraph, list
            
            // Check heading
            if let AstNode::Heading { level, .. } = &children[0] {
                assert_eq!(*level, 1);
            } else {
                panic!("Expected heading");
            }
            
            // Check paragraph
            if let AstNode::Paragraph { content } = &children[1] {
                assert!(content.len() > 1); // Should have mixed content
            } else {
                panic!("Expected paragraph");
            }
            
            // Check list
            if let AstNode::List { ordered, items } = &children[2] {
                assert_eq!(*ordered, false);
                assert_eq!(items.len(), 2);
            } else {
                panic!("Expected list");
            }
        }
    }

    #[test]
    fn test_empty_document() {
        let ast = parse_markdown("").unwrap();
        
        if let AstNode::Document { children } = ast {
            assert_eq!(children.len(), 0);
        } else {
            panic!("Expected empty document");
        }
    }

    #[test]
    fn test_multiple_newlines() {
        let ast = parse_markdown("# Title\n\n\nParagraph").unwrap();
        
        if let AstNode::Document { children } = ast {
            assert_eq!(children.len(), 2); // Should skip empty lines
            
            if let AstNode::Heading { .. } = &children[0] {
                // OK
            } else {
                panic!("Expected heading");
            }
            
            if let AstNode::Paragraph { .. } = &children[1] {
                // OK
            } else {
                panic!("Expected paragraph");
            }
        }
    }
}
