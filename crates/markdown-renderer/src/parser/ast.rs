#[derive(Debug, Clone, PartialEq)]
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

    LineBreak
}

impl AstNode {
    pub fn is_inline(&self) -> bool {
        matches!(self,
            AstNode::Text(_) |
            AstNode::Bold(_) |
            AstNode::Italic(_) |
            AstNode::Link { .. } |
            AstNode::LineBreak
        )
    }

    pub fn is_block(&self) -> bool {
        !self.is_inline()
    }
}
