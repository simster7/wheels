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
        if self.token.is_some() {
            write!(f, "{:?} ({:?})", self.node_type, self.token.as_ref().unwrap())?;
        } else {
            write!(f, "{:?}", self.node_type)?;
        }
        if self.children.len() > 0 {
            write!(f, " [\n")?;
            for child in self.children.iter() {
                write!(f, "\t{}\n", child)?;
            }
            write!(f, "]")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum NodeType {
    Function,
    Identifier,
    ParameterList,
}
