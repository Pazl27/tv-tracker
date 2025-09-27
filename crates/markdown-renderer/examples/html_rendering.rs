use markdown_renderer::{parse_markdown, render_html, render_html_with_options, HtmlOptions, CssClasses, CustomAttributes};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example markdown content
    let markdown = r#"# HTML Rendering Demo

This is a **comprehensive** demonstration of the *HTML renderer* with various [features](https://example.com).

## Basic Features

Here are some basic markdown elements:

- **Bold text** formatting
- *Italic text* styling  
- Links like [Rust](https://rust-lang.org)
- Line breaks and paragraphs

### Ordered Lists

1. First ordered item
2. Second ordered item with **bold** text
3. Third item with [embedded link](https://github.com)

## Advanced Content

Mix of **bold** and *italic* formatting in the same paragraph, plus some `code-like` text.

- Unordered item 1
- Unordered item 2

Final paragraph with special characters: <script>alert('test')</script> & "quotes"."#;

    println!("=== Original Markdown ===");
    println!("{}", markdown);
    println!();

    // Parse the markdown
    let ast = parse_markdown(markdown)?;

    println!("=== Basic HTML Rendering ===");
    let basic_html = render_html(&ast)?;
    println!("{}", basic_html);
    println!();

    println!("=== Pretty-Printed HTML ===");
    let pretty_options = HtmlOptions {
        pretty_print: true,
        ..Default::default()
    };
    let pretty_html = render_html_with_options(&ast, pretty_options)?;
    println!("{}", pretty_html);
    println!();

    println!("=== HTML with CSS Classes ===");
    let mut css_classes = CssClasses::default();
    css_classes.document = Some("markdown-content".to_string());
    css_classes.heading = Some("heading".to_string());
    css_classes.paragraph = Some("paragraph".to_string());
    css_classes.list_ordered = Some("ordered-list".to_string());
    css_classes.list_unordered = Some("unordered-list".to_string());
    css_classes.list_item = Some("list-item".to_string());
    css_classes.bold = Some("bold".to_string());
    css_classes.italic = Some("italic".to_string());
    css_classes.link = Some("link".to_string());

    let css_options = HtmlOptions {
        pretty_print: true,
        css_classes,
        ..Default::default()
    };
    let css_html = render_html_with_options(&ast, css_options)?;
    println!("{}", css_html);
    println!();

    println!("=== HTML with External Links in New Tab ===");
    let external_links_options = HtmlOptions {
        external_links_new_tab: true,
        pretty_print: true,
        ..Default::default()
    };
    let external_html = render_html_with_options(&ast, external_links_options)?;
    println!("{}", external_html);
    println!();

    println!("=== HTML with Custom Attributes ===");
    let mut custom_attributes = CustomAttributes::default();
    custom_attributes.document = vec![
        ("data-theme".to_string(), "dark".to_string()),
        ("role".to_string(), "main".to_string()),
    ];
    custom_attributes.heading = vec![
        ("data-level".to_string(), "auto".to_string()),
    ];
    custom_attributes.link = vec![
        ("data-track".to_string(), "click".to_string()),
    ];

    let mut css_classes_custom = CssClasses::default();
    css_classes_custom.document = Some("markdown-document".to_string());

    let custom_options = HtmlOptions {
        pretty_print: true,
        css_classes: css_classes_custom,
        custom_attributes,
        external_links_new_tab: true,
        ..Default::default()
    };
    let custom_html = render_html_with_options(&ast, custom_options)?;
    println!("{}", custom_html);
    println!();

    println!("=== HTML without Escaping (Dangerous!) ===");
    let no_escape_options = HtmlOptions {
        escape_html: false,
        pretty_print: true,
        ..Default::default()
    };
    let no_escape_html = render_html_with_options(&ast, no_escape_options)?;
    println!("{}", no_escape_html);
    println!();

    // Demonstrate individual component rendering
    println!("=== Individual Component Examples ===");
    
    // Just a heading
    let heading_ast = parse_markdown("### Component Example")?;
    let heading_html = render_html(&heading_ast)?;
    println!("Heading only: {}", heading_html);

    // Just a list
    let list_ast = parse_markdown("- Item A\n- Item B\n- Item C")?;
    let list_html = render_html(&list_ast)?;
    println!("List only: {}", list_html);

    // Complex inline formatting
    let inline_ast = parse_markdown("**Bold** with *italic* and [link](https://test.com) combined.")?;
    let inline_html = render_html(&inline_ast)?;
    println!("Inline formatting: {}", inline_html);

    println!("\n=== Performance Test ===");
    let large_markdown = generate_large_markdown();
    let start = std::time::Instant::now();
    let large_ast = parse_markdown(&large_markdown)?;
    let parse_time = start.elapsed();

    let start = std::time::Instant::now();
    let _large_html = render_html(&large_ast)?;
    let render_time = start.elapsed();

    println!("Parsed {} characters in {:?}", large_markdown.len(), parse_time);
    println!("Rendered to HTML in {:?}", render_time);
    println!("Total time: {:?}", parse_time + render_time);

    Ok(())
}

fn generate_large_markdown() -> String {
    let mut markdown = String::new();
    
    markdown.push_str("# Large Document Performance Test\n\n");
    
    for section in 1..=10 {
        markdown.push_str(&format!("## Section {}\n\n", section));
        markdown.push_str(&format!("This is section {} with **bold** and *italic* text.\n\n", section));
        
        markdown.push_str("### Subsection Features\n\n");
        markdown.push_str("Here are some features:\n\n");
        
        for item in 1..=5 {
            markdown.push_str(&format!("- Feature {} with [link{}](https://example{}.com)\n", item, item, item));
        }
        
        markdown.push_str("\n### Ordered List\n\n");
        
        for item in 1..=3 {
            markdown.push_str(&format!("{}. Ordered item {} with **emphasis**\n", item, item));
        }
        
        markdown.push_str(&format!("\nParagraph {} with mixed **bold** and *italic* formatting, plus some [external links](https://external{}.com) for good measure.\n\n", section, section));
    }
    
    markdown
}