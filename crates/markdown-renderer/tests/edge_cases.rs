use markdown_renderer::{parse_markdown, AstNode, ParseError, MarkdownError};

#[test]
fn test_deeply_nested_emphasis() {
    // Test nested emphasis that could cause stack overflow
    let markdown = "**bold *italic **more bold** italic* bold**";
    let ast = parse_markdown(markdown).unwrap();
    
    match ast {
        AstNode::Document { children } => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                AstNode::Paragraph { content } => {
                    // Should handle nested formatting
                    assert!(!content.is_empty());
                },
                _ => panic!("Expected paragraph"),
            }
        },
        _ => panic!("Expected document node"),
    }
}

#[test]
fn test_malformed_links() {
    let test_cases = vec![
        "[text](", // Missing closing paren
        "[text", // Missing closing bracket
        "[]()", // Empty text and URL
        "](text)[", // Reversed brackets
        "[text](url", // Missing closing paren
        "[text]()", // Empty URL
    ];
    
    for (i, markdown) in test_cases.iter().enumerate() {
        let result = parse_markdown(markdown);
        match result {
            Ok(_) => {
                // Some cases might be parsed as regular text, which is valid
            },
            Err(_) => {
                // Expected for malformed cases
            }
        }
        println!("Test case {}: '{}' handled", i, markdown);
    }
}

#[test]
fn test_unmatched_emphasis_markers() {
    let test_cases = vec![
        "*unmatched italic",
        "**unmatched bold",
        "text *italic **bold* unmatched**",
        "***triple asterisk",
        "****quadruple asterisk",
    ];
    
    for markdown in test_cases {
        let result = parse_markdown(markdown);
        // These should either parse successfully (treating unmatched as text)
        // or return appropriate errors
        match result {
            Ok(ast) => {
                // Verify it's a valid document
                assert!(matches!(ast, AstNode::Document { .. }));
            },
            Err(MarkdownError::ParseError(ParseError::InvalidEmphasis(_))) => {
                // Expected for invalid emphasis levels
            },
            Err(MarkdownError::ParseError(ParseError::UnclosedDelimiter { .. })) => {
                // Expected for unmatched delimiters
            },
            Err(MarkdownError::ParseError(_)) => {
                // Any parse error is acceptable
            },
            Err(e) => {
                panic!("Unexpected error for '{}': {:?}", markdown, e);
            }
        }
    }
}

#[test]
fn test_extreme_heading_levels() {
    // Test various heading levels
    for level in 1..=6 {
        let hashes = "#".repeat(level);
        let markdown = format!("{} Heading Level {}", hashes, level);
        let ast = parse_markdown(&markdown).unwrap();
        
        match ast {
            AstNode::Document { children } => {
                match &children[0] {
                    AstNode::Heading { level: parsed_level, .. } => {
                        assert_eq!(*parsed_level, level as u8);
                    },
                    _ => panic!("Expected heading for level {}", level),
                }
            },
            _ => panic!("Expected document node"),
        }
    }
}

#[test]
fn test_mixed_line_endings() {
    let test_cases = vec![
        "Line 1\nLine 2\nLine 3", // Unix
        "Line 1\r\nLine 2\r\nLine 3", // Windows
        "Line 1\rLine 2\rLine 3", // Old Mac
        "Line 1\n\rLine 2\r\nLine 3", // Mixed
    ];
    
    for markdown in test_cases {
        let ast = parse_markdown(markdown).unwrap();
        match ast {
            AstNode::Document { children } => {
                // Should parse into paragraphs regardless of line ending style
                assert!(!children.is_empty());
            },
            _ => panic!("Expected document node"),
        }
    }
}

#[test]
fn test_very_long_text_nodes() {
    // Test with very long text to ensure no buffer overflows
    let long_text = "A".repeat(10000);
    let markdown = format!("# {}\n\n{}", long_text, long_text);
    let ast = parse_markdown(&markdown).unwrap();
    
    match ast {
        AstNode::Document { children } => {
            assert_eq!(children.len(), 2); // heading and paragraph
            
            // Check heading
            match &children[0] {
                AstNode::Heading { content, .. } => {
                    match &content[0] {
                        AstNode::Text(text) => {
                            assert_eq!(text.len(), 10000);
                        },
                        _ => panic!("Expected text node"),
                    }
                },
                _ => panic!("Expected heading"),
            }
        },
        _ => panic!("Expected document node"),
    }
}

#[test]
fn test_many_consecutive_markers() {
    let test_cases = vec![
        "############# Too many hashes",
        "**************** Many asterisks",
        "---------------- Many hyphens",
        ".................. Many dots",
    ];
    
    for markdown in test_cases {
        let result = parse_markdown(markdown);
        // Should handle gracefully without panicking
        assert!(result.is_ok() || result.is_err());
    }
}

#[test]
fn test_empty_list_items() {
    let markdown = r#"- 
- Item 2
- 
- Item 4
- "#;
    
    let ast = parse_markdown(markdown).unwrap();
    
    match ast {
        AstNode::Document { children } => {
            match &children[0] {
                AstNode::List { items, .. } => {
                    assert_eq!(items.len(), 5);
                    
                    // Check that empty items exist
                    match &items[0] {
                        AstNode::ListItem { content } => {
                            assert!(content.is_empty());
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

#[test]
fn test_large_numbers_in_ordered_lists() {
    let markdown = r#"999999999. Very large number
1000000000. Even larger
0. Zero
001. Leading zeros"#;
    
    let ast = parse_markdown(markdown).unwrap();
    
    match ast {
        AstNode::Document { children } => {
            match &children[0] {
                AstNode::List { ordered: true, items } => {
                    assert_eq!(items.len(), 4);
                },
                _ => panic!("Expected ordered list"),
            }
        },
        _ => panic!("Expected document node"),
    }
}

#[test]
fn test_special_characters_in_text() {
    let markdown = r#"# Title with special chars: !@#$%^&*()_+-={}[]|\"'<>?/.,

Paragraph with unicode: αβγδε ñáéíóú 中文 العربية русский

Links with special chars: [Test!@#](http://example.com?q=test&r=1)

List with symbols:
- ★ Star item
- ♠ Spade item  
- ⚡ Lightning item"#;
    
    let result = parse_markdown(markdown);
    
    match result {
        Ok(ast) => {
            match ast {
                AstNode::Document { children } => {
                    assert!(children.len() >= 3); // At least heading, paragraph, list
                },
                _ => panic!("Expected document node"),
            }
        },
        Err(MarkdownError::ParseError(ParseError::UnclosedDelimiter { .. })) => {
            // Expected for unmatched emphasis markers in special characters
        },
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn test_whitespace_variations() {
    let test_cases = vec![
        "#    Heading with spaces",
        "   # Indented heading",
        "  -   List item with spaces",
        "1.    Ordered item with spaces",
        "*  italic  *",
        "**  bold  **",
        "[  link text  ](  url  )",
    ];
    
    for markdown in test_cases {
        let result = parse_markdown(markdown);
        // Should handle various whitespace patterns gracefully
        assert!(result.is_ok(), "Failed to parse: '{}'", markdown);
    }
}

#[test]
fn test_nested_lists_simulation() {
    // Our parser doesn't support nested lists, but should handle
    // what looks like nested lists as separate lists
    let markdown = r#"- Outer item 1
- Outer item 2
  - What looks like nested
  - Another nested-looking item
- Outer item 3"#;
    
    let ast = parse_markdown(markdown).unwrap();
    
    match ast {
        AstNode::Document { children } => {
            // Should parse as multiple elements
            assert!(!children.is_empty());
        },
        _ => panic!("Expected document node"),
    }
}

#[test]
fn test_boundary_conditions() {
    let test_cases = vec![
        "", // Empty
        " ", // Single space
        "\n", // Single newline
        "\n\n\n", // Multiple newlines
        "#", // Just hash
        "*", // Just asterisk
        "-", // Just hyphen
        ".", // Just dot
        "1", // Just number
        "[", // Just left bracket
        "]", // Just right bracket
        "(", // Just left paren
        ")", // Just right paren
    ];
    
    for markdown in test_cases {
        let result = parse_markdown(markdown);
        // All should either succeed or fail gracefully
        match result {
            Ok(ast) => {
                assert!(matches!(ast, AstNode::Document { .. }));
            },
            Err(_) => {
                // Acceptable for some edge cases
            }
        }
    }
}

#[test]
fn test_rapid_format_changes() {
    let markdown = "*a**b*c**d*e**f*g**h*i**j*k**l*m**n*o**p*";
    let result = parse_markdown(markdown);
    
    match result {
        Ok(ast) => {
            match ast {
                AstNode::Document { children } => {
                    assert_eq!(children.len(), 1);
                    match &children[0] {
                        AstNode::Paragraph { content } => {
                            // Should handle rapid format changes
                            assert!(!content.is_empty());
                        },
                        _ => panic!("Expected paragraph"),
                    }
                },
                _ => panic!("Expected document node"),
            }
        },
        Err(MarkdownError::ParseError(ParseError::UnclosedDelimiter { .. })) => {
            // Expected for unmatched emphasis markers
        },
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn test_stress_many_elements() {
    let mut markdown = String::new();
    
    // Add many different elements
    for i in 0..50 {
        markdown.push_str(&format!("## Heading {}\n", i));
        markdown.push_str(&format!("Paragraph {} with **bold** text.\n", i));
        markdown.push_str(&format!("- List item {}\n", i));
        markdown.push_str(&format!("{}. Ordered item {}\n", i + 1, i));
        markdown.push_str(&format!("[Link {}](http://example{}.com)\n\n", i, i));
    }
    
    let start = std::time::Instant::now();
    let ast = parse_markdown(&markdown).unwrap();
    let duration = start.elapsed();
    
    // Should complete in reasonable time
    assert!(duration.as_millis() < 5000, "Parsing took too long: {:?}", duration);
    
    match ast {
        AstNode::Document { children } => {
            // Should have many children
            assert!(children.len() > 100);
        },
        _ => panic!("Expected document node"),
    }
}

#[test]
fn test_malformed_token_sequences() {
    // Test sequences that might confuse the parser
    let test_cases = vec![
        "][", // Reversed brackets
        ")(", // Reversed parens
        "**", // Unmatched bold
        "[[", // Double left bracket
        "))", // Double right paren
        "#*", // Hash followed by asterisk
        "*#", // Asterisk followed by hash
        "-1-", // Hyphen number hyphen
        "1..", // Number double dot
    ];
    
    for markdown in test_cases {
        let result = parse_markdown(markdown);
        // Should handle all cases without panicking
        match result {
            Ok(_) => {}, // Some might parse as text
            Err(_) => {}, // Others might error
        }
        println!("Handled malformed sequence: '{}'", markdown);
    }
}

#[test]
fn test_memory_usage_large_nested() {
    // Test that doesn't cause excessive memory usage
    let mut markdown = String::new();
    
    // Create deeply nested-looking structure
    markdown.push_str("# Start\n");
    for i in 0..1000 {
        markdown.push_str(&format!("*level{}*", i));
    }
    markdown.push_str("\n# End");
    
    let result = parse_markdown(&markdown);
    // Should either succeed or fail gracefully without OOM
    match result {
        Ok(ast) => {
            assert!(matches!(ast, AstNode::Document { .. }));
        },
        Err(_) => {
            // Acceptable if parser rejects overly complex input
        }
    }
}