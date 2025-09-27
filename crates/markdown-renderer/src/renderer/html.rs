use crate::parser::AstNode;
use crate::error::MarkdownError;
use super::Renderer;
use std::fmt::Write;

/// Configuration options for HTML rendering
#[derive(Debug, Clone)]
pub struct HtmlOptions {
    /// Whether to pretty-print the HTML with indentation
    pub pretty_print: bool,
    /// Custom CSS classes for different elements
    pub css_classes: CssClasses,
    /// Whether to escape HTML in text content
    pub escape_html: bool,
    /// Whether to add target="_blank" to links
    pub external_links_new_tab: bool,
    /// Custom attributes to add to specific elements
    pub custom_attributes: CustomAttributes,
}

/// CSS class configuration
#[derive(Debug, Clone)]
pub struct CssClasses {
    pub document: Option<String>,
    pub heading: Option<String>,
    pub paragraph: Option<String>,
    pub list_ordered: Option<String>,
    pub list_unordered: Option<String>,
    pub list_item: Option<String>,
    pub bold: Option<String>,
    pub italic: Option<String>,
    pub link: Option<String>,
}

/// Custom HTML attributes
#[derive(Debug, Clone)]
pub struct CustomAttributes {
    pub document: Vec<(String, String)>,
    pub heading: Vec<(String, String)>,
    pub paragraph: Vec<(String, String)>,
    pub list: Vec<(String, String)>,
    pub link: Vec<(String, String)>,
}

impl Default for HtmlOptions {
    fn default() -> Self {
        Self {
            pretty_print: false,
            css_classes: CssClasses::default(),
            escape_html: true,
            external_links_new_tab: false,
            custom_attributes: CustomAttributes::default(),
        }
    }
}

impl Default for CssClasses {
    fn default() -> Self {
        Self {
            document: None,
            heading: None,
            paragraph: None,
            list_ordered: None,
            list_unordered: None,
            list_item: None,
            bold: None,
            italic: None,
            link: None,
        }
    }
}

impl Default for CustomAttributes {
    fn default() -> Self {
        Self {
            document: Vec::new(),
            heading: Vec::new(),
            paragraph: Vec::new(),
            list: Vec::new(),
            link: Vec::new(),
        }
    }
}

/// HTML renderer for converting AST to HTML
pub struct HtmlRenderer {
    options: HtmlOptions,
}

impl HtmlRenderer {
    /// Create a new HTML renderer with default options
    pub fn new() -> Self {
        Self {
            options: HtmlOptions::default(),
        }
    }

    /// Create a new HTML renderer with custom options
    pub fn with_options(options: HtmlOptions) -> Self {
        Self { options }
    }

    /// Render AST node to HTML string
    fn render_node(&self, node: &AstNode, output: &mut String, depth: usize) -> Result<(), MarkdownError> {
        match node {
            AstNode::Document { children } => {
                self.render_document(children, output, depth)?;
            },
            AstNode::Heading { level, content } => {
                self.render_heading(*level, content, output, depth)?;
            },
            AstNode::Paragraph { content } => {
                self.render_paragraph(content, output, depth)?;
            },
            AstNode::List { ordered, items } => {
                self.render_list(*ordered, items, output, depth)?;
            },
            AstNode::ListItem { content } => {
                self.render_list_item(content, output, depth)?;
            },
            AstNode::Bold(content) => {
                self.render_bold(content, output, depth)?;
            },
            AstNode::Italic(content) => {
                self.render_italic(content, output, depth)?;
            },
            AstNode::Link { text, url } => {
                self.render_link(text, url, output, depth)?;
            },
            AstNode::Text(text) => {
                self.render_text(text, output)?;
            },
            AstNode::LineBreak => {
                output.push_str("<br>");
                if self.options.pretty_print {
                    output.push('\n');
                }
            },
        }
        Ok(())
    }

    fn render_document(&self, children: &[AstNode], output: &mut String, depth: usize) -> Result<(), MarkdownError> {
        let indent = if self.options.pretty_print { "  ".repeat(depth) } else { String::new() };
        
        if let Some(ref class) = self.options.css_classes.document {
            write!(output, r#"{}<div class="{}"{}>"#, indent, class, self.format_attributes(&self.options.custom_attributes.document))
                .map_err(|e| MarkdownError::RenderError { message: e.to_string() })?;
        } else if !self.options.custom_attributes.document.is_empty() {
            write!(output, r#"{}<div{}>"#, indent, self.format_attributes(&self.options.custom_attributes.document))
                .map_err(|e| MarkdownError::RenderError { message: e.to_string() })?;
        }

        if self.options.pretty_print && (self.options.css_classes.document.is_some() || !self.options.custom_attributes.document.is_empty()) {
            output.push('\n');
        }

        for child in children {
            self.render_node(child, output, depth + if self.options.css_classes.document.is_some() || !self.options.custom_attributes.document.is_empty() { 1 } else { 0 })?;
        }

        if self.options.css_classes.document.is_some() || !self.options.custom_attributes.document.is_empty() {
            if self.options.pretty_print {
                output.push_str(&format!("{}</div>\n", indent));
            } else {
                output.push_str("</div>");
            }
        }

        Ok(())
    }

    fn render_heading(&self, level: u8, content: &[AstNode], output: &mut String, depth: usize) -> Result<(), MarkdownError> {
        let indent = if self.options.pretty_print { "  ".repeat(depth) } else { String::new() };
        let tag = format!("h{}", level);
        
        write!(output, "{}<{}", indent, tag)
            .map_err(|e| MarkdownError::RenderError { message: e.to_string() })?;

        if let Some(ref class) = self.options.css_classes.heading {
            write!(output, r#" class="{}""#, class)
                .map_err(|e| MarkdownError::RenderError { message: e.to_string() })?;
        }

        write!(output, "{}>{}", self.format_attributes(&self.options.custom_attributes.heading), if self.options.pretty_print { "" } else { "" })
            .map_err(|e| MarkdownError::RenderError { message: e.to_string() })?;

        for child in content {
            self.render_node(child, output, depth)?;
        }

        write!(output, "</{}>", tag)
            .map_err(|e| MarkdownError::RenderError { message: e.to_string() })?;

        if self.options.pretty_print {
            output.push('\n');
        }

        Ok(())
    }

    fn render_paragraph(&self, content: &[AstNode], output: &mut String, depth: usize) -> Result<(), MarkdownError> {
        let indent = if self.options.pretty_print { "  ".repeat(depth) } else { String::new() };
        
        write!(output, "{}<p", indent)
            .map_err(|e| MarkdownError::RenderError { message: e.to_string() })?;

        if let Some(ref class) = self.options.css_classes.paragraph {
            write!(output, r#" class="{}""#, class)
                .map_err(|e| MarkdownError::RenderError { message: e.to_string() })?;
        }

        write!(output, "{}>", self.format_attributes(&self.options.custom_attributes.paragraph))
            .map_err(|e| MarkdownError::RenderError { message: e.to_string() })?;

        for child in content {
            self.render_node(child, output, depth)?;
        }

        output.push_str("</p>");
        if self.options.pretty_print {
            output.push('\n');
        }

        Ok(())
    }

    fn render_list(&self, ordered: bool, items: &[AstNode], output: &mut String, depth: usize) -> Result<(), MarkdownError> {
        let indent = if self.options.pretty_print { "  ".repeat(depth) } else { String::new() };
        let tag = if ordered { "ol" } else { "ul" };
        
        write!(output, "{}<{}", indent, tag)
            .map_err(|e| MarkdownError::RenderError { message: e.to_string() })?;

        let class = if ordered {
            &self.options.css_classes.list_ordered
        } else {
            &self.options.css_classes.list_unordered
        };

        if let Some(ref class) = class {
            write!(output, r#" class="{}""#, class)
                .map_err(|e| MarkdownError::RenderError { message: e.to_string() })?;
        }

        write!(output, "{}>", self.format_attributes(&self.options.custom_attributes.list))
            .map_err(|e| MarkdownError::RenderError { message: e.to_string() })?;

        if self.options.pretty_print {
            output.push('\n');
        }

        for item in items {
            self.render_node(item, output, depth + 1)?;
        }

        if self.options.pretty_print {
            output.push_str(&format!("{}</{}>", indent, tag));
        } else {
            write!(output, "</{}>", tag)
                .map_err(|e| MarkdownError::RenderError { message: e.to_string() })?;
        }

        if self.options.pretty_print {
            output.push('\n');
        }

        Ok(())
    }

    fn render_list_item(&self, content: &[AstNode], output: &mut String, depth: usize) -> Result<(), MarkdownError> {
        let indent = if self.options.pretty_print { "  ".repeat(depth) } else { String::new() };
        
        write!(output, "{}<li", indent)
            .map_err(|e| MarkdownError::RenderError { message: e.to_string() })?;

        if let Some(ref class) = self.options.css_classes.list_item {
            write!(output, r#" class="{}""#, class)
                .map_err(|e| MarkdownError::RenderError { message: e.to_string() })?;
        }

        output.push('>');

        for child in content {
            self.render_node(child, output, depth)?;
        }

        output.push_str("</li>");
        if self.options.pretty_print {
            output.push('\n');
        }

        Ok(())
    }

    fn render_bold(&self, content: &[AstNode], output: &mut String, depth: usize) -> Result<(), MarkdownError> {
        if let Some(ref class) = self.options.css_classes.bold {
            write!(output, r#"<strong class="{}">"#, class)
                .map_err(|e| MarkdownError::RenderError { message: e.to_string() })?;
        } else {
            output.push_str("<strong>");
        }

        for child in content {
            self.render_node(child, output, depth)?;
        }

        output.push_str("</strong>");
        Ok(())
    }

    fn render_italic(&self, content: &[AstNode], output: &mut String, depth: usize) -> Result<(), MarkdownError> {
        if let Some(ref class) = self.options.css_classes.italic {
            write!(output, r#"<em class="{}">"#, class)
                .map_err(|e| MarkdownError::RenderError { message: e.to_string() })?;
        } else {
            output.push_str("<em>");
        }

        for child in content {
            self.render_node(child, output, depth)?;
        }

        output.push_str("</em>");
        Ok(())
    }

    fn render_link(&self, text: &[AstNode], url: &str, output: &mut String, depth: usize) -> Result<(), MarkdownError> {
        write!(output, r#"<a href="{}""#, self.escape_attribute(url))
            .map_err(|e| MarkdownError::RenderError { message: e.to_string() })?;

        if let Some(ref class) = self.options.css_classes.link {
            write!(output, r#" class="{}""#, class)
                .map_err(|e| MarkdownError::RenderError { message: e.to_string() })?;
        }

        if self.options.external_links_new_tab && (url.starts_with("http://") || url.starts_with("https://")) {
            output.push_str(r#" target="_blank" rel="noopener noreferrer""#);
        }

        write!(output, "{}>{}", self.format_attributes(&self.options.custom_attributes.link), if self.options.pretty_print { "" } else { "" })
            .map_err(|e| MarkdownError::RenderError { message: e.to_string() })?;

        for child in text {
            self.render_node(child, output, depth)?;
        }

        output.push_str("</a>");
        Ok(())
    }

    fn render_text(&self, text: &str, output: &mut String) -> Result<(), MarkdownError> {
        if self.options.escape_html {
            output.push_str(&self.escape_html_text(text));
        } else {
            output.push_str(text);
        }
        Ok(())
    }

    fn format_attributes(&self, attributes: &[(String, String)]) -> String {
        if attributes.is_empty() {
            String::new()
        } else {
            let mut result = String::new();
            for (key, value) in attributes {
                write!(result, r#" {}="{}""#, key, self.escape_attribute(value))
                    .unwrap_or_else(|_| {});
            }
            result
        }
    }

    fn escape_html_text(&self, text: &str) -> String {
        text.chars()
            .map(|c| match c {
                '&' => "&amp;".to_string(),
                '<' => "&lt;".to_string(),
                '>' => "&gt;".to_string(),
                '"' => "&quot;".to_string(),
                '\'' => "&#x27;".to_string(),
                _ => c.to_string(),
            })
            .collect()
    }

    fn escape_attribute(&self, text: &str) -> String {
        text.chars()
            .map(|c| match c {
                '&' => "&amp;".to_string(),
                '<' => "&lt;".to_string(),
                '>' => "&gt;".to_string(),
                '"' => "&quot;".to_string(),
                '\'' => "&#x27;".to_string(),
                _ => c.to_string(),
            })
            .collect()
    }
}

impl Default for HtmlRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer for HtmlRenderer {
    type Output = String;
    type Error = MarkdownError;

    fn render(&self, node: &AstNode) -> Result<Self::Output, Self::Error> {
        let mut output = String::new();
        self.render_node(node, &mut output, 0)?;
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::AstNode;

    fn create_simple_ast() -> AstNode {
        AstNode::Document {
            children: vec![
                AstNode::Heading {
                    level: 1,
                    content: vec![AstNode::Text("Hello World".to_string())],
                },
                AstNode::Paragraph {
                    content: vec![
                        AstNode::Text("This is a ".to_string()),
                        AstNode::Bold(vec![AstNode::Text("bold".to_string())]),
                        AstNode::Text(" paragraph.".to_string()),
                    ],
                },
            ],
        }
    }

    #[test]
    fn test_basic_html_rendering() {
        let ast = create_simple_ast();
        let renderer = HtmlRenderer::new();
        let html = renderer.render(&ast).unwrap();
        
        assert!(html.contains("<h1>Hello World</h1>"));
        assert!(html.contains("<p>This is a <strong>bold</strong> paragraph.</p>"));
    }

    #[test]
    fn test_pretty_printing() {
        let ast = create_simple_ast();
        let options = HtmlOptions {
            pretty_print: true,
            ..Default::default()
        };
        let renderer = HtmlRenderer::with_options(options);
        let html = renderer.render(&ast).unwrap();
        
        assert!(html.contains('\n'));
        // The simple AST doesn't have nested structure, so no indentation expected
        // Just check that newlines are present for pretty printing
    }

    #[test]
    fn test_css_classes() {
        let ast = AstNode::Paragraph {
            content: vec![AstNode::Text("Test".to_string())],
        };
        
        let mut css_classes = CssClasses::default();
        css_classes.paragraph = Some("my-paragraph".to_string());
        
        let options = HtmlOptions {
            css_classes,
            ..Default::default()
        };
        
        let renderer = HtmlRenderer::with_options(options);
        let html = renderer.render(&ast).unwrap();
        
        assert!(html.contains(r#"<p class="my-paragraph">Test</p>"#));
    }

    #[test]
    fn test_html_escaping() {
        let ast = AstNode::Text("<script>alert('xss')</script>".to_string());
        let renderer = HtmlRenderer::new();
        let html = renderer.render(&ast).unwrap();
        
        assert!(html.contains("&lt;script&gt;"));
        assert!(!html.contains("<script>"));
    }

    #[test]
    fn test_external_links() {
        let ast = AstNode::Link {
            text: vec![AstNode::Text("External Link".to_string())],
            url: "https://example.com".to_string(),
        };
        
        let options = HtmlOptions {
            external_links_new_tab: true,
            ..Default::default()
        };
        
        let renderer = HtmlRenderer::with_options(options);
        let html = renderer.render(&ast).unwrap();
        
        assert!(html.contains(r#"target="_blank""#));
        assert!(html.contains(r#"rel="noopener noreferrer""#));
    }

    #[test]
    fn test_lists() {
        let ast = AstNode::List {
            ordered: true,
            items: vec![
                AstNode::ListItem {
                    content: vec![AstNode::Text("First item".to_string())],
                },
                AstNode::ListItem {
                    content: vec![AstNode::Text("Second item".to_string())],
                },
            ],
        };
        
        let renderer = HtmlRenderer::new();
        let html = renderer.render(&ast).unwrap();
        
        assert!(html.contains("<ol>"));
        assert!(html.contains("<li>First item</li>"));
        assert!(html.contains("<li>Second item</li>"));
        assert!(html.contains("</ol>"));
    }

    #[test]
    fn test_empty_content() {
        let ast = AstNode::Paragraph { content: vec![] };
        let renderer = HtmlRenderer::new();
        let html = renderer.render(&ast).unwrap();
        
        assert_eq!(html, "<p></p>");
    }

    #[test]
    fn test_line_break() {
        let ast = AstNode::LineBreak;
        let renderer = HtmlRenderer::new();
        let html = renderer.render(&ast).unwrap();
        
        assert_eq!(html, "<br>");
    }
}