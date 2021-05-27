use crate::ast::ast::{Node, NodeType};
use crate::token::token::Token;

pub trait Emitter {
    fn new() -> Self;
    fn add_header(&mut self);
    fn program(&mut self, node: &Node);
    fn function_def(&mut self, node: &Node);
    fn function_signature(&mut self, node: &Node);
    fn block(&mut self, node: &Node);

    fn get_code(&self) -> String;
}
