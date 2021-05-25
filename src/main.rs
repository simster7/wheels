mod lexer;
mod token;

use crate::lexer::lexer::Lexer;

fn main() {
    let lexer = &mut Lexer::new(String::from("var simon"));
    println!("{:?}", lexer);
    println!("{:?}", lexer.next_token());
    println!("{:?}", lexer.next_token());
}
