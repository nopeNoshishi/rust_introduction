pub mod lexer;
pub mod parser;
pub mod token;
pub mod error;

use crate::parser::Ast;

use std::io::{stdin, stdout, BufRead, BufReader, Result, Write};

fn main() -> Result<()> {
    let stdin = stdin();
    let stdin = stdin.lock();
    let stdin = BufReader::new(stdin);
    let mut lines = stdin.lines();

    loop {
        prompt("> ")?;
        let Some(Ok(line)) = lines.next() else {
            break;
        };

        let ast = match line.parse::<Ast>() {
            Ok(ast) => ast,
            Err(e) => {
                unimplemented!()
            }
        };
        println!("{:?}", ast);
    }

    Ok(())
}

fn prompt(s: &str) -> Result<()> {
    let stdout = stdout();
    let mut stdout = stdout.lock();
    stdout.write_all(s.as_bytes())?;
    stdout.flush()
}
