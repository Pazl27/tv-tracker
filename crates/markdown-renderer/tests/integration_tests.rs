use markdown_renderer::{parse_markdown, tokenize, parse, AstNode, ParseError, MarkdownError};

#[test]
fn test_complete_pipeline_simple() {
    let markdown = "# Hello World";
    let ast = parse_markdown(markdown).unwrap();
    
    match ast {
        AstNode::Document { children } => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                AstNode::Heading { level, content } => {
                    assert_eq!(*level, 1);
                    assert_eq!(content.len(), 1);
                    assert_eq!(content[0], AstNode::Text("Hello World".to_string()));
                },
                _ => panic!("Expected heading node"),
            }
        },
        _ => panic!("Expected document node"),
    }
}

#[test]
fn test_complete_pipeline_complex() {
    let markdown = r#"# Main Title

This is a paragraph with **bold** and *italic* text.

## Subsection

Here's a list:
- First item with [link](https://example.com)
- Second item

### Numbers

1. Ordered item one
2. Ordered item two

Final paragraph."#;

    let ast = parse_markdown(markdown).unwrap();
    
    match ast {
        AstNode::Document { children } => {
            assert_eq!(children.len(), 8); // title, paragraph, subsection, paragraph, list, subsubsection, ordered list, final paragraph
            
            // Check main title
            assert!(matches!(children[0], AstNode::Heading { level: 1, .. }));
            
            // Check first paragraph with formatting
            match &children[1] {
                AstNode::Paragraph { content } => {
                    assert!(content.len() > 3); // Should have text, bold, text, italic, text
                    // Find bold and italic elements
                    let has_bold = content.iter().any(|node| matches!(node, AstNode::Bold(_)));
                    let has_italic = content.iter().any(|node| matches!(node, AstNode::Italic(_)));
                    assert!(has_bold);
                    assert!(has_italic);
                },
                _ => panic!("Expected paragraph"),
            }
            
            // Check subsection
            assert!(matches!(children[2], AstNode::Heading { level: 2, .. }));
            
            // Find the unordered list (should be somewhere in the children)
            let unordered_list = children.iter().find(|child| {
                matches!(child, AstNode::List { ordered: false, .. })
            });
            assert!(unordered_list.is_some(), "Expected to find unordered list");
            
            // Find the ordered list (should be somewhere in the children)  
            let ordered_list = children.iter().find(|child| {
                matches!(child, AstNode::List { ordered: true, .. })
            });
            assert!(ordered_list.is_some(), "Expected to find ordered list");
            
            // Check the unordered list has the expected structure
            if let Some(AstNode::List { ordered: false, items }) = unordered_list {
                assert_eq!(items.len(), 2); // The parser groups consecutive list items together
                
                // Check first item has a link
                match &items[0] {
                    AstNode::ListItem { content } => {
                        let has_link = content.iter().any(|node| matches!(node, AstNode::Link { .. }));
                        assert!(has_link);
                    },
                    _ => panic!("Expected list item"),
                }
            }
            
            // Check the ordered list
            if let Some(AstNode::List { ordered: true, items }) = ordered_list {
                assert_eq!(items.len(), 2);
            }
        },
        _ => panic!("Expected document node"),
    }
}

#[test]
fn test_tokenize_then_parse() {
    let markdown = "## Test\n\nContent with *emphasis*.";
    
    // Step 1: Tokenize
    let tokens = tokenize(markdown).unwrap();
    assert!(!tokens.is_empty());
    
    // Step 2: Parse
    let ast = parse(tokens).unwrap();
    
    match ast {
        AstNode::Document { children } => {
            assert_eq!(children.len(), 2); // heading and paragraph
        },
        _ => panic!("Expected document node"),
    }
}

#[test]
fn test_error_propagation() {
    // Test with malformed markdown that should cause a parse error
    let markdown = "[unclosed link";
    
    match parse_markdown(markdown) {
        Err(MarkdownError::ParseError(ParseError::UnclosedDelimiter { .. })) => {
            // Expected error
        },
        Ok(_) => panic!("Expected parse error"),
        Err(e) => panic!("Unexpected error type: {:?}", e),
    }
}

#[test]
fn test_empty_input() {
    let ast = parse_markdown("").unwrap();
    
    match ast {
        AstNode::Document { children } => {
            assert!(children.is_empty());
        },
        _ => panic!("Expected empty document"),
    }
}

#[test]
fn test_whitespace_only() {
    let ast = parse_markdown("   \n  \n   ").unwrap();
    
    match ast {
        AstNode::Document { children } => {
            assert!(children.is_empty());
        },
        _ => panic!("Expected empty document"),
    }
}

#[test]
fn test_multiple_headings() {
    let markdown = r#"# H1
## H2
### H3
#### H4
##### H5
###### H6"#;
    
    let ast = parse_markdown(markdown).unwrap();
    
    match ast {
        AstNode::Document { children } => {
            assert_eq!(children.len(), 6);
            
            for (i, child) in children.iter().enumerate() {
                match child {
                    AstNode::Heading { level, .. } => {
                        assert_eq!(*level as usize, i + 1);
                    },
                    _ => panic!("Expected heading at position {}", i),
                }
            }
        },
        _ => panic!("Expected document node"),
    }
}

#[test]
fn test_nested_formatting() {
    let markdown = "Text with **bold and *italic inside*** more text.";
    let result = parse_markdown(markdown);
    
    match result {
        Ok(ast) => {
            match ast {
                AstNode::Document { children } => {
                    assert_eq!(children.len(), 1);
                    match &children[0] {
                        AstNode::Paragraph { content } => {
                            // Should have text, bold section, text
                            assert!(content.len() >= 3);
                            
                            // Find the bold section
                            let bold_found = content.iter().any(|node| {
                                matches!(node, AstNode::Bold(_))
                            });
                            assert!(bold_found);
                        },
                        _ => panic!("Expected paragraph"),
                    }
                },
                _ => panic!("Expected document node"),
            }
        },
        Err(MarkdownError::ParseError(ParseError::UnclosedDelimiter { .. })) => {
            // Expected for complex nested formatting that might not match properly
        },
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn test_mixed_list_types() {
    let markdown = r#"- Unordered item 1
- Unordered item 2

1. Ordered item 1
2. Ordered item 2

- Another unordered item"#;
    
    let ast = parse_markdown(markdown).unwrap();
    
    match ast {
        AstNode::Document { children } => {
            assert_eq!(children.len(), 3); // Two lists separated by content
            
            // First should be unordered
            match &children[0] {
                AstNode::List { ordered: false, items } => {
                    assert_eq!(items.len(), 2);
                },
                _ => panic!("Expected unordered list"),
            }
            
            // Second should be ordered
            match &children[1] {
                AstNode::List { ordered: true, items } => {
                    assert_eq!(items.len(), 2);
                },
                _ => panic!("Expected ordered list"),
            }
            
            // Third should be unordered again
            match &children[2] {
                AstNode::List { ordered: false, items } => {
                    assert_eq!(items.len(), 1);
                },
                _ => panic!("Expected unordered list"),
            }
        },
        _ => panic!("Expected document node"),
    }
}

#[test]
fn test_links_with_formatting() {
    let markdown = "Check out [**bold link**](https://example.com) and [*italic link*](https://test.com).";
    let ast = parse_markdown(markdown).unwrap();
    
    match ast {
        AstNode::Document { children } => {
            match &children[0] {
                AstNode::Paragraph { content } => {
                    // Should contain links
                    let link_count = content.iter().filter(|node| {
                        matches!(node, AstNode::Link { .. })
                    }).count();
                    assert_eq!(link_count, 2);
                },
                _ => panic!("Expected paragraph"),
            }
        },
        _ => panic!("Expected document node"),
    }
}

#[test]
fn test_consecutive_formatting() {
    let markdown = "**bold***italic*normal**bold2**";
    let result = parse_markdown(markdown);
    
    match result {
        Ok(ast) => {
            match ast {
                AstNode::Document { children } => {
                    match &children[0] {
                        AstNode::Paragraph { content } => {
                            // Should have multiple formatting elements
                            let has_bold = content.iter().any(|node| matches!(node, AstNode::Bold(_)));
                            let has_italic = content.iter().any(|node| matches!(node, AstNode::Italic(_)));
                            assert!(has_bold);
                            assert!(has_italic);
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
fn test_list_with_empty_items() {
    let markdown = r#"- First item
- 
- Third item"#;
    
    let ast = parse_markdown(markdown).unwrap();
    
    match ast {
        AstNode::Document { children } => {
            match &children[0] {
                AstNode::List { items, .. } => {
                    assert_eq!(items.len(), 3);
                    
                    // Check second item is empty
                    match &items[1] {
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
fn test_performance_large_document() {
    // Generate a large markdown document
    let mut markdown = String::new();
    for i in 1..=100 {
        markdown.push_str(&format!("# Heading {}\n\n", i));
        markdown.push_str(&format!("This is paragraph {} with **bold** and *italic* text.\n\n", i));
        markdown.push_str(&format!("- List item {} with [link](https://example{}.com)\n", i, i));
        markdown.push_str("- Another item\n\n");
    }
    
    let start = std::time::Instant::now();
    let ast = parse_markdown(&markdown).unwrap();
    let duration = start.elapsed();
    
    // Should complete within reasonable time (adjust as needed)
    assert!(duration.as_millis() < 1000, "Parsing took too long: {:?}", duration);
    
    // Verify structure
    match ast {
        AstNode::Document { children } => {
            // Should have many children (headings, paragraphs, lists)
            assert!(children.len() > 200);
        },
        _ => panic!("Expected document node"),
    }
}

#[test]
fn test_unicode_content() {
    let markdown = "# 🚀 Unicode Title\n\nParagraph with émojis 🎉 and àccénts.";
    let ast = parse_markdown(markdown).unwrap();
    
    match ast {
        AstNode::Document { children } => {
            assert_eq!(children.len(), 2);
            
            // Check title has unicode
            match &children[0] {
                AstNode::Heading { content, .. } => {
                    match &content[0] {
                        AstNode::Text(text) => {
                            assert!(text.contains("🚀"));
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
fn test_edge_case_empty_elements() {
    let markdown = r#"#

**

*

[]()

- 

1. "#;
    
    let result = parse_markdown(markdown);
    
    match result {
        Ok(ast) => {
            match ast {
                AstNode::Document { children } => {
                    // Should handle empty elements gracefully
                    assert!(!children.is_empty());
                },
                _ => panic!("Expected document node"),
            }
        },
        Err(MarkdownError::ParseError(_)) => {
            // Expected for malformed empty elements
        },
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}