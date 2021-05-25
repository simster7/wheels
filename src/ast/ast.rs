
#[derive(Debug)]
pub struct Node {
    node_type: NodeType,
    children: Option<Vec<Node>>,
    // TODO: Symbol table
}

impl Node {
    pub fn new(node_type: NodeType, children: Option<Vec<Node>>) -> Node {
        Node {node_type, children}
    }
}

#[derive(Debug)]
pub enum NodeType {
    Function,
    Identifier,
}
