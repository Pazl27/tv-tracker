#[cfg(feature = "html")]
pub mod html;

#[cfg(feature = "html")]
pub use html::HtmlRenderer;

use crate::parser::AstNode;

#[cfg(feature = "html")]
use crate::error::MarkdownError;

/// Trait for rendering AST nodes to different output formats
pub trait Renderer {
    type Output;
    type Error;

    /// Render an AST node to the target format
    fn render(&self, node: &AstNode) -> Result<Self::Output, Self::Error>;
}

/// Render AST to HTML (only available with "html" feature)
#[cfg(feature = "html")]
pub fn render_html(ast: &AstNode) -> Result<String, MarkdownError> {
    let renderer = HtmlRenderer::new();
    renderer.render(ast)
}

/// Render AST to HTML with custom options (only available with "html" feature)
#[cfg(feature = "html")]
pub fn render_html_with_options(ast: &AstNode, options: html::HtmlOptions) -> Result<String, MarkdownError> {
    let renderer = HtmlRenderer::with_options(options);
    renderer.render(ast)
}