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
        let mut parser = Parser { lexer, current_token: Token::EOF };
        parser.next_token();
        parser
    }

    pub fn program(&mut self) -> Result<Node, ParseError> {
        self.function_def()
    }

    fn expect(&mut self, token_type: Token) -> Result<(), ParseError> {
        if self.current_token == token_type {
            return Ok(());
        }
        let found = self.current_token.to_string();
        return Err(ParseError::UnexpectedToken {pos: self.lexer.get_position(), expected: token_type.to_string(), found });
    }

    fn expect_one_of(&mut self, tokens: &[Token]) -> Result<(), ParseError> {
        let mut one_of = String::from("one of: ");
        for token in tokens.iter() {
            if self.current_token == *token {
                return Ok(());
            }
            one_of += token.to_string().as_str();
            one_of += ",";
        }
        let found = self.current_token.to_string();
        return Err(ParseError::UnexpectedToken { pos: self.lexer.get_position(), expected: one_of, found });
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token()
    }

    // FunctionDef = "fn" identifier FunctionSignature FunctionBody .
    fn function_def(&mut self) -> Result<Node, ParseError> {
        self.expect(Token::Function)?;
        self.next_token();

        let mut function_node = self.identifier(NodeType::Function)?;

        function_node.add_child(self.function_signature()?);
        function_node.add_child(self.block()?);

        println!("{}", function_node);

        Ok(function_node)
    }

    // FunctionSignature = "(" [ ParameterList ] ")" [ Result ] .
    fn function_signature(&mut self) -> Result<Node, ParseError> {
        self.expect(Token::LeftParenthesis)?;
        let mut function_signature_node = Node::new(NodeType::FunctionSignature);
        let mut parameter_list_node = Node::new(NodeType::ParameterList);
        self.next_token();

        // ParameterList = Parameter { "," Parameter }
        while self.current_token != Token::RightParenthesis {
            self.expect_one_of(&[Token::Identifier("".into()), Token::Comma])?;
            match self.current_token {
                Token::Comma => self.next_token(),
                _ => {
                    let parameter_node = self.parameter()?;
                    parameter_list_node.add_child(parameter_node);
                }
            }
        }

        function_signature_node.add_child(parameter_list_node);

        self.expect(Token::RightParenthesis)?;
        self.next_token();

        if self.current_token == Token::Colon {
            self.expect(Token::Colon)?;
            self.next_token();

            function_signature_node.add_child(self.identifier(NodeType::Type)?);
        }

        Ok(function_signature_node)
    }

    // Parameter = [ identifier ] ":" Type
    fn parameter(&mut self) -> Result<Node, ParseError> {
        let mut node = self.identifier(NodeType::Parameter)?;

        self.expect(Token::Colon)?;
        self.next_token();

        let mut parameter_type = self.identifier(NodeType::Type)?;
        node.add_child(parameter_type);

        Ok(node)
    }

    // Block = "{" StatementList "}" .
    fn block(&mut self) -> Result<Node, ParseError> {
        self.expect(Token::LeftBrace)?;
        self.next_token();

        let mut block_node = Node::new(NodeType::Block);

        // StatementList = { Statement ";" } .
        while self.current_token != Token::RightBrace {
            block_node.add_child(self.statement()?);

            self.expect(Token::SemiColon)?;
            self.next_token();
        }

        Ok(block_node)
    }

    // Statement = VarDecl .
    fn statement(&mut self) -> Result<Node, ParseError> {
        self.expect_one_of(&[Token::Var])?;
        match self.current_token {
            Token::Var => {
                self.var_decl()
            }
            _ => {
                Err(ParseError::UnexpectedToken { pos: self.lexer.get_position(), expected: "var".to_owned(), found: self.current_token.to_string() })
            }
        }
    }

    // VarDecl = "var" identifier ":" Type "=" Expression .
    fn var_decl(&mut self) -> Result<Node, ParseError> {
        self.expect(Token::Var)?;
        let mut var_decl_node = Node::new(NodeType::VarDecl);
        self.next_token();

        var_decl_node.add_child(self.identifier(NodeType::Identifier)?);

        self.expect(Token::Colon)?;
        self.next_token();

        var_decl_node.add_child(self.identifier(NodeType::Type)?);

        self.expect(Token::Equals)?;
        self.next_token();

        var_decl_node.add_child(self.expression()?);

        Ok(var_decl_node)
    }

    fn identifier(&mut self, node_type: NodeType) -> Result<Node, ParseError> {
        self.expect(Token::Identifier("".into()))?;
        let mut node = Node::new(node_type);
        node.add_token(self.current_token.clone());
        // TODO: Add ident to symbol table
        self.next_token();

        Ok(node)
    }

    fn integer(&mut self) -> Result<Node, ParseError> {
        self.expect(Token::Integer("".into()))?;
        let mut node = Node::new(NodeType::Integer);
        node.add_token(self.current_token.clone());
        self.next_token();

        Ok(node)
    }

    // Expression = UnaryExpression .
    fn expression(&mut self) -> Result<Node, ParseError> {
        // TODO: Obviously this is an incomplete implementation
        self.expect(Token::Integer("".into()))?;
        self.integer()
    }
}