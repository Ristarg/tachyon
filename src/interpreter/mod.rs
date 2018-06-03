use parser::*;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

type FnBin = Fn(Vec<f64>) -> f64;

macro_rules! context {
    ($($key:expr => $value:expr),*) => ({
        let mut tmp: HashMap<String, Box<FnBin>> = HashMap::new();
        $(
            tmp.insert($key.to_owned(), Box::new($value));
        )*
        tmp
    });
}

pub fn eval(source: &str) -> Option<f64> {
    let ctx = context!{
        "+"    => |argv| argv[0] + argv[1],
        "-"    => |argv| argv[0] - argv[1],
        "*"    => |argv| argv[0] * argv[1],
        "/"    => |argv| argv[0] / argv[1],
        "test" => |_| 69.1337
    };

    Parser::new(source)
        .parse_expression()
        .and_then(|e| Some(eval_expr(&e, &ctx)))
}

fn eval_expr(expr: &Expr, ctx: &HashMap<String, Box<FnBin>>) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::FnExprPtr(box expr) => match ctx.get(&expr.op.0) {
            Some(fnbin) => fnbin(expr.args.iter().map(|a| eval_expr(a, &ctx)).collect()),
            None => panic!("no such function: \"{}\"", &expr.op.0),
        },
    }
}
