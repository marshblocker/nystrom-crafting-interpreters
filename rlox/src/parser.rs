// New Grammar:
// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary
//                | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")" ;

use crate::{
    grammar::{BinaryExpr, BinaryOp, Expr, Literal, LiteralExpr, UnaryExpr, UnaryOp},
    token::{
        Token,
        TokenType::{self, *},
    },
};

pub struct Parser {
    tokens: Vec<Token>,
    // Current token to be consumed.
    curr: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, curr: 0 }
    }

    pub fn parse(&mut self) -> Expr {
        self.parse_equality()
    }

    fn parse_binary_expr<F>(&mut self, parse_next: F, token_types: Vec<TokenType>) -> Expr
    where
        F: Fn(&mut Parser) -> Expr,
    {
        let mut left = parse_next(self);

        while self.match_type(&token_types) {
            let op = match &self.previous().typ {
                t if token_types.contains(t) => match t {
                    EqualEqual => BinaryOp::Equal,
                    BangEqual => BinaryOp::NotEqual,
                    Less => BinaryOp::LessThan,
                    LessEqual => BinaryOp::LessThanOrEqual,
                    Greater => BinaryOp::GreaterThan,
                    GreaterEqual => BinaryOp::GreaterThanOrEqual,
                    Minus => BinaryOp::Minus,
                    Plus => BinaryOp::Plus,
                    Slash => BinaryOp::Divide,
                    Star => BinaryOp::Multiply,
                    _ => panic!(),
                },
                _ => panic!(),
            };
            let right = parse_next(self);
            let expr = Expr::BinaryExpr(BinaryExpr {
                left: Box::new(left),
                op,
                right: Box::new(right),
            });
            left = expr;
        }

        left
    }

    fn parse_equality(&mut self) -> Expr {
        self.parse_binary_expr(Self::parse_comparison, vec![EqualEqual, BangEqual])
    }

    fn parse_comparison(&mut self) -> Expr {
        self.parse_binary_expr(
            Self::parse_term,
            vec![Less, LessEqual, Greater, GreaterEqual],
        )
    }

    fn parse_term(&mut self) -> Expr {
        self.parse_binary_expr(Self::parse_factor, vec![Plus, Minus])
    }

    fn parse_factor(&mut self) -> Expr {
        self.parse_binary_expr(Self::parse_unary, vec![Slash, Star])
    }

    fn parse_unary(&mut self) -> Expr {
        if self.match_type(&[Bang, Minus]) {
            let op = match self.previous().typ {
                Bang => UnaryOp::Not,
                Minus => UnaryOp::Negate,
                _ => panic!(),
            };
            let expr = self.parse_unary();
            return Expr::UnaryExpr(UnaryExpr {
                op,
                expr: Box::new(expr),
            });
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Expr {
        let token = self.advance();
        match &token.typ {
            Number(n) => Expr::LiteralExpr(LiteralExpr(Literal::Number(*n))),
            String(s) => Expr::LiteralExpr(LiteralExpr(Literal::String(s.clone()))),
            True => Expr::LiteralExpr(LiteralExpr(Literal::Boolean(true))),
            False => Expr::LiteralExpr(LiteralExpr(Literal::Boolean(false))),
            Nil => Expr::LiteralExpr(LiteralExpr(Literal::Nil)),
            LeftParen => {
                let expr = self.parse();
                if self.peek().typ != RightParen {
                    panic!();
                }

                // Consume ")"
                self.advance();

                expr
            }
            _ => panic!(),
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.curr]
    }

    fn match_type(&mut self, token_types: &[TokenType]) -> bool {
        for typ in token_types {
            if self.check(typ) {
                self.advance();
                return true
            }
        }

        false
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() { self.curr += 1; }
        self.previous()
    }

    fn check(&self, typ: &TokenType) -> bool {
        if self.is_at_end() { return false; }
        self.peek().typ == *typ
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.curr-1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().typ == TokenType::EOF
    }
}
