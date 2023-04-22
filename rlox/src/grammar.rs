#[derive(PartialEq, Debug)]
pub enum Expr {
    LiteralExpr(Literal),
    UnaryExpr {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    BinaryExpr {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    GroupingExpr(Box<Expr>),
}

#[derive(PartialEq, Debug)]
pub enum Literal {
    Number(f32),
    String(String),
    True,
    False,
    Nil,
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