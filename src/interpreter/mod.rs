use parser::*;
use std::{self, collections::HashMap};

#[cfg(test)]
mod tests;

type FnVec = Fn(Vec<f64>) -> f64;

macro_rules! context {
    ($($key:expr => $value:expr),*) => ({
        let mut tmp: HashMap<String, Box<FnVec>> = HashMap::new();
        $(tmp.insert($key.to_owned(), Box::new($value));)*
        tmp
    });
}

pub fn eval(source: &str) -> Option<f64> {
    let ctx = context!{
        "+"  => |argv| argv.iter().fold(0.0, |acc, x| acc + x),
        "-"  => |argv| argv.iter().fold(0.0, |acc, x| acc - x),
        "*"  => |argv| argv.iter().fold(1.0, |acc, x| acc * x),
        "/"  => |argv| argv.iter().skip(1).fold(*argv.first().unwrap_or(&0.0), |acc, x| acc / x),
        "pi" => |_| std::f64::consts::PI,
        "e"  => |_| std::f64::consts::E
    };

    Parser::new(source)
        .parse_expression()
        .and_then(|e| Some(eval_expr(&e, &ctx)))
}

fn eval_expr(expr: &Expr, ctx: &HashMap<String, Box<FnVec>>) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::FnExprPtr(box expr) => match ctx.get(&expr.op.0) {
            Some(func) => func(expr.args.iter().map(|a| eval_expr(a, &ctx)).collect()),
            None => error!("no such function: \"{}\"", &expr.op.0),
        },
    }
}
