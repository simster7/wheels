use crate::ast::ast::{Node, NodeType};
use crate::emitter::emitter::Emitter;
use crate::token::token::Token;

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

        self.output += format!("def {}", node.must_get_token_ref().get_literal()).as_str();

        self.function_signature(node.get_child(0));
        self.new_line_with_indent();

        self.block(node.get_child(1));

        self.new_line_with_back()
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
                child.must_get_token_ref().get_literal()
            })
            .collect::<Vec<&str>>()
            .join(", ")
            .as_str();

        self.output += "):";
    }

    fn block(&mut self, node: &Node) {
        node.ensure_type(NodeType::Block);

        for statement in node.get_children().iter() {
            self.statement(statement)
        }
    }

    fn statement(&mut self, node: &Node) {
        match node.get_type() {
            NodeType::VarDecl => self.var_decl(node),
            NodeType::Jump => self.jump(node),
            _ => panic!("unknown statement type"),
        }
    }

    fn var_decl(&mut self, node: &Node) {
        node.ensure_type(NodeType::VarDecl);

        self.output += node.get_child(0).must_get_token_ref().get_literal();
        self.output += " = ";

        self.expression(node.get_child(2));
        self.new_line()
    }

    fn jump(&mut self, node: &Node) {
        node.ensure_type(NodeType::Jump);

        let jump_literal = match node.must_get_token_ref() {
            Token::Return => "return",
            _ => panic!("unknown jump literal"),
        };

        self.output += format!("{} ", jump_literal).as_str();

        self.expression(node.get_child(0));

        self.new_line()
    }

    fn expression(&mut self, node: &Node) {
        node.ensure_type(NodeType::Expression);
        let expression = node.get_child(0);
        match expression.get_type() {
            NodeType::BinaryOperation => self.binary_operation(expression),
            NodeType::VarReference => {
                self.output += expression.must_get_token_ref().get_literal();
            }
            _ => panic!("unknown expression type"),
        }
    }

    fn binary_operation(&mut self, node: &Node) {
        node.ensure_type(NodeType::BinaryOperation);

        self.expression(node.get_child(0));

        let binary_operator = PythonEmitter::get_token_symbol(node.must_get_token_ref());
        self.output += format!(" {} ", binary_operator).as_str();

        self.expression(node.get_child(1));
    }

    fn operand(&mut self, node: &Node) {
        match node.get_type() {
            NodeType::Integer => (),
            NodeType::Float => (),
            NodeType::VarReference => (),
            _ => panic!("unknown operand"),
        }

        self.output += node.must_get_token_ref().get_literal()
    }

    fn get_token_symbol(token: &Token) -> &str {
        match *token {
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Times => "*",
            Token::DividedBy => "/",
            _ => panic!("unknown symbol"),
        }
    }

    fn get_code(&self) -> String {
        // TODO: For now prints, should return
        println!("{}", self.output);
        "".to_owned()
    }
}
