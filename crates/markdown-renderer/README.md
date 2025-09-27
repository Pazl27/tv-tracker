# Markdown Renderer

A Rust crate for parsing Markdown text into an Abstract Syntax Tree (AST) and rendering it to various output formats.

## Features

This crate provides a complete pipeline from Markdown text to structured output:

- **Lexer**: Tokenizes Markdown text into a stream of tokens
- **Parser**: Converts tokens into an Abstract Syntax Tree
- **AST**: Rich tree structure representing Markdown elements
- **HTML Renderer**: Convert AST to HTML (optional feature)

### Supported Markdown Elements

- **Headings** (H1-H6): `# ## ### #### ##### ######`
- **Emphasis**: 
  - *Italic*: `*text*`
  - **Bold**: `**text**`
- **Links**: `[text](url)`
- **Lists**:
  - Unordered: `- item`
  - Ordered: `1. item`
- **Paragraphs**: Plain text blocks
- **Line breaks**: `\n`

## Feature Flags

This crate uses feature flags to enable optional functionality:

- `html` - Enables HTML rendering capabilities
- `full` - Enables all features (currently just `html`)

By default, no features are enabled, giving you a minimal parser-only build.

### Enabling Features

Add to your `Cargo.toml`:

```toml
# For parsing only (default)
markdown-renderer = "0.1"

# For HTML rendering
markdown-renderer = { version = "0.1", features = ["html"] }

# For all features
markdown-renderer = { version = "0.1", features = ["full"] }
```

## Usage

### Basic Parsing (No Features Required)

```rust
use markdown_renderer::parse_markdown;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let markdown = "# Hello **World**\n\nThis is a paragraph with *italic* text.";
    let ast = parse_markdown(markdown)?;
    println!("{:#?}", ast);
    Ok(())
}
```

### HTML Rendering (Requires `html` Feature)

```rust
use markdown_renderer::{markdown_to_html, parse_markdown, render_html};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let markdown = "# Hello **World**\n\nThis is a paragraph with *italic* text.";
    
    // Direct conversion
    let html = markdown_to_html(markdown)?;
    println!("{}", html);
    // Output: <h1>Hello <strong>World</strong></h1><p>This is a paragraph with <em>italic</em> text.</p>
    
    // Or parse then render
    let ast = parse_markdown(markdown)?;
    let html = render_html(&ast)?;
    println!("{}", html);
    
    Ok(())
}
```

### HTML Rendering with Custom Options

```rust
use markdown_renderer::{
    markdown_to_html_with_options, HtmlOptions, CssClasses, CustomAttributes
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let markdown = "# Title\n\n[Link](https://example.com)";
    
    // Configure HTML options
    let mut css_classes = CssClasses::default();
    css_classes.heading = Some("my-heading".to_string());
    css_classes.link = Some("my-link".to_string());
    
    let mut custom_attributes = CustomAttributes::default();
    custom_attributes.document = vec![("data-theme".to_string(), "dark".to_string())];
    
    let options = HtmlOptions {
        pretty_print: true,
        css_classes,
        custom_attributes,
        external_links_new_tab: true,
        escape_html: true,
    };
    
    let html = markdown_to_html_with_options(markdown, options)?;
    println!("{}", html);
    
    Ok(())
}
```

### Step-by-Step Processing

```rust
use markdown_renderer::{tokenize, parse, AstNode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let markdown = "## Heading\n\n- Item 1\n- Item 2";
    
    // Step 1: Tokenize
    let tokens = tokenize(markdown)?;
    println!("Tokens: {:?}", tokens);
    
    // Step 2: Parse to AST
    let ast = parse(tokens)?;
    println!("AST: {:#?}", ast);
    
    Ok(())
}
```

## AST Structure

The AST is built using the `AstNode` enum:

```rust
pub enum AstNode {
    Document { children: Vec<AstNode> },
    
    // Block elements
    Heading { level: u8, content: Vec<AstNode> },
    Paragraph { content: Vec<AstNode> },
    List { ordered: bool, items: Vec<AstNode> },
    ListItem { content: Vec<AstNode> },
    
    // Inline elements
    Text(String),
    Bold(Vec<AstNode>),
    Italic(Vec<AstNode>),
    Link { text: Vec<AstNode>, url: String },
    
    LineBreak,
}
```

### Working with the AST

```rust
use markdown_renderer::{parse_markdown, AstNode};

fn extract_headings(node: &AstNode) -> Vec<(u8, String)> {
    let mut headings = Vec::new();
    
    match node {
        AstNode::Document { children } => {
            for child in children {
                headings.extend(extract_headings(child));
            }
        },
        AstNode::Heading { level, content } => {
            let text = extract_text(content);
            headings.push((*level, text));
        },
        AstNode::Paragraph { content } |
        AstNode::Bold(content) |
        AstNode::Italic(content) => {
            for child in content {
                headings.extend(extract_headings(child));
            }
        },
        AstNode::List { items, .. } => {
            for item in items {
                headings.extend(extract_headings(item));
            }
        },
        AstNode::ListItem { content } => {
            for child in content {
                headings.extend(extract_headings(child));
            }
        },
        AstNode::Link { text, .. } => {
            for child in text {
                headings.extend(extract_headings(child));
            }
        },
        _ => {} // Text and LineBreak don't contain headings
    }
    
    headings
}

fn extract_text(nodes: &[AstNode]) -> String {
    nodes.iter()
        .filter_map(|node| match node {
            AstNode::Text(text) => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("")
}
```

## Error Handling

The crate provides comprehensive error handling:

```rust
use markdown_renderer::{parse_markdown, MarkdownError};

match parse_markdown("# Invalid **bold") {
    Ok(ast) => println!("Parsed successfully: {:#?}", ast),
    Err(MarkdownError::ParseError(e)) => {
        eprintln!("Parse error: {}", e);
    },
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

## Examples

### Parser Example

Run the parsing example:

```bash
cargo run --example parse_markdown
```

This demonstrates:
1. Tokenization of Markdown text
2. AST generation
3. AST analysis and statistics

### HTML Rendering Example (Requires `html` feature)

Run the HTML rendering example:

```bash
cargo run --example html_rendering --features html
```

This demonstrates:
1. Basic HTML conversion
2. Pretty-printed HTML
3. Custom CSS classes
4. Custom attributes
5. External link handling
6. Performance testing

## API Reference

### Core Functions

- `parse_markdown(input: &str) -> Result<AstNode, MarkdownError>` - Complete parsing pipeline
- `tokenize(input: &str) -> Result<Vec<Token>, LexerError>` - Tokenize only
- `parse(tokens: Vec<Token>) -> Result<AstNode, ParseError>` - Parse tokens to AST

### HTML Functions (with `html` feature)

- `markdown_to_html(input: &str) -> Result<String, MarkdownError>` - Direct markdown to HTML
- `markdown_to_html_with_options(input: &str, options: HtmlOptions) -> Result<String, MarkdownError>` - With custom options
- `render_html(ast: &AstNode) -> Result<String, MarkdownError>` - Render AST to HTML
- `render_html_with_options(ast: &AstNode, options: HtmlOptions) -> Result<String, MarkdownError>` - With custom options

### Core Types

- `AstNode` - Represents nodes in the Abstract Syntax Tree
- `Token` - Represents lexical tokens
- `Parser` - The parser struct for manual parsing
- `Lexer` - The lexer struct for manual tokenization

### HTML Types (with `html` feature)

- `HtmlRenderer` - HTML renderer implementing the `Renderer` trait
- `HtmlOptions` - Configuration for HTML rendering
- `CssClasses` - CSS class configuration for HTML elements
- `CustomAttributes` - Custom HTML attributes configuration
- `Renderer` - Trait for implementing custom renderers

### Errors

- `MarkdownError` - Top-level error type
- `ParseError` - Parser-specific errors  
- `LexerError` - Lexer-specific errors

## HTML Rendering Options

When using the `html` feature, you can customize the output with `HtmlOptions`:

### Configuration Options

- `pretty_print: bool` - Add indentation and newlines for readable HTML
- `escape_html: bool` - Escape HTML characters in text content (default: true)
- `external_links_new_tab: bool` - Add `target="_blank"` to external links
- `css_classes: CssClasses` - Custom CSS classes for elements
- `custom_attributes: CustomAttributes` - Custom HTML attributes

### CSS Classes

You can add custom CSS classes to any HTML element:

```rust
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
```

### Custom Attributes

Add custom HTML attributes to elements:

```rust
let mut custom_attributes = CustomAttributes::default();
custom_attributes.document = vec![("data-theme".to_string(), "dark".to_string())];
custom_attributes.heading = vec![("data-level".to_string(), "auto".to_string())];
custom_attributes.link = vec![("data-track".to_string(), "click".to_string())];
```

## Testing

Run tests for all features:

```bash
# Test parser only
cargo test

# Test with HTML features
cargo test --features html

# Test all features
cargo test --features full
```

## License

This project is licensed under the same terms as the parent project.