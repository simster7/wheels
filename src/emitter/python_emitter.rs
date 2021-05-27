use crate::ast::ast::{Node, NodeType};
use crate::emitter::emitter::Emitter;
use crate::token::token::{get_literal, Token};

pub struct PythonEmitter {
    level: usize,
    output: String,
}

impl PythonEmitter {
    fn tabs(&self) -> String {
        " ".to_owned().repeat(4 * self.level)
    }
    fn new_line(&mut self) {
        self.output += "\n";
        self.output += self.tabs().as_str();
    }
    fn new_line_with_indent(&mut self) {
        self.level += 1;
        self.new_line()
    }
    fn new_line_with_back(&mut self) {
        self.level -= 1;
        self.new_line()
    }
}

impl Emitter for PythonEmitter {
    fn new() -> Self {
        PythonEmitter {
            level: 0,
            output: "".to_owned(),
        }
    }

    fn add_header(&mut self) {
        self.output += "# Generated from wheels code";
        self.new_line();
    }

    fn program(&mut self, node: &Node) {
        self.add_header();

        node.ensure_type(NodeType::Program);

        for child in node.get_children().iter() {
            if child.is_type(NodeType::Function) {
                self.function_def(child);
            }
        }
    }

    fn function_def(&mut self, node: &Node) {
        node.ensure_type(NodeType::Function);

        self.output += format!("def {}", get_literal(node.must_get_token_ref())).as_str();

        self.function_signature(node.get_child(0));
        self.new_line_with_indent();
    }

    fn function_signature(&mut self, node: &Node) {
        node.ensure_type(NodeType::FunctionSignature);
        self.output += "(";

        let parameter_list = node.get_child(0);
        parameter_list.ensure_type(NodeType::ParameterList);

        self.output += parameter_list
            .get_children()
            .iter()
            .map(|child| {
                child.ensure_type(NodeType::Parameter);
                get_literal(child.must_get_token_ref())
            })
            .collect::<Vec<&str>>()
            .join(", ")
            .as_str();

        self.output += "):";
    }

    fn block(&mut self, node: &Node) {
        todo!()
    }

    fn get_code(&self) -> String {
        // TODO: For now prints, should return
        println!("{}", self.output);
        "".to_owned()
    }
}
