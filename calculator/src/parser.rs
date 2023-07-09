use crate::token::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UniOpKind {
    Plus,
    Minus
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
    Div
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
    BinOp { op: BinOp, l: Box<Ast>, r: Box<Ast> }
}

type Ast = Annotation<AstKind>;

impl Ast {
    pub fn num(n: u64, loc: Location) -> Self {
        Self::new(AstKind::Num(n), loc)
    }

    pub fn uniop(op: UniOp, e: Ast, loc: Location) -> Self {
        Self::new(AstKind::UniOp { op, e: Box::new(e) }, loc)
    }

    pub fn binop(op: BinOp, l: Ast, r: Ast, loc: Location) -> Self {
        Self::new(AstKind::BinOp { op, l: Box::new(l), r: Box::new(r) },  loc)
    }
}

pub enum ParserError {
    UnexpectedToken(Token),
    NotExpression(Token),
    NotOperator(Token),
    UnclosedOpenParen(Token),
    RedundantExpression(Token),
    Eof
}
