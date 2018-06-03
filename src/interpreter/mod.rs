use lexer::Token;
use parser::*;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

type FnBin = Fn(f64, f64) -> f64;

macro_rules! context {
    ($($key:expr => $value:expr),*) => ({
        let mut tmp: HashMap<&'static str, Box<FnBin>> = HashMap::new();
        $(
            tmp.insert($key, Box::new($value));
        )*
        tmp
    });
}

pub fn eval(source: &str) -> f64 {
    let ctx = context!{
        "+" => |a, b| a + b,
        "-" => |a, b| a - b,
        "*" => |a, b| a * b,
        "/" => |a, b| a / b
    };

    eval_expr(&Parser::new(source).parse_expression(), &ctx)
}

fn eval_expr(expr: &Expr, ctx: &HashMap<&'static str, Box<FnBin>>) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::BinExprPtr(box expr) => match expr.op {
            Token::Identifier(id) => match ctx.get(id) {
                Some(fnbin) => fnbin(eval_expr(&expr.left, &ctx), eval_expr(&expr.right, &ctx)),
                None => panic!("no such function: \"{}\"", id),
            },
            _ => panic!("expected identifier, got instead: {:?}", expr.op),
        },
    }
}
