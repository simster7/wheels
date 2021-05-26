use crate::token::token::Token;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Node {
    node_type: NodeType,
    children: Vec<Node>,
    token: Option<Token>,
    // TODO: Symbol table
}

impl Node {
    pub fn new(node_type: NodeType) -> Node {
        Node {node_type, children: Vec::new(), token: None}
    }
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child)
    }
    pub fn add_token(&mut self, token: Token) {
        self.token = Some(token)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", format_print(self, 0))
    }
}

fn format_print(node: &Node, level: usize) -> String {
    let tabs = "\t".repeat(level);

    let mut out = String::from("");
    if node.token.is_some() {
        out += format!("{}{:?} ({:?})", tabs, node.node_type, node.token.as_ref().unwrap()).as_str();
    } else {
        out += format!("{}{:?}", tabs, node.node_type).as_str();
    }

    if node.children.len() > 0 {
        out += " [\n";
        for child in node.children.iter() {
            out += format_print(child, level+1).as_str();
        }
        out += format!("{}]", tabs).as_str();
    }
    out += "\n";
    out
}

#[derive(Debug)]
pub enum NodeType {
    Function,
    Identifier,
    FunctionSignature,
    ParameterList,
    Return,
    Parameter,
    Type,
    VarDecl,
}
