use select::node::Node;

/// Convert node to text string
pub fn node_to_text(node: Node) -> String {
    node.text().trim().to_string()
}
