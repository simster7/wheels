use crate::token::token::Token;

#[derive(Debug)]
pub struct Lexer {
    position: usize,
    read_position: usize,
    char: char,
    characters: Vec<char>,
    line: usize,
    column_start: usize,
    column: usize,
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
            column_start: 0,
            column: 0,
        };
        lexer.read_char();
        return lexer;
    }

    pub fn get_position(&self) -> (usize, usize, usize) {
        return (self.line, self.column_start, self.column);
    }

    fn read_char(&mut self) {
        if self.read_position >= self.characters.len() {
            self.char = '\0'
        } else {
            self.char = self.characters[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
        self.column += 1;

        if self.char == '\n' {
            self.line += 1;
            self.column = 0;
        }
    }

    pub fn next_token(&mut self) -> Token {
        // TODO: Skip comments
        self.skip_whitespace();

        self.column_start = self.column;
        let res = match self.char {
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            ':' => Token::Colon,
            ';' => Token::SemiColon,
            ',' => Token::Comma,
            '=' => Token::Equals,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '\0' => Token::EOF,
            _ => {
                if is_digit(self.char) {
                    let integer = self.read_number();
                    return if self.char == '.' {
                        self.read_char();
                        let decimal = self.read_number();
                        // TODO: Check float ending here. Currently `2.34a` would be a valid float (2.34)
                        Token::Float(integer + "." + decimal.as_str())
                    } else {
                        Token::Integer(integer)
                    };
                }
                return self.read_literal().into();
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
        self.read_fn(&is_literal)
    }

    fn read_number(&mut self) -> String {
        self.read_fn(&is_digit)
    }

    fn read_fn(&mut self, test: &dyn Fn(char) -> bool) -> String {
        let start: usize = self.position;
        while test(self.char) {
            self.read_char()
        }
        self.characters[start..self.position].iter().collect()
    }
}

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\n' || c == '\r'
}

fn is_literal(c: char) -> bool {
    is_letter(c) || is_digit(c) || c == '_'
}

fn is_letter(c: char) -> bool {
    ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z')
}

fn is_digit(c: char) -> bool {
    '0' <= c && c <= '9'
}
