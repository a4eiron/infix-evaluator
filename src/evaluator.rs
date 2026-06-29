use crate::{lexer::Token, parser::Expr};

pub fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Unary { op, expr } => {
            let val = eval(expr);
            match op {
                Token::Minus => -val,
                _ => val,
            }
        }
        Expr::Binary { op, left, right } => {
            let l_value = eval(left);
            let r_value = eval(right);

            match op {
                Token::Plus => l_value + r_value,
                Token::Minus => l_value - r_value,
                Token::Star => l_value * r_value,
                Token::Slash => l_value / r_value,
                _ => 0.0,
            }
        }
    }
}
