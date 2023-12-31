use crate::token::*;

pub fn lexer(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();

    let input = input.as_bytes();
    let mut pos = 0;

    while pos < input.len() {
        // 区切り文字は先に処理
        if [b' ', b'\n', b'\t'].contains(&input[pos]) {
            let p = lex_spaces(input, pos)?;
            pos = p;

            continue;
        }

        //  数字か記号か
        let (token, p) = match input[pos] {
            b'0'..=b'9' => lex_number(input, pos)?,
            _ => lex_symbol(input, pos)?,
        };

        tokens.push(token);
        pos = p;
    }

    Ok(tokens)
}

// クロージャーに適応するcharの連続性を確認
fn recognize_many(input: &[u8], mut pos: usize, mut f: impl FnMut(u8) -> bool) -> usize {
    while pos < input.len() && f(input[pos]) {
        pos += 1;
    }

    pos
}

fn consume_byte(input: &[u8], pos: usize, b: u8) -> Result<(u8, usize), LexError> {
    if input.len() <= pos {
        return Err(LexError::eof(Location::new(pos, pos)));
    }

    if input[pos] != b {
        return Err(LexError::invalid_char(
            input[pos] as char,
            Location::new(pos, pos + 1),
        ));
    }

    Ok((b, pos + 1))
}

fn lex_spaces(input: &[u8], pos: usize) -> Result<usize, LexError> {
    let start = pos;
    let end = recognize_many(input, start, |b| b" \n\t".contains(&b));

    Ok(end)
}

fn lex_number(input: &[u8], pos: usize) -> Result<(Token, usize), LexError> {
    use std::str::from_utf8;

    let start = pos;
    let end = recognize_many(input, start, |b| b"1234567890".contains(&b));

    let n = from_utf8(&input[start..end]).unwrap().parse().unwrap();

    Ok((Token::number(n, Location::new(start, end)), end))
}

fn lex_symbol(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    match input[start] {
        b'+' => consume_byte(input, start, input[start])
            .map(|(_, end)| (Token::plus(Location::new(start, end)), end)),
        b'-' => consume_byte(input, start, input[start])
            .map(|(_, end)| (Token::minus(Location::new(start, end)), end)),
        b'*' => consume_byte(input, start, input[start])
            .map(|(_, end)| (Token::asterisk(Location::new(start, end)), end)),
        b'/' => consume_byte(input, start, input[start])
            .map(|(_, end)| (Token::slash(Location::new(start, end)), end)),
        b'(' => consume_byte(input, start, input[start])
            .map(|(_, end)| (Token::lparen(Location::new(start, end)), end)),
        b')' => consume_byte(input, start, input[start])
            .map(|(_, end)| (Token::rparen(Location::new(start, end)), end)),
        b => Err(LexError::invalid_char(
            b as char,
            Location::new(start, start + 1),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let result = lexer("1 + 2 * 3 - - 10");
        assert!(result.is_ok());

        let test_tokens = vec![
            Token::number(1, Location::new(0, 1)),
            Token::plus(Location::new(2, 3)),
            Token::number(2, Location::new(4, 5)),
            Token::asterisk(Location::new(6, 7)),
            Token::number(3, Location::new(8, 9)),
            Token::minus(Location::new(10, 11)),
            Token::minus(Location::new(12, 13)),
            Token::number(10, Location::new(14, 16)),
        ];

        assert_eq!(result, Ok(test_tokens));
    }

    #[test]
    fn test_debug() {
        if [b' ', b'\n', b'\t'].contains(&b'\t') {
            println!("hekk")
        }
    }
}
