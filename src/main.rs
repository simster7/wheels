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
fn simon(a: int, b: int, c: float): int {
    var sum: int = a + b * c ;
    var two_sum: float = sum - 2.7;
    return two_sum * c;
}
    "#,
    ));

    let parser = &mut Parser::new(lexer);
    let root = parser.program().expect("error");
    println!("{}", root);

    let mut python = PythonEmitter::new();
    python.program(&root);

    python.get_code();

    // while let Some(token) = lexer.next_token() {
    //     println!("{:?}", token)
    // }
}
