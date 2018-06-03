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

pub fn eval(source: &str) -> f64 {
    let ctx = context!{
        "+"    => |argv| argv[0] + argv[1],
        "-"    => |argv| argv[0] - argv[1],
        "*"    => |argv| argv[0] * argv[1],
        "/"    => |argv| argv[0] / argv[1],
        "test" => |_| 69.1337
    };

    eval_expr(&Parser::new(source).parse_expression(), &ctx)
}

fn eval_expr(expr: &Expr, ctx: &HashMap<String, Box<FnBin>>) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::BinExprPtr(box expr) => match ctx.get(&expr.op.0) {
            Some(fnbin) => fnbin(vec![
                eval_expr(&expr.left, &ctx),
                eval_expr(&expr.right, &ctx),
            ]),
            None => panic!("no such function: \"{}\"", &expr.op.0),
        },
    }
}
