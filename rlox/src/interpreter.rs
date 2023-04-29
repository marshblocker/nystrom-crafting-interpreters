use crate::{
    error_reporter::RuntimeError,
    grammar::{BinaryExpr, Expr, GroupingExpr, Literal, LiteralExpr, UnaryExpr},
    token::TokenType,
    visitor::Visitor,
};

pub struct Interpreter;

impl Visitor<Result<Literal, RuntimeError>> for Interpreter {
    fn visit_expr(&self, expr: &Expr) -> Result<Literal, RuntimeError> {
        use Expr::*;

        match expr {
            LiteralExpr(lexpr) => self.visit_literal_expr(lexpr),
            UnaryExpr(uexpr) => self.visit_unary_expr(uexpr),
            BinaryExpr(bexpr) => self.visit_binary_expr(bexpr),
            GroupingExpr(gexpr) => self.visit_grouping_expr(gexpr),
        }
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Literal, RuntimeError> {
        let literal = match &expr.0.typ {
            TokenType::String(s) => Literal::String(s.clone()),
            TokenType::Number(n) => Literal::Number(*n),
            TokenType::True => Literal::Boolean(true),
            TokenType::False => Literal::Boolean(false),
            TokenType::Nil => Literal::Nil,
            _ => unreachable!(),
        };

        Ok(literal)
    }

    fn visit_unary_expr(&self, unary_expr: &UnaryExpr) -> Result<Literal, RuntimeError> {
        use Literal::*;

        let literal = self.visit_expr(&unary_expr.expr)?;
        let literal = match unary_expr.op.typ {
            TokenType::Bang => match literal {
                Boolean(b) => Boolean(!b),
                a => {
                    let message = format!(
                        "Cannot perform '{:?}' on operand '{:?}'",
                        unary_expr.op.typ, a
                    );
                    return Err(RuntimeError {
                        line: unary_expr.op.line,
                        message,
                        exit_code: exitcode::DATAERR,
                    });
                }
            },
            TokenType::Minus => match literal {
                Number(n) => Number(-n),
                a => {
                    let message = format!(
                        "Cannot perform '{:?}' on operand '{:?}'",
                        unary_expr.op.typ, a
                    );
                    return Err(RuntimeError {
                        line: unary_expr.op.line,
                        message,
                        exit_code: exitcode::DATAERR,
                    });
                }
            },
            _ => unreachable!(),
        };

        Ok(literal)
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Literal, RuntimeError> {
        use Literal::*;

        let left = self.visit_expr(&expr.left)?;
        let right = self.visit_expr(&expr.right)?;
        let op = &expr.op;

        let literal = match (left, right, &op.typ) {
            // Plus OP
            (Number(a), Number(b), TokenType::Plus) => Number(a + b),
            (String(a), String(b), TokenType::Plus) => {
                let mut a = a;
                a.push_str(&b);
                String(a)
            }
            // Minus OP
            (Number(a), Number(b), TokenType::Minus) => Number(a - b),
            // Multiply OP
            (Number(a), Number(b), TokenType::Star) => Number(a * b),
            // Divide OP
            (Number(a), Number(b), TokenType::Slash) => Number(a / b),
            // Equal OP
            (Boolean(a), Boolean(b), TokenType::EqualEqual) => Boolean(a == b),
            (Number(a), Number(b), TokenType::EqualEqual) => Boolean(a == b),
            (String(a), String(b), TokenType::EqualEqual) => Boolean(a.eq(&b)),
            // Not Equal OP
            (Boolean(a), Boolean(b), TokenType::BangEqual) => Boolean(a != b),
            (Number(a), Number(b), TokenType::BangEqual) => Boolean(a != b),
            (String(a), String(b), TokenType::BangEqual) => Boolean(!a.eq(&b)),
            // Less Than OP
            (Number(a), Number(b), TokenType::Less) => Boolean(a < b),
            // Less Than Or Equal OP
            (Number(a), Number(b), TokenType::LessEqual) => Boolean(a <= b),
            // Greater Than OP
            (Number(a), Number(b), TokenType::Greater) => Boolean(a > b),
            // Greater Than Or Equal OP
            (Number(a), Number(b), TokenType::GreaterEqual) => Boolean(a >= b),
            (a, b, op) => {
                let message = format!(
                    "Cannot perform '{:?}' on operands '{:?}' and '{:?}'",
                    op, a, b
                );
                return Err(RuntimeError {
                    line: expr.op.line,
                    message,
                    exit_code: exitcode::DATAERR,
                });
            }
        };

        Ok(literal)
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Literal, RuntimeError> {
        self.visit_expr(&expr.0)
    }
}
