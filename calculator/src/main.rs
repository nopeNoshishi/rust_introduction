
pub mod token;
pub mod lexer;

use lexer::*;

fn main() {
    let tokens = lexer("1 + 2 * 3 - - 10").unwrap();
    println!("{:?}", tokens);
}
