use crate::{
    grammar::{BinaryExpr, BinaryOp, Expr, GroupingExpr, Literal, LiteralExpr, UnaryExpr, UnaryOp},
    visitor::Visitor,
};

pub struct Interpreter;

impl Visitor<Literal> for Interpreter {
    fn visit_expr(&self, expr: &Expr) -> Literal {
        use Expr::*;

        match expr {
            LiteralExpr(lexpr) => self.visit_literal_expr(&lexpr),
            UnaryExpr(uexpr) => self.visit_unary_expr(&uexpr),
            BinaryExpr(bexpr) => self.visit_binary_expr(&bexpr),
            GroupingExpr(gexpr) => self.visit_grouping_expr(&gexpr),
        }
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Literal {
        expr.0.clone()
    }

    fn visit_unary_expr(&self, unary_expr: &UnaryExpr) -> Literal {
        let literal = self.visit_expr(&unary_expr.expr);
        match unary_expr.op {
            UnaryOp::Not => match literal {
                Literal::Boolean(b) => Literal::Boolean(!b),
                _ => panic!(),
            },
            UnaryOp::Negate => match literal {
                Literal::Number(n) => Literal::Number(-n),
                _ => panic!(),
            },
        }
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Literal {
        use BinaryOp::*;
        use Literal::*;

        let left = self.visit_expr(&expr.left);
        let right = self.visit_expr(&expr.right);
        let op = &expr.op;

        match (left, right, op) {
            // Plus OP
            (Number(a), Number(b), Plus) => Number(a + b),
            (String(a), String(b), Plus) => {
                let mut a = a;
                a.push_str(&b);
                String(a)
            }
            // Minus OP
            (Number(a), Number(b), Minus) => Number(a - b),
            // Multiply OP
            (Number(a), Number(b), Multiply) => Number(a * b),
            // Divide OP
            (Number(a), Number(b), Divide) => Number(a / b),
            // Equal OP
            (Boolean(a), Boolean(b), Equal) => Boolean(a == b),
            (Number(a), Number(b), Equal) => Boolean(a == b),
            (String(a), String(b), Equal) => Boolean(a.eq(&b)),
            // Not Equal OP
            (Boolean(a), Boolean(b), NotEqual) => Boolean(a != b),
            (Number(a), Number(b), NotEqual) => Boolean(a == b),
            (String(a), String(b), NotEqual) => Boolean(!a.eq(&b)),
            // Less Than OP
            (Number(a), Number(b), LessThan) => Boolean(a < b),
            // Less Than Or Equal OP
            (Number(a), Number(b), LessThanOrEqual) => Boolean(a <= b),
            // Greater Than OP
            (Number(a), Number(b), GreaterThan) => Boolean(a > b),
            // Greater Than Or Equal OP
            (Number(a), Number(b), GreaterThanOrEqual) => Boolean(a >= b),
            _ => panic!(),
        }
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Literal {
        let literal = self.visit_expr(&expr.0);

        literal
    }
}
