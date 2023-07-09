
pub mod token;
pub mod lexer;
pub mod parser;

use lexer::*;

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

        let token = lexer(&line);
        println!("{:?}", token);
    }

    Ok(())
}

fn prompt(s: &str) -> Result<()> {
    let stdout = stdout();
    let mut stdout = stdout.lock();
    stdout.write(s.as_bytes())?;
    stdout.flush()

}
