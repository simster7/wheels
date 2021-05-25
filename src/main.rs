mod lexer;
mod token;
mod parser;
mod ast;

use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;

fn main() {
    let lexer = Lexer::new(String::from(r#"
fn simon(a: int): int {
    return a;
}
    "#));

    let parser = &mut Parser::new(lexer);
    parser.program().expect("error");

    // while let Some(token) = lexer.next_token() {
    //     println!("{:?}", token)
    // }
}
