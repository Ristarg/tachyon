use parser::*;

#[cfg(test)]
mod tests;

pub fn eval(source: &str) -> f64 {
    eval_expr(&Parser::new(source).parse_expression())
}

fn eval_expr(expr: &Expr) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::BinExprPtr(box expr) => match expr.op {
            Operator::Add => eval_expr(&expr.left) + eval_expr(&expr.right),
            Operator::Multiply => eval_expr(&expr.left) * eval_expr(&expr.right),
            Operator::Subtract => eval_expr(&expr.left) - eval_expr(&expr.right),
        },
    }
}
