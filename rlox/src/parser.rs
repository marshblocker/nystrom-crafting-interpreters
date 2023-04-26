// expression     → literal
//                | unary
//                | binary
//                | grouping ;

// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
// grouping       → "(" expression ")" ;
// unary          → ( "-" | "!" ) expression ;
// binary         → expression operator expression ;
// operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
//                | "+"  | "-"  | "*" | "/" ;

use crate::{
    error_reporter::ErrorReporter,
    grammar::{BinaryExpr, BinaryOp, Expr, GroupingExpr, Literal, LiteralExpr, UnaryExpr, UnaryOp},
    token::{Token, TokenType},
};

pub struct Parser<'a> {
    tokens: Vec<Token>,
    next: usize,

    error_reporter: &'a mut ErrorReporter,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, error_reporter: &'a mut ErrorReporter) -> Parser {
        Parser {
            tokens,
            next: 0,
            error_reporter,
        }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        let save = self.next;

        if let Some(literal_expr) = self.parse_literal_expr() {
            return Some(literal_expr);
        }

        self.next = save;

        if let Some(grouping_expr) = self.parse_grouping_expr() {
            return Some(grouping_expr);
        }

        self.next = save;

        if let Some(unary_expr) = self.parse_unary_expr() {
            return Some(unary_expr);
        }

        self.next = save;

        if let Some(binary_expr) = self.parse_binary_expr() {
            return Some(binary_expr);
        }

        self.next = save;
        None
    }

    fn parse_literal_expr(&mut self) -> Option<Expr> {
        use TokenType::*;

        let token = self.get_next_token();

        match token.typ {
            Number(n) => {
                self.next += 1;
                Some(Expr::LiteralExpr(LiteralExpr(Literal::Number(n))))
            }
            String(s) => {
                self.next += 1;
                Some(Expr::LiteralExpr(LiteralExpr(Literal::String(s))))
            }
            True => {
                self.next += 1;
                Some(Expr::LiteralExpr(LiteralExpr(Literal::Boolean(true))))
            }
            False => {
                self.next += 1;
                Some(Expr::LiteralExpr(LiteralExpr(Literal::Boolean(false))))
            }
            Nil => {
                self.next += 1;
                Some(Expr::LiteralExpr(LiteralExpr(Literal::Nil)))
            }
            _ => None,
        }
    }

    fn parse_grouping_expr(&mut self) -> Option<Expr> {
        use TokenType::*;

        let mut token = self.get_next_token();
        if token.typ != LeftParen {
            return None;
        }

        self.next += 1;

        let expr = match self.parse() {
            Some(expr) => expr,
            None => return None,
        };

        token = self.get_next_token();
        if token.typ != RightParen {
            return None;
        }

        self.next += 1;

        Some(Expr::GroupingExpr(GroupingExpr(Box::new(expr))))
    }

    fn parse_unary_expr(&mut self) -> Option<Expr> {
        use TokenType::*;

        let token = self.get_next_token();

        match token.typ {
            Bang | Minus => {
                self.next += 1;
                if let Some(expr) = self.parse() {
                    let op = if token.typ == Bang {
                        UnaryOp::Not
                    } else {
                        UnaryOp::Negate
                    };
                    return Some(Expr::UnaryExpr(UnaryExpr {
                        op: op,
                        expr: Box::new(expr),
                    }));
                } else {
                    return None;
                }
            }
            _ => None,
        }
    }

    fn parse_binary_expr(&mut self) -> Option<Expr> {
        let left = match self.parse() {
            Some(expr) => expr,
            None => return None,
        };

        let token = self.get_next_token();
        let op = match token.typ {
            TokenType::EqualEqual => BinaryOp::Equal,
            TokenType::BangEqual => BinaryOp::NotEqual,
            TokenType::Less => BinaryOp::LessThan,
            TokenType::LessEqual => BinaryOp::LessThanOrEqual,
            TokenType::Greater => BinaryOp::GreaterThan,
            TokenType::GreaterEqual => BinaryOp::GreaterThanOrEqual,
            TokenType::Plus => BinaryOp::Plus,
            TokenType::Minus => BinaryOp::Minus,
            TokenType::Star => BinaryOp::Multiply,
            TokenType::Slash => BinaryOp::Divide,
            _ => return None,
        };

        self.next += 1;

        let right = match self.parse() {
            Some(expr) => expr,
            None => return None,
        };

        Some(Expr::BinaryExpr(BinaryExpr {
            left: Box::new(left),
            op: op,
            right: Box::new(right),
        }))
    }

    fn get_next_token(&self) -> Token {
        self.tokens[self.next].to_owned()
    }
}
