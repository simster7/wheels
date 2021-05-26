use crate::lexer::lexer::Lexer;
use crate::token::token::Token;
use crate::ast::ast::{Node, NodeType};
use crate::parser::errors::ParserError;
use std::borrow::Borrow;

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>
    // TODO: Likely will need peek_token
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {lexer, current_token: None};
        parser.next_token();
        parser
    }

    pub fn program(&mut self) -> Result<Node, ParserError> {
        self.function_def()
    }

    fn expect(&mut self, token_type: &Token) -> Result<(), ParserError> {
        if self.current_token.as_ref().unwrap() == token_type {
            return Ok(())
        }
        let found = self.current_token.as_ref().unwrap().to_string();
        return Err(ParserError::UnexpectedToken{expected: token_type.to_string(), found})
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token()
    }

    // FunctionDef = "fn" identifier FunctionSignature FunctionBody .
    fn function_def(&mut self) -> Result<Node, ParserError> {
        self.expect(&Token::Function)?;
        let mut function_node = Node::new(NodeType::Function);
        self.next_token();

        self.expect(&Token::Identifier(String::from("")))?;
        let mut node = Node::new(NodeType::Identifier);
        node.add_token(self.current_token.as_ref().unwrap().clone());
        function_node.add_child(node);
        // TODO: Add ident to symbol table
        self.next_token();

        let function_signature = self.function_signature()?;
        function_node.add_child(function_signature);

        println!("{}", function_node);

        Ok(function_node)
    }

    // FunctionSignature = "(" [ ParameterList ] ")" [ Result ] .
    fn function_signature(&mut self) -> Result<Node, ParserError> {
        self.expect(&Token::LeftParenthesis)?;
        let mut parameter_list_node = Node::new(NodeType::ParameterList);
        self.next_token();

        while self.current_token.as_ref().unwrap() != &Token::RightParenthesis {
            let parameter_node = self.parameter()?;
            parameter_list_node.add_child(parameter_node);
        }

        Ok(parameter_list_node)
    }

    // Parameter = [ identifier ] ":" Type
    fn parameter(&mut self) -> Result<Node, ParserError> {
        self.expect(&Token::Identifier(String::from("")))?;
        let mut node = Node::new(NodeType::Identifier);
        node.add_token(self.current_token.as_ref().unwrap().clone());
        self.next_token();

        self.expect(&Token::Colon)?;
        self.next_token();

        self.expect(&Token::Identifier(String::from("")))?;
        let mut parameter_type = Node::new(NodeType::Identifier);
        parameter_type.add_token(self.current_token.as_ref().unwrap().clone());
        node.add_child(parameter_type);
        self.next_token();

        Ok(node)
    }
}