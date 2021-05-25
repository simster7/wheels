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
    Return,
    EOF
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
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