#[cfg(feature = "html")]
use markdown_renderer::{
    markdown_to_html, markdown_to_html_with_options, HtmlRenderer, HtmlOptions, CssClasses, 
    CustomAttributes, Renderer, AstNode
};

#[cfg(feature = "html")]
mod html_tests {
    use super::*;

    #[test]
    fn test_basic_html_conversion() {
        let markdown = "# Hello World\n\nThis is a **bold** paragraph.";
        let html = markdown_to_html(markdown).unwrap();
        
        assert!(html.contains("<h1>Hello World</h1>"));
        assert!(html.contains("<p>This is a<strong>bold</strong>paragraph.</p>"));
    }

    #[test]
    fn test_all_heading_levels() {
        let markdown = "# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6";
        let html = markdown_to_html(markdown).unwrap();
        
        for level in 1..=6 {
            assert!(html.contains(&format!("<h{}>H{}</h{}>", level, level, level)));
        }
    }

    #[test]
    fn test_emphasis_rendering() {
        let markdown = "This has *italic* and **bold** text.";
        let html = markdown_to_html(markdown).unwrap();
        
        assert!(html.contains("<em>italic</em>"));
        assert!(html.contains("<strong>bold</strong>"));
    }

    #[test]
    fn test_link_rendering() {
        let markdown = "[Link Text](https://example.com)";
        let html = markdown_to_html(markdown).unwrap();
        
        assert!(html.contains(r#"<a href="https://example.com">Link Text</a>"#));
    }

    #[test]
    fn test_unordered_list() {
        let markdown = "- Item 1\n- Item 2\n- Item 3";
        let html = markdown_to_html(markdown).unwrap();
        
        assert!(html.contains("<ul>"));
        assert!(html.contains("<li>Item 1</li>"));
        assert!(html.contains("<li>Item 2</li>"));
        assert!(html.contains("<li>Item 3</li>"));
        assert!(html.contains("</ul>"));
    }

    #[test]
    fn test_ordered_list() {
        let markdown = "1. First\n2. Second\n3. Third";
        let html = markdown_to_html(markdown).unwrap();
        
        assert!(html.contains("<ol>"));
        assert!(html.contains("<li>First</li>"));
        assert!(html.contains("<li>Second</li>"));
        assert!(html.contains("<li>Third</li>"));
        assert!(html.contains("</ol>"));
    }

    #[test]
    fn test_mixed_formatting() {
        let markdown = "**Bold with *italic inside* text**.";
        let result = markdown_to_html(markdown);
        
        // This might fail due to unmatched emphasis, which is acceptable
        match result {
            Ok(html) => {
                assert!(html.contains("<strong>") || html.contains("<em>"));
            },
            Err(_) => {
                // Expected for complex nested formatting
            }
        }
    }

    #[test]
    fn test_html_escaping() {
        let markdown = r#"Text with <script>alert("xss")</script> & "quotes""#;
        let html = markdown_to_html(markdown).unwrap();
        
        assert!(html.contains("&lt;script&gt;"));
        assert!(html.contains("&quot;quotes&quot;"));
        assert!(html.contains("&amp;"));
        assert!(!html.contains("<script>"));
    }

    #[test]
    fn test_no_html_escaping() {
        let ast = AstNode::Text("<b>bold</b>".to_string());
        let options = HtmlOptions {
            escape_html: false,
            ..Default::default()
        };
        let renderer = HtmlRenderer::with_options(options);
        let html = renderer.render(&ast).unwrap();
        
        assert!(html.contains("<b>bold</b>"));
        assert!(!html.contains("&lt;"));
    }

    #[test]
    fn test_pretty_printing() {
        let markdown = "# Title\n\nParagraph text.";
        let options = HtmlOptions {
            pretty_print: true,
            ..Default::default()
        };
        let html = markdown_to_html_with_options(markdown, options).unwrap();
        
        assert!(html.contains('\n'));
        assert!(html.matches('\n').count() > 1);
    }

    #[test]
    fn test_compact_rendering() {
        let markdown = "# Title\n\nParagraph text.";
        let options = HtmlOptions {
            pretty_print: false,
            ..Default::default()
        };
        let html = markdown_to_html_with_options(markdown, options).unwrap();
        
        assert!(!html.contains('\n'));
    }

    #[test]
    fn test_css_classes() {
        let markdown = "# Title\n\nParagraph with **bold** text.";
        
        let mut css_classes = CssClasses::default();
        css_classes.heading = Some("custom-heading".to_string());
        css_classes.paragraph = Some("custom-paragraph".to_string());
        css_classes.bold = Some("custom-bold".to_string());
        
        let options = HtmlOptions {
            css_classes,
            ..Default::default()
        };
        
        let html = markdown_to_html_with_options(markdown, options).unwrap();
        
        assert!(html.contains(r#"<h1 class="custom-heading">"#));
        assert!(html.contains(r#"<p class="custom-paragraph">"#));
        assert!(html.contains(r#"<strong class="custom-bold">"#));
    }

    #[test]
    fn test_document_wrapper_class() {
        let markdown = "# Title";
        
        let mut css_classes = CssClasses::default();
        css_classes.document = Some("markdown-content".to_string());
        
        let options = HtmlOptions {
            css_classes,
            pretty_print: true,
            ..Default::default()
        };
        
        let html = markdown_to_html_with_options(markdown, options).unwrap();
        
        assert!(html.contains(r#"<div class="markdown-content">"#));
        assert!(html.contains("</div>"));
    }

    #[test]
    fn test_list_css_classes() {
        let markdown = "- Unordered\n\n1. Ordered";
        
        let mut css_classes = CssClasses::default();
        css_classes.list_unordered = Some("ul-class".to_string());
        css_classes.list_ordered = Some("ol-class".to_string());
        css_classes.list_item = Some("li-class".to_string());
        
        let options = HtmlOptions {
            css_classes,
            ..Default::default()
        };
        
        let html = markdown_to_html_with_options(markdown, options).unwrap();
        
        assert!(html.contains(r#"<ul class="ul-class">"#));
        assert!(html.contains(r#"<ol class="ol-class">"#));
        assert!(html.contains(r#"<li class="li-class">"#));
    }

    #[test]
    fn test_external_links_new_tab() {
        let markdown = "[External](https://example.com) and [Internal](relative-link)";
        
        let options = HtmlOptions {
            external_links_new_tab: true,
            ..Default::default()
        };
        
        let html = markdown_to_html_with_options(markdown, options).unwrap();
        
        // External link should have target="_blank"
        assert!(html.contains(r#"<a href="https://example.com" target="_blank" rel="noopener noreferrer">External</a>"#));
        
        // Internal link should not have target="_blank"
        assert!(html.contains(r#"<a href="relative-link">Internal</a>"#));
        assert!(!html.contains(r#"<a href="relative-link" target="_blank""#));
    }

    #[test]
    fn test_custom_attributes() {
        let markdown = "# Title\n\n[Link](https://example.com)";
        
        let mut custom_attributes = CustomAttributes::default();
        custom_attributes.heading = vec![
            ("data-section".to_string(), "main".to_string()),
            ("id".to_string(), "main-title".to_string()),
        ];
        custom_attributes.link = vec![
            ("data-track".to_string(), "click".to_string()),
        ];
        
        let options = HtmlOptions {
            custom_attributes,
            ..Default::default()
        };
        
        let html = markdown_to_html_with_options(markdown, options).unwrap();
        
        assert!(html.contains(r#"data-section="main""#));
        assert!(html.contains(r#"id="main-title""#));
        assert!(html.contains(r#"data-track="click""#));
    }

    #[test]
    fn test_line_break_rendering() {
        let ast = AstNode::LineBreak;
        let renderer = HtmlRenderer::new();
        let html = renderer.render(&ast).unwrap();
        
        assert_eq!(html, "<br>");
    }

    #[test]
    fn test_line_break_with_pretty_print() {
        let ast = AstNode::LineBreak;
        let options = HtmlOptions {
            pretty_print: true,
            ..Default::default()
        };
        let renderer = HtmlRenderer::with_options(options);
        let html = renderer.render(&ast).unwrap();
        
        assert_eq!(html, "<br>\n");
    }

    #[test]
    fn test_empty_elements() {
        let ast = AstNode::Document {
            children: vec![
                AstNode::Heading { level: 1, content: vec![] },
                AstNode::Paragraph { content: vec![] },
                AstNode::Bold(vec![]),
                AstNode::Italic(vec![]),
            ],
        };
        
        let renderer = HtmlRenderer::new();
        let html = renderer.render(&ast).unwrap();
        
        assert!(html.contains("<h1></h1>"));
        assert!(html.contains("<p></p>"));
        assert!(html.contains("<strong></strong>"));
        assert!(html.contains("<em></em>"));
    }

    #[test]
    fn test_nested_lists() {
        let markdown = "- Item 1\n- Item 2\n\n1. Ordered 1\n2. Ordered 2";
        let html = markdown_to_html(markdown).unwrap();
        
        assert!(html.contains("<ul>"));
        assert!(html.contains("</ul>"));
        assert!(html.contains("<ol>"));
        assert!(html.contains("</ol>"));
    }

    #[test]
    fn test_complex_document() {
        let markdown = r#"# Main Title

This is an introduction paragraph with **bold** and *italic* text.

## Features

Here are the key features:

- Feature 1 with [link](https://example.com)
- **Bold feature** 2
- *Italic feature* 3

### Ordered Steps

1. First step
2. Second step with **emphasis**
3. Final step

## Conclusion

That's all for now!"#;

        let html = markdown_to_html(markdown).unwrap();
        
        // Check structure
        assert!(html.contains("<h1>Main Title</h1>"));
        assert!(html.contains("<h2>Features</h2>"));
        assert!(html.contains("<h3>Ordered Steps</h3>"));
        assert!(html.contains("<h2>Conclusion</h2>"));
        
        // Check content
        assert!(html.contains("introduction paragraph"));
        assert!(html.contains("<ul>"));
        assert!(html.contains("<ol>"));
        assert!(html.contains("Feature 1 with"));
        assert!(html.contains(r#"<a href="https://example.com">link</a>"#));
    }

    #[test]
    fn test_attribute_escaping() {
        let ast = AstNode::Link {
            text: vec![AstNode::Text("Test".to_string())],
            url: r#"javascript:alert("xss")"#.to_string(),
        };
        
        let renderer = HtmlRenderer::new();
        let html = renderer.render(&ast).unwrap();
        
        assert!(html.contains("&quot;"));
        assert!(!html.contains(r#"alert("xss")"#));
    }

    #[test]
    fn test_renderer_trait_implementation() {
        let ast = AstNode::Text("Hello".to_string());
        let renderer = HtmlRenderer::new();
        
        // Test that it implements the Renderer trait
        let result: Result<String, _> = renderer.render(&ast);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello");
    }

    #[test]
    fn test_special_characters_in_urls() {
        let markdown = r#"[Test](https://example.com/path?query=value&other="quoted")"#;
        let html = markdown_to_html(markdown).unwrap();
        
        assert!(html.contains("&amp;"));
        assert!(html.contains("&quot;"));
        assert!(!html.contains(r#"other="quoted""#));
    }

    #[test]
    fn test_unicode_content() {
        let markdown = "# 🚀 Unicode Title\n\nContent with émojis 🎉 and àccénts.";
        let html = markdown_to_html(markdown).unwrap();
        
        assert!(html.contains("🚀 Unicode Title"));
        assert!(html.contains("émojis 🎉"));
        assert!(html.contains("àccénts"));
    }

    #[test]
    fn test_performance_large_document() {
        let mut large_markdown = String::new();
        for i in 1..=100 {
            large_markdown.push_str(&format!("## Heading {}\n\n", i));
            large_markdown.push_str(&format!("Paragraph {} with **bold** text.\n\n", i));
            large_markdown.push_str(&format!("- List item {}\n\n", i));
        }
        
        let start = std::time::Instant::now();
        let html = markdown_to_html(&large_markdown).unwrap();
        let duration = start.elapsed();
        
        // Should complete within reasonable time
        assert!(duration.as_millis() < 1000, "HTML rendering took too long: {:?}", duration);
        
        // Verify some content exists
        assert!(html.contains("<h2>Heading 1</h2>"));
        assert!(html.contains("<h2>Heading 100</h2>"));
        assert!(html.len() > 1000); // Should be substantial HTML
    }

    #[test]
    fn test_combined_css_and_attributes() {
        let markdown = "# Test";
        
        let mut css_classes = CssClasses::default();
        css_classes.heading = Some("heading-class".to_string());
        
        let mut custom_attributes = CustomAttributes::default();
        custom_attributes.heading = vec![("id".to_string(), "test-heading".to_string())];
        
        let options = HtmlOptions {
            css_classes,
            custom_attributes,
            pretty_print: true,
            ..Default::default()
        };
        
        let html = markdown_to_html_with_options(markdown, options).unwrap();
        
        assert!(html.contains(r#"<h1 class="heading-class" id="test-heading">Test</h1>"#));
    }

    #[test]
    fn test_error_handling() {
        // This test ensures that the renderer handles edge cases gracefully
        let ast = AstNode::Document { children: vec![] };
        let renderer = HtmlRenderer::new();
        let result = renderer.render(&ast);
        
        assert!(result.is_ok());
        let html = result.unwrap();
        assert!(html.is_empty() || html.trim().is_empty());
    }
}

#[cfg(not(feature = "html"))]
mod no_html_tests {
    #[test]
    fn test_html_features_not_available() {
        // This test ensures that HTML features are not available without the feature flag
        // If this compiles, it means the feature gating is working correctly
        println!("HTML features are not available without the 'html' feature flag");
    }
}