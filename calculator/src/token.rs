#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Location(usize, usize);

impl Location {
    pub fn new(start: usize, end: usize) -> Self {
        Self(start, end)
    }
    pub fn merge(&self, other: &Location) -> Location {
        use std::cmp::{max, min};
        Location(min(self.0, other.0), max(self.1, other.1))
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Annotation<T> {
    pub value: T,
    loc: Location,
}

impl<T: Clone> Annotation<T> {
    pub fn new(value: T, loc: Location) -> Self {
        Self { value, loc }
    }

    pub fn value(&self) -> T {
        self.value.clone()
    }

    pub fn loc(&self) -> Location {
        self.loc.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Number(u64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::TokenKind::*;
        match self {
            Number(n) => n.fmt(f),
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Asterisk => write!(f, "*"),
            Slash => write!(f, "/"),
            LParen => write!(f, "("),
            RParen => write!(f, ")"),
        }
    }
}

pub type Token = Annotation<TokenKind>;

impl Token {
    pub fn number(n: u64, loc: Location) -> Self {
        Self::new(TokenKind::Number(n), loc)
    }

    pub fn plus(loc: Location) -> Self {
        Self::new(TokenKind::Plus, loc)
    }

    pub fn minus(loc: Location) -> Self {
        Self::new(TokenKind::Minus, loc)
    }

    pub fn asterisk(loc: Location) -> Self {
        Self::new(TokenKind::Asterisk, loc)
    }

    pub fn slash(loc: Location) -> Self {
        Self::new(TokenKind::Slash, loc)
    }

    pub fn lparen(loc: Location) -> Self {
        Self::new(TokenKind::LParen, loc)
    }

    pub fn rparen(loc: Location) -> Self {
        Self::new(TokenKind::RParen, loc)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LexErrorKind {
    InvalidChar(char),
    Eof,
}

pub type LexError = Annotation<LexErrorKind>;

impl LexError {
    pub fn invalid_char(c: char, loc: Location) -> Self {
        LexError::new(LexErrorKind::InvalidChar(c), loc)
    }

    pub fn eof(loc: Location) -> Self {
        LexError::new(LexErrorKind::Eof, loc)
    }
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_merge() {
        let loc1 = Location(1, 6);
        let loc2 = Location(2, 3);

        let merged_loc = loc1.merge(&loc2);

        assert_eq!(merged_loc, Location(1, 6))
    }

    #[test]
    fn test_annotation_new() {
        let loc = Location(0, 1);
        let anno = Annotation::<String>::new("+".to_string(), loc);

        println!("{:?}", anno);
    }
}
