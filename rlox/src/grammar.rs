use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub enum Expr {
    LiteralExpr(LiteralExpr),
    UnaryExpr(UnaryExpr),
    BinaryExpr(BinaryExpr),
    GroupingExpr(GroupingExpr),
}

#[derive(PartialEq, Debug)]
pub struct LiteralExpr(pub Literal);

#[derive(PartialEq, Debug)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub expr: Box<Expr>,
}
#[derive(PartialEq, Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: BinaryOp,
    pub right: Box<Expr>,
}
#[derive(PartialEq, Debug)]
pub struct GroupingExpr(pub Box<Expr>);

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Number(f32),
    String(String),
    Boolean(bool),
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::String(s) => write!(f, "{}", s),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Nil => write!(f, "nil"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(PartialEq, Debug)]
pub enum BinaryOp {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Plus,
    Minus,
    Multiply,
    Divide,
}
