mod ast;
mod emitter;
mod lexer;
mod parser;
mod token;

use crate::emitter::emitter::Emitter;
use crate::emitter::python_emitter::PythonEmitter;
use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;

fn main() {
    let lexer = Lexer::new(String::from(
        r#"
fn simon(a: int, b: int): int {
    var sum: int = a + b;
    var two_sum: int = sum - 2.7;
    return two_sum;
}
    "#,
    ));

    let parser = &mut Parser::new(lexer);
    let root = parser.program().expect("error");
    println!("{}", root);

    let python = PythonEmitter::new(root);
    let program = python.program();

    println!("{}", program);

    // while let Some(token) = lexer.next_token() {
    //     println!("{:?}", token)
    // }
}
