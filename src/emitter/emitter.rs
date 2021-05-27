use crate::ast::ast::{Node, NodeType};
use crate::token::token::Token;

pub trait Emitter {
    fn new(ast: Node) -> Self;
    fn header(&self) -> String;
    fn program(&self) -> String;
    fn function_def(&self, node: &Node) -> String;
    fn function_signature(&self, node: &Node) -> String;
}
