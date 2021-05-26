mod lexer;
mod token;
mod parser;
mod ast;

use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;

fn main() {
    let lexer = Lexer::new(String::from(r#"
fn simon(a: int, b: int): int {
    var sum: int = a + b;
    var two_sum: int = sum + 2.7;
    return two_sum;
}
    "#));

    let parser = &mut Parser::new(lexer);
    parser.program().expect("error");

    // while let Some(token) = lexer.next_token() {
    //     println!("{:?}", token)
    // }
}
