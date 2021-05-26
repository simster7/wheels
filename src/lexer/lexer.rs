use crate::token::token::{Token, lookup_literal};

#[derive(Debug)]
pub struct Lexer {
    position: usize,
    read_position: usize,
    char: char,
    characters: Vec<char>,
    line: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let char_vec: Vec<char> = input.chars().collect();
        let mut lexer = Lexer {
            position: 0,
            read_position: 0,
            char: '\0',
            characters: char_vec,
            line: 1,
        };
        lexer.read_char();
        return lexer;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.characters.len() {
            self.char = '\0'
        } else {
            self.char = self.characters[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;

        if self.char == '\n' {
            self.line += 1
        }
    }

    pub fn next_token(&mut self) -> Token {
        // TODO: Skip whitespace and comments
        self.skip_whitespace();
        let res = match self.char {
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            ':' => Token::Colon,
            ';' => Token::SemiColon,
            ',' => Token::Comma,
            '\0' => Token::EOF,
            _ => {
                return lookup_literal(self.read_literal());
            }
        };
        self.read_char();
        return res;
    }

    fn skip_whitespace(&mut self) {
        while is_whitespace(self.char) {
            self.read_char()
        }
    }

    fn read_literal(&mut self) -> String {
        let start: usize = self.position;
        while is_literal(self.char) {
            self.read_char()
        }
        self.characters[start..self.position].iter().collect()
    }
}

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\n' || c == '\r'
}

fn is_literal(c: char) -> bool {
    is_letter(c) || is_digit(c)
}

fn is_letter(c: char) -> bool {
    ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z')
}

fn is_digit(c: char) -> bool {
    '0' <= c && c <= '9'
}