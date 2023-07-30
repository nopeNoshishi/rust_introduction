use std::str::FromStr;
use std::iter::Peekable;

use crate::token::*;  
use crate::lexer::lexer;
use crate::error::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UniOpKind {
    Plus,
    Minus,
}

type UniOp = Annotation<UniOpKind>;

impl UniOp {
    pub fn plus(loc: Location) -> Self {
        Self::new(UniOpKind::Plus, loc)
    }

    pub fn minus(loc: Location) -> Self {
        Self::new(UniOpKind::Minus, loc)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
}

type BinOp = Annotation<BinOpKind>;

impl BinOp {
    pub fn add(loc: Location) -> Self {
        Self::new(BinOpKind::Add, loc)
    }

    pub fn sub(loc: Location) -> Self {
        Self::new(BinOpKind::Sub, loc)
    }

    pub fn mul(loc: Location) -> Self {
        Self::new(BinOpKind::Mul, loc)
    }

    pub fn div(loc: Location) -> Self {
        Self::new(BinOpKind::Div, loc)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstKind {
    Num(u64),
    UniOp { op: UniOp, e: Box<Ast> },
    BinOp { op: BinOp, l: Box<Ast>, r: Box<Ast> },
}

pub type Ast = Annotation<AstKind>;

impl Ast {
    pub fn num(n: u64, loc: Location) -> Self {
        Self::new(AstKind::Num(n), loc)
    }

    pub fn uniop(op: UniOp, e: Ast, loc: Location) -> Self {
        Self::new(AstKind::UniOp { op, e: Box::new(e) }, loc)
    }

    pub fn binop(op: BinOp, l: Ast, r: Ast, loc: Location) -> Self {
        Self::new(
            AstKind::BinOp {
                op,
                l: Box::new(l),
                r: Box::new(r),
            },
            loc,
        )
    }
}

impl FromStr for Ast {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = lexer(s)?;
        let ast = parse(tokens)?;
        Ok(ast)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParserError {
    UnexpectedToken(Token),
    NotExpression(Token),
    NotOperator(Token),
    UnclosedOpenParen(Token),
    RedundantExpression(Token),
    Eof,
}

pub fn parse(tokens: Vec<Token>) -> Result<Ast, ParserError> {
    let mut tokens = tokens.into_iter().peekable();

    let ret = parse_entry(&mut tokens)?;

    match tokens.next() {
        Some(t) => Err(ParserError::RedundantExpression(t)),
        None => Ok(ret),
    }
}

fn parse_entry<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Result<Ast, ParserError> {
    parse_expr3(tokens)
}

fn parse_expr3<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Result<Ast, ParserError> {
    parse_left_binop(tokens, parse_expr2, |tokens: &mut Peekable<I>| {
        let op = tokens
            .peek()
            .ok_or(ParserError::Eof)
            .and_then(|t| match t.value() {
                TokenKind::Plus => Ok(BinOp::add(t.loc())),
                TokenKind::Minus => Ok(BinOp::sub(t.loc())),
                _ => Err(ParserError::NotOperator(t.clone())),
            })?;
        tokens.next();
        Ok(op)
    })
}

fn parse_expr2<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Result<Ast, ParserError> {
    parse_left_binop(tokens, parse_expr1, |tokens: &mut Peekable<I>| {
        let op = tokens
            .peek()
            .ok_or(ParserError::Eof)
            .and_then(|t| match t.value() {
                TokenKind::Asterisk => Ok(BinOp::mul(t.loc())),
                TokenKind::Slash => Ok(BinOp::div(t.loc())),
                _ => Err(ParserError::NotOperator(t.clone())),
            })?;
        tokens.next();
        Ok(op)
    })
}

fn parse_expr1<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Result<Ast, ParserError> {
    match tokens.peek().map(|t| t.value()) {
        Some(TokenKind::Plus) => {
            let loc = tokens.next().unwrap().loc();
            let op = UniOp::plus(loc);
            let e = parse_atom(tokens)?;
            let loc = op.loc().merge(&e.loc());
            Ok(Ast::uniop(op, e, loc))
        }

        Some(TokenKind::Minus) => {
            let loc = tokens.next().unwrap().loc();
            let op = UniOp::minus(loc);
            let e = parse_atom(tokens)?;
            let loc = op.loc().merge(&e.loc());
            Ok(Ast::uniop(op, e, loc))
        }
        _ => parse_atom(tokens),
    }
}

fn parse_atom<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Result<Ast, ParserError> {
    tokens
        .next()
        .ok_or(ParserError::Eof)
        .and_then(|t| match t.value() {
            TokenKind::Number(n) => Ok(Ast::new(AstKind::Num(n), t.loc())),
            TokenKind::LParen => {
                let e = parse_entry(tokens)?;
                match tokens.next() {
                    Some(Token {
                        value: TokenKind::RParen,
                        ..
                    }) => Ok(e),
                    Some(t) => Err(ParserError::RedundantExpression(t)),
                    _ => Err(ParserError::UnclosedOpenParen(t)),
                }
            }
            _ => Err(ParserError::NotExpression(t)),
        })
}

fn parse_left_binop<I: Iterator<Item = Token>>(
    tokens: &mut Peekable<I>,
    subexpr_parser: fn(&mut Peekable<I>) -> Result<Ast, ParserError>,
    or_parser: fn(&mut Peekable<I>) -> Result<BinOp, ParserError>,
) -> Result<Ast, ParserError> {
    let mut l = subexpr_parser(tokens)?;

    loop {
        match tokens.peek() {
            Some(_) => {
                let op = match or_parser(tokens) {
                    Ok(op) => op,
                    Err(_) => break,
                };

                let r = subexpr_parser(tokens)?;
                let loc = l.loc().merge(&r.loc());
                l = Ast::binop(op, l, r, loc)
            }
            _ => break,
        }
    }

    Ok(l)
}
