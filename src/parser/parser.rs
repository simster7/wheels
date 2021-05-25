use crate::lexer::lexer::Lexer;
use crate::token::token::Token;
use crate::ast::ast::{Node, NodeType};
use crate::parser::errors::ParserError;

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

    fn consume(&mut self) {
        self.next_token()
    }

    fn expect(&mut self, token_type: &Token) -> Result<(), ParserError> {
        // TODO: This should certainly be in Token
        if matches!(self.current_token.as_ref().unwrap(), token_type) {
            return Ok(())
        }
        return Err(ParserError::UnexpectedToken)
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token()
    }

    // FunctionDef = "fn" identifier FunctionSignature FunctionBody .
    fn function_def(&mut self) -> Result<Node, ParserError> {

        self.expect(&Token::Identifier(String::from("")))?;
        let mut node = Node::new(NodeType::Identifier, None);
        self.consume();

        println!("node {:?}", node);

        Ok(node)
    }
}