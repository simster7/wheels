use crate::lexer::lexer::Lexer;
use crate::token::token::Token;
use crate::ast::ast::{Node, NodeType};
use crate::parser::errors::ParseError;
use std::borrow::Borrow;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    // TODO: Likely will need peek_token
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {lexer, current_token: Token::EOF};
        parser.next_token();
        parser
    }

    pub fn program(&mut self) -> Result<Node, ParseError> {
        self.function_def()
    }

    fn expect(&mut self, token_type: Token) -> Result<(), ParseError> {
        if self.current_token == token_type {
            return Ok(())
        }
        let found = self.current_token.to_string();
        return Err(ParseError::UnexpectedToken{expected: token_type.to_string(), found})
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token()
    }

    // FunctionDef = "fn" identifier FunctionSignature FunctionBody .
    fn function_def(&mut self) -> Result<Node, ParseError> {
        self.expect(Token::Function)?;
        let mut function_node = Node::new(NodeType::Function);
        self.next_token();

        self.expect(Token::Identifier(String::from("")))?;
        let mut node = Node::new(NodeType::Identifier);
        node.add_token(self.current_token.clone());
        function_node.add_child(node);
        // TODO: Add ident to symbol table
        self.next_token();

        let function_signature = self.function_signature()?;
        function_node.add_child(function_signature);

        println!("{}", function_node);

        Ok(function_node)
    }

    // FunctionSignature = "(" [ ParameterList ] ")" [ Result ] .
    fn function_signature(&mut self) -> Result<Node, ParseError> {
        self.expect(Token::LeftParenthesis)?;
        let mut function_signature_node = Node::new(NodeType::FunctionSignature);
        let mut parameter_list_node = Node::new(NodeType::ParameterList);
        function_signature_node.add_child(parameter_list_node);
        self.next_token();

        while self.current_token != Token::RightParenthesis {
            let parameter_node = self.parameter()?;
            parameter_list_node.add_child(parameter_node);

            if self.current_token == Token::Comma {
                self.expect(Token::Comma)?;
                self.next_token();
            }
        }

        self.expect(Token::RightParenthesis)?;
        self.next_token();

        if self.current_token == Token::Colon {
            self.expect(Token::Colon)?;
            self.next_token();

            self.expect(Token::Identifier(String::from("")))?;
            let mut return_node = Node::new(NodeType::Identifier);
            return_node.add_token(self.current_token.clone());
            function_signature_node.add_child(return_node);
            self.next_token();
        }

        Ok(function_signature_node)
    }

    // Parameter = [ identifier ] ":" Type
    fn parameter(&mut self) -> Result<Node, ParseError> {
        self.expect(Token::Identifier(String::from("")))?;
        let mut node = Node::new(NodeType::Identifier);
        node.add_token(self.current_token.clone());
        self.next_token();

        self.expect(Token::Colon)?;
        self.next_token();

        self.expect(Token::Identifier(String::from("")))?;
        let mut parameter_type = Node::new(NodeType::Identifier);
        parameter_type.add_token(self.current_token.clone());
        node.add_child(parameter_type);
        self.next_token();

        Ok(node)
    }
}