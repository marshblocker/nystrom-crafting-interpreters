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
    error_reporter::ErrorReporter,
    grammar::{BinaryExpr, BinaryOp, Expr, Literal, LiteralExpr, UnaryExpr, UnaryOp},
    token::{
        Token,
        TokenType::{self, *},
    },
};

pub struct Parser<'a> {
    tokens: Vec<Token>,
    // Current token to be consumed.
    curr: usize,

    error_reporter: &'a mut ErrorReporter,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, error_reporter: &'a mut ErrorReporter) -> Self {
        Self {
            tokens,
            curr: 0,
            error_reporter,
        }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.parse_equality()
    }

    fn parse_binary_expr<F>(&mut self, parse_next: F, token_types: Vec<TokenType>) -> Option<Expr>
    where
        F: Fn(&mut Parser<'a>) -> Option<Expr>,
    {
        let mut left = match parse_next(self) {
            Some(e) => e,
            None => return None,
        };

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
                    _ => {
                        self.error(self.peek().clone(), "Expected a binary operator.");
                        return None;
                    }
                },
                _ => unreachable!(),
            };
            let right = match parse_next(self) {
                Some(e) => e,
                None => return None,
            };
            let expr = Expr::BinaryExpr(BinaryExpr {
                left: Box::new(left),
                op,
                right: Box::new(right),
            });
            left = expr;
        }

        Some(left)
    }

    fn parse_equality(&mut self) -> Option<Expr> {
        self.parse_binary_expr(Self::parse_comparison, vec![EqualEqual, BangEqual])
    }

    fn parse_comparison(&mut self) -> Option<Expr> {
        self.parse_binary_expr(
            Self::parse_term,
            vec![Less, LessEqual, Greater, GreaterEqual],
        )
    }

    fn parse_term(&mut self) -> Option<Expr> {
        self.parse_binary_expr(Self::parse_factor, vec![Plus, Minus])
    }

    fn parse_factor(&mut self) -> Option<Expr> {
        self.parse_binary_expr(Self::parse_unary, vec![Slash, Star])
    }

    fn parse_unary(&mut self) -> Option<Expr> {
        if self.match_type(&[Bang, Minus]) {
            let op = match self.previous().typ {
                Bang => UnaryOp::Not,
                Minus => UnaryOp::Negate,
                _ => unreachable!(),
            };
            let expr = match self.parse_unary() {
                Some(e) => e,
                None => return None,
            };
            return Some(Expr::UnaryExpr(UnaryExpr {
                op,
                expr: Box::new(expr),
            }));
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        if self.peek().typ == EOF {
            self.error(self.peek().clone(), "Expected a literal or '('.");
            return None;
        }
        let token = self.advance();
        match &token.typ {
            Number(n) => Some(Expr::LiteralExpr(LiteralExpr(Literal::Number(*n)))),
            String(s) => Some(Expr::LiteralExpr(LiteralExpr(Literal::String(s.clone())))),
            True => Some(Expr::LiteralExpr(LiteralExpr(Literal::Boolean(true)))),
            False => Some(Expr::LiteralExpr(LiteralExpr(Literal::Boolean(false)))),
            Nil => Some(Expr::LiteralExpr(LiteralExpr(Literal::Nil))),
            LeftParen => {
                let expr = match self.parse() {
                    Some(e) => e,
                    None => {
                        self.error(self.peek().clone(), "Expected an expression.");
                        return None;
                    }
                };
                if self.peek().typ != RightParen {
                    self.error(self.peek().clone(), "Expected ')'.");
                    return None;
                }

                // Consume ")"
                self.advance();

                Some(expr)
            }
            _ => {
                self.error(self.peek().clone(), "Expected a literal or '('.");
                None
            }
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.curr]
    }

    fn match_type(&mut self, token_types: &[TokenType]) -> bool {
        for typ in token_types {
            if self.check(typ) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.curr += 1;
        }
        self.previous()
    }

    fn check(&self, typ: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().typ == *typ
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.curr - 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().typ == TokenType::EOF
    }

    fn error(&mut self, token: Token, message: &str) -> Option<()> {
        self.error_reporter
            .parse_error(&token, message, exitcode::DATAERR);
        None
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().typ == SemiColon {
                return;
            }

            match self.peek().typ {
                Class | Fun | Var | For | If | While | Print | Return => return,
                _ => (),
            }

            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() {
        let expr = get_expr(vec![Token {
            lexeme: "".to_string(),
            line: 1,
            typ: EOF,
        }]);

        assert_eq!(expr, None);
    }

    #[test]
    fn parse_missing_rparen() {
        let expr = get_expr(vec![
            Token {
                lexeme: "(".to_string(),
                line: 1,
                typ: LeftParen,
            },
            Token {
                lexeme: "1".to_string(),
                line: 1,
                typ: Number(1.0),
            },
            Token {
                lexeme: "+".to_string(),
                line: 1,
                typ: Plus,
            },
            Token {
                lexeme: "2".to_string(),
                line: 1,
                typ: Number(2.0),
            },
            Token {
                lexeme: "".to_string(),
                line: 1,
                typ: EOF,
            },
        ]);

        assert_eq!(expr, None);
    }

    fn get_expr(tokens: Vec<Token>) -> Option<Expr> {
        let mut error_reporter = ErrorReporter::default();
        let mut parser = Parser::new(tokens, &mut error_reporter);
        parser.parse()
    }
}
