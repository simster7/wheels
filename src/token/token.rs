use crate::token::token::Token::Identifier;

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    LeftBrace,
    RightBrace,
    LeftParenthesis,
    RightParenthesis,
    Colon,
    SemiColon,
    Integer(isize),
    Function,
    Var,
}

pub fn lookup_literal(literal: String) -> Token {
    match literal.as_str() {
        "var" => Token::Var,
        "fn" => Token::Function,
        _ => Identifier(literal)
    }
}