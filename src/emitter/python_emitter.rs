use crate::ast::ast::{Node, NodeType};
use crate::emitter::emitter::Emitter;
use crate::token::token::{get_literal, Token};

pub struct PythonEmitter {
    root: Node,
}

impl Emitter for PythonEmitter {
    fn new(ast: Node) -> Self {
        PythonEmitter { root: ast }
    }

    fn header(&self) -> String {
        "# Generated from wheels code\n".into()
    }

    fn program(&self) -> String {
        let mut program = self.header();

        program += self.function_def(&self.root).as_str();

        // for child in self.root.get_children().iter() {
        //     if *child.get_type() == NodeType::Function {
        //         program += self.function_def(child).as_str()
        //     }
        // }

        program
    }

    fn function_def(&self, node: &Node) -> String {
        node.ensure_type(NodeType::Function);
        let mut code = String::from("def ");

        code += self.function_signature(node.must_get_only_child()).as_str();

        code
    }

    fn function_signature(&self, node: &Node) -> String {
        node.ensure_type(NodeType::FunctionSignature);
        let mut code = String::from("(");

        for child in node.get_children().iter() {
            child.ensure_type(NodeType::ParameterList);

            code += get_literal(child.must_get_token_ref());
        }

        code
    }
}
