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
    Assignment,
    Plus,
    Minus,
    Times,
    DividedBy,
    And,
    BitwiseAnd,
    Or,
    BitwiseOr,
    Not,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    Equal,
    NotEqual,
    EOF,
}

impl Token {
    pub fn get_literal(&self) -> &str {
        let out = match self {
            Token::Identifier(literal) => literal,
            Token::Integer(literal) => literal,
            Token::Float(literal) => literal,
            _ => panic!("token without literal"),
        };
        out.as_str()
    }
}

impl From<String> for Token {
    fn from(string: String) -> Self {
        match string.as_str() {
            "var" => Token::Var,
            "fn" => Token::Function,
            "return" => Token::Return,
            _ => Token::Identifier(string),
        }
    }
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
