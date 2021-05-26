use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    Integer(String),
    Float(String),
    LeftBrace,
    RightBrace,
    LeftParenthesis,
    RightParenthesis,
    Colon,
    SemiColon,
    Function,
    Var,
    Return,
    Comma,
    Equals,
    Plus,
    EOF,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

pub fn lookup_literal(literal: String) -> Token {
    match literal.as_str() {
        "var" => Token::Var,
        "fn" => Token::Function,
        "return" => Token::Return,
        _ => Token::Identifier(literal)
    }
}