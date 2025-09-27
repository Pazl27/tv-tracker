use markdown_renderer::{parse_markdown, tokenize, AstNode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example markdown content
    let markdown_content = r#"# My Markdown Document

This is a **bold** paragraph with some *italic* text and a [link](https://example.com).

## Features

Here are some features:

- **Headings** (H1-H6)
- *Emphasis* and **strong** text
- Links like [Rust](https://rust-lang.org)
- Lists (both ordered and unordered)

### Ordered List Example

1. First item
2. Second item
3. Third item

That's all for now!
"#;

    println!("=== Original Markdown ===");
    println!("{}", markdown_content);
    
    println!("\n=== Step 1: Tokenization ===");
    let tokens = tokenize(markdown_content)?;
    println!("Generated {} tokens:", tokens.len());
    for (i, token) in tokens.iter().enumerate().take(10) {
        println!("  {}: {:?}", i, token);
    }
    if tokens.len() > 10 {
        println!("  ... and {} more tokens", tokens.len() - 10);
    }
    
    println!("\n=== Step 2: Parsing to AST ===");
    let ast = parse_markdown(markdown_content)?;
    println!("Generated AST:");
    print_ast(&ast, 0);
    
    println!("\n=== Step 3: AST Analysis ===");
    analyze_ast(&ast);
    
    Ok(())
}

fn print_ast(node: &AstNode, indent: usize) {
    let spaces = "  ".repeat(indent);
    
    match node {
        AstNode::Document { children } => {
            println!("{}Document ({} children)", spaces, children.len());
            for child in children {
                print_ast(child, indent + 1);
            }
        },
        AstNode::Heading { level, content } => {
            println!("{}Heading (level {})", spaces, level);
            for child in content {
                print_ast(child, indent + 1);
            }
        },
        AstNode::Paragraph { content } => {
            println!("{}Paragraph ({} elements)", spaces, content.len());
            for child in content {
                print_ast(child, indent + 1);
            }
        },
        AstNode::List { ordered, items } => {
            let list_type = if *ordered { "Ordered" } else { "Unordered" };
            println!("{}{} List ({} items)", spaces, list_type, items.len());
            for child in items {
                print_ast(child, indent + 1);
            }
        },
        AstNode::ListItem { content } => {
            println!("{}ListItem", spaces);
            for child in content {
                print_ast(child, indent + 1);
            }
        },
        AstNode::Bold(content) => {
            println!("{}Bold", spaces);
            for child in content {
                print_ast(child, indent + 1);
            }
        },
        AstNode::Italic(content) => {
            println!("{}Italic", spaces);
            for child in content {
                print_ast(child, indent + 1);
            }
        },
        AstNode::Link { text, url } => {
            println!("{}Link (url: \"{}\")", spaces, url);
            for child in text {
                print_ast(child, indent + 1);
            }
        },
        AstNode::Text(text) => {
            let display_text = if text.len() > 30 {
                format!("{}...", &text[..27])
            } else {
                text.clone()
            };
            println!("{}Text: \"{}\"", spaces, display_text);
        },
        AstNode::LineBreak => {
            println!("{}LineBreak", spaces);
        },
    }
}

fn analyze_ast(node: &AstNode) {
    let mut stats = AstStats::new();
    collect_stats(node, &mut stats);
    
    println!("AST Statistics:");
    println!("  Total nodes: {}", stats.total_nodes);
    println!("  Headings: {}", stats.headings);
    println!("  Paragraphs: {}", stats.paragraphs);
    println!("  Lists: {}", stats.lists);
    println!("  Links: {}", stats.links);
    println!("  Bold elements: {}", stats.bold);
    println!("  Italic elements: {}", stats.italic);
    println!("  Text nodes: {}", stats.text_nodes);
    println!("  Total text length: {} characters", stats.total_text_length);
}

#[derive(Default)]
struct AstStats {
    total_nodes: usize,
    headings: usize,
    paragraphs: usize,
    lists: usize,
    links: usize,
    bold: usize,
    italic: usize,
    text_nodes: usize,
    total_text_length: usize,
}

impl AstStats {
    fn new() -> Self {
        Self::default()
    }
}

fn collect_stats(node: &AstNode, stats: &mut AstStats) {
    stats.total_nodes += 1;
    
    match node {
        AstNode::Document { children } => {
            for child in children {
                collect_stats(child, stats);
            }
        },
        AstNode::Heading { content, .. } => {
            stats.headings += 1;
            for child in content {
                collect_stats(child, stats);
            }
        },
        AstNode::Paragraph { content } => {
            stats.paragraphs += 1;
            for child in content {
                collect_stats(child, stats);
            }
        },
        AstNode::List { items, .. } => {
            stats.lists += 1;
            for child in items {
                collect_stats(child, stats);
            }
        },
        AstNode::ListItem { content } => {
            for child in content {
                collect_stats(child, stats);
            }
        },
        AstNode::Bold(content) => {
            stats.bold += 1;
            for child in content {
                collect_stats(child, stats);
            }
        },
        AstNode::Italic(content) => {
            stats.italic += 1;
            for child in content {
                collect_stats(child, stats);
            }
        },
        AstNode::Link { text, .. } => {
            stats.links += 1;
            for child in text {
                collect_stats(child, stats);
            }
        },
        AstNode::Text(text) => {
            stats.text_nodes += 1;
            stats.total_text_length += text.len();
        },
        AstNode::LineBreak => {
            // LineBreak doesn't need special counting
        },
    }
}