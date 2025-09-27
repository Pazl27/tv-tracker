use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use markdown_renderer::{tokenize, parse, parse_markdown};

#[cfg(feature = "html")]
use markdown_renderer::render_html;

// Sample markdown content of varying sizes and complexity
const SMALL_MARKDOWN: &str = r#"# Hello World

This is a simple paragraph with **bold** and *italic* text.

Here's a [link](https://example.com) to somewhere."#;

const MEDIUM_MARKDOWN: &str = r#"# Project Documentation

## Overview

This is a comprehensive guide to using our **markdown renderer**. It supports various features including:

- **Bold text** and *italic text*
- [External links](https://github.com/example/repo)
- Multiple heading levels
- Nested lists and complex formatting

## Installation

To install this library, add it to your `Cargo.toml`:

```toml
[dependencies]
markdown-renderer = "0.1.0"
```

### Usage Examples

Here are some basic usage examples:

1. First, create a lexer
2. Then parse the tokens
3. Finally, render to HTML

## Advanced Features

The renderer supports:
- Custom CSS classes
- HTML attribute injection
- Pretty printing with indentation
- Proper HTML escaping for security

### Performance Notes

The parser is designed to be fast and memory efficient. It uses a **recursive descent** approach with proper error handling.

## Conclusion

This markdown renderer provides a robust solution for converting markdown to HTML with excellent performance characteristics."#;

const LARGE_MARKDOWN: &str = r#"# Complete Markdown Feature Test

## Table of Contents

1. [Basic Formatting](#basic-formatting)
2. [Lists and Structure](#lists-and-structure)
3. [Links and References](#links-and-references)
4. [Advanced Examples](#advanced-examples)

## Basic Formatting

### Text Emphasis

This paragraph demonstrates **bold text**, *italic text*, and even ***bold italic*** combinations.
You can also use __alternative bold__ and _alternative italic_ syntax.

### Headings

# Heading Level 1
## Heading Level 2
### Heading Level 3
#### Heading Level 4
##### Heading Level 5
###### Heading Level 6

## Lists and Structure

### Unordered Lists

- First item with **bold** text
- Second item with *italic* text
- Third item with a [link](https://example.com)
  - Nested item one
  - Nested item two
    - Deeply nested item
    - Another deeply nested item
- Fourth item with `inline code`

### Ordered Lists

1. First numbered item
2. Second numbered item with **formatting**
3. Third numbered item
   1. Nested numbered item
   2. Another nested item
   3. Third nested item
4. Fourth numbered item

### Mixed Lists

1. Ordered item
   - Unordered sub-item
   - Another unordered sub-item
2. Another ordered item
   1. Ordered sub-item
   2. Another ordered sub-item

## Links and References

Here are various types of links:

- [Simple link](https://github.com)
- [Link with **bold** text](https://example.com)
- [Another link](https://rust-lang.org) in the middle of text
- Multiple [link one](https://first.com) and [link two](https://second.com) in one paragraph

### Complex Paragraph with Everything

This is a **very complex** paragraph that contains *multiple* types of formatting. 
It has [several links](https://example1.com) scattered throughout, including 
[this one](https://example2.com) and [this other one](https://example3.com). 
The paragraph also contains ***bold and italic*** text combinations.

## Advanced Examples

### Long Paragraph Test

Lorem ipsum dolor sit amet, consectetur adipiscing elit. **Sed do eiusmod** tempor incididunt ut labore et dolore magna aliqua. *Ut enim ad minim* veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. [Duis aute](https://example.com) irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. ***Excepteur sint*** occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.

Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, **totam rem** aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. *Nemo enim* ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt.

### Stress Test Section

Here's a section designed to stress test the parser with lots of formatting:

**Bold** *italic* ***bold-italic*** **bold** *italic* ***bold-italic***
[link1](https://1.com) [link2](https://2.com) [link3](https://3.com)

1. Item **one** with *formatting*
2. Item **two** with [link](https://example.com)
3. Item **three** with ***everything***
   - Sub **item** *one*
   - Sub **item** *two* with [link](https://sub.com)
   - Sub **item** ***three***

## Final Section

This is the final section of our comprehensive markdown test. It contains a mix of all the elements we've tested above, providing a good benchmark for real-world usage scenarios.

The parser should handle all of these elements efficiently, maintaining good performance even with complex nested structures and multiple formatting types within the same document."#;

// Generate even larger content by repeating sections
fn generate_huge_markdown() -> String {
    let mut content = String::with_capacity(LARGE_MARKDOWN.len() * 10);
    for i in 0..10 {
        content.push_str(&format!("# Section {}\n\n", i + 1));
        content.push_str(LARGE_MARKDOWN);
        content.push_str("\n\n");
    }
    content
}

fn bench_lexer(c: &mut Criterion) {
    let mut group = c.benchmark_group("lexer");
    
    let samples = [
        ("small", SMALL_MARKDOWN),
        ("medium", MEDIUM_MARKDOWN), 
        ("large", LARGE_MARKDOWN),
    ];
    
    for (name, content) in samples {
        group.throughput(Throughput::Bytes(content.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("tokenize", name),
            content,
            |b, content| {
                b.iter(|| {
                    let tokens = tokenize(black_box(content));
                    black_box(tokens)
                });
            },
        );
    }
    
    // Benchmark huge content
    let huge_content = generate_huge_markdown();
    group.throughput(Throughput::Bytes(huge_content.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("tokenize", "huge"),
        &huge_content,
        |b, content| {
            b.iter(|| {
                let tokens = tokenize(black_box(content));
                black_box(tokens)
            });
        },
    );
    
    group.finish();
}

fn bench_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser");
    
    let samples = [
        ("small", SMALL_MARKDOWN),
        ("medium", MEDIUM_MARKDOWN),
        ("large", LARGE_MARKDOWN),
    ];
    
    for (name, content) in samples {
        group.throughput(Throughput::Bytes(content.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("parse", name),
            content,
            |b, content| {
                b.iter(|| {
                    let tokens = tokenize(black_box(content)).unwrap();
                    let ast = parse(tokens);
                    black_box(ast)
                });
            },
        );
    }
    
    // Benchmark huge content
    let huge_content = generate_huge_markdown();
    group.throughput(Throughput::Bytes(huge_content.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("parse", "huge"),
        &huge_content,
        |b, content| {
            b.iter(|| {
                let tokens = tokenize(black_box(content)).unwrap();
                let ast = parse(tokens);
                black_box(ast)
            });
        },
    );
    
    group.finish();
}

#[cfg(feature = "html")]
fn bench_html_renderer(c: &mut Criterion) {
    let mut group = c.benchmark_group("html_renderer");
    
    let samples = [
        ("small", SMALL_MARKDOWN),
        ("medium", MEDIUM_MARKDOWN),
        ("large", LARGE_MARKDOWN),
    ];
    
    for (name, content) in samples {
        // Pre-parse the content for rendering benchmarks
        if let Ok(ast) = parse_markdown(content) {
            group.throughput(Throughput::Bytes(content.len() as u64));
            group.bench_with_input(
                BenchmarkId::new("render", name),
                &ast,
                |b, ast| {
                    b.iter(|| {
                        let html = render_html(black_box(ast));
                        black_box(html)
                    });
                },
            );
        }
    }
    
    // Benchmark huge content
    let huge_content = generate_huge_markdown();
    if let Ok(ast) = parse_markdown(&huge_content) {
        group.throughput(Throughput::Bytes(huge_content.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("render", "huge"),
            &ast,
            |b, ast| {
                b.iter(|| {
                    let html = render_html(black_box(ast));
                    black_box(html)
                });
            },
        );
    }
    
    group.finish();
}

fn bench_end_to_end(c: &mut Criterion) {
    let mut group = c.benchmark_group("end_to_end");
    
    let samples = [
        ("small", SMALL_MARKDOWN),
        ("medium", MEDIUM_MARKDOWN),
        ("large", LARGE_MARKDOWN),
    ];
    
    for (name, content) in samples {
        group.throughput(Throughput::Bytes(content.len() as u64));
        
        // Benchmark parsing only (lexer + parser)
        group.bench_with_input(
            BenchmarkId::new("lex_and_parse", name),
            content,
            |b, content| {
                b.iter(|| {
                    let ast = parse_markdown(black_box(content));
                    black_box(ast)
                });
            },
        );
        
        #[cfg(feature = "html")]
        {
            // Benchmark full pipeline (lexer + parser + renderer)
            group.bench_with_input(
                BenchmarkId::new("full_pipeline", name),
                content,
                |b, content| {
                    b.iter(|| {
                        if let Ok(ast) = parse_markdown(black_box(content)) {
                            let html = render_html(&ast);
                            black_box(html)
                        } else {
                            black_box(Ok(String::new()))
                        }
                    });
                },
            );
        }
    }
    
    // Benchmark huge content end-to-end
    let huge_content = generate_huge_markdown();
    group.throughput(Throughput::Bytes(huge_content.len() as u64));
    
    group.bench_with_input(
        BenchmarkId::new("lex_and_parse", "huge"),
        &huge_content,
        |b, content| {
            b.iter(|| {
                let ast = parse_markdown(black_box(content));
                black_box(ast)
            });
        },
    );
    
    #[cfg(feature = "html")]
    {
        group.bench_with_input(
            BenchmarkId::new("full_pipeline", "huge"),
            &huge_content,
            |b, content| {
                b.iter(|| {
                    if let Ok(ast) = parse_markdown(black_box(content)) {
                        let html = render_html(&ast);
                        black_box(html)
                    } else {
                        black_box(Ok(String::new()))
                    }
                });
            },
        );
    }
    
    group.finish();
}

fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");
    
    // Benchmark token allocation patterns
    group.bench_function("token_allocation", |b| {
        b.iter(|| {
            let tokens = tokenize(black_box(LARGE_MARKDOWN)).unwrap();
            // Force tokens to stay allocated during measurement
            black_box(tokens.len())
        });
    });
    
    // Benchmark AST allocation patterns
    group.bench_function("ast_allocation", |b| {
        b.iter(|| {
            if let Ok(ast) = parse_markdown(black_box(LARGE_MARKDOWN)) {
                // Force AST to stay allocated during measurement
                let size = std::mem::size_of_val(&ast);
                black_box(size)
            } else {
                black_box(0)
            }
        });
    });
    
    #[cfg(feature = "html")]
    {
        // Benchmark HTML string allocation patterns
        group.bench_function("html_allocation", |b| {
            // Pre-parse once
            let ast = parse_markdown(LARGE_MARKDOWN).unwrap_or_else(|_| {
                // Fallback to a simple document if parsing fails
                parse_markdown("# Simple fallback").unwrap()
            });
            
            b.iter(|| {
                if let Ok(html) = render_html(black_box(&ast)) {
                    // Force HTML string to stay allocated during measurement
                    black_box(html.len())
                } else {
                    black_box(0)
                }
            });
        });
    }
    
    group.finish();
}

fn bench_specific_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("specific_patterns");
    
    // Benchmark emphasis-heavy content
    let emphasis_heavy = "**bold** *italic* ***both*** ".repeat(100);
    group.bench_function("emphasis_heavy", |b| {
        b.iter(|| {
            let ast = parse_markdown(black_box(&emphasis_heavy));
            black_box(ast)
        });
    });
    
    // Benchmark link-heavy content
    let link_heavy = "[link](https://example.com) ".repeat(100);
    group.bench_function("link_heavy", |b| {
        b.iter(|| {
            let ast = parse_markdown(black_box(&link_heavy));
            black_box(ast)
        });
    });
    
    // Benchmark list-heavy content
    let mut list_heavy = String::new();
    for i in 0..100 {
        list_heavy.push_str(&format!("- List item {}\n", i));
    }
    group.bench_function("list_heavy", |b| {
        b.iter(|| {
            let ast = parse_markdown(black_box(&list_heavy));
            black_box(ast)
        });
    });
    
    // Benchmark heading-heavy content
    let mut heading_heavy = String::new();
    for i in 0..100 {
        let level = (i % 6) + 1;
        let hashes = "#".repeat(level);
        heading_heavy.push_str(&format!("{} Heading {}\n\n", hashes, i));
    }
    group.bench_function("heading_heavy", |b| {
        b.iter(|| {
            let ast = parse_markdown(black_box(&heading_heavy));
            black_box(ast)
        });
    });
    
    group.finish();
}

// Create benchmark groups
#[cfg(feature = "html")]
criterion_group!(
    benches,
    bench_lexer,
    bench_parser, 
    bench_html_renderer,
    bench_end_to_end,
    bench_memory_usage,
    bench_specific_patterns
);

#[cfg(not(feature = "html"))]
criterion_group!(
    benches,
    bench_lexer,
    bench_parser,
    bench_end_to_end,
    bench_memory_usage,
    bench_specific_patterns
);

criterion_main!(benches);