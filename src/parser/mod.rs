use lexer::*;
use std;

#[cfg(test)]
mod tests;

macro_rules! syntax_error {
    ($($tt:tt)*) => ({
        eprintln!($($tt)*);
        std::process::exit(1);
    });
}

/// Wrapper over enum identifier variant for better type checking.
#[derive(Debug, PartialEq)]
pub struct Identifier(pub String);

#[derive(Debug, PartialEq)]
pub struct FnExpr {
    pub op: Identifier,
    pub args: Vec<Expr>,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    FnExprPtr(Box<FnExpr>),
}

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(source: &str) -> Parser {
        Parser {
            lexer: Lexer::new(source),
        }
    }

    pub fn parse_expression(&mut self) -> Expr {
        match self.lexer.next_token() {
            Some(Token::NumberLiteral(i)) => Expr::Number(i),
            Some(Token::OpenParenthesis) => {
                let expr = self.parse_fn_expression();
                Expr::FnExprPtr(Box::new(expr))
            }
            other => syntax_error!(
                "Expected token: NumberLiteral | OpenParenthesis\nGot instead: {:?}",
                other
            ),
        }
    }

    fn parse_fn_expression(&mut self) -> FnExpr {
        let op = self.expect_identifier();
        let mut args = vec![];
        while let Some(t) = self.lexer.next_token() {
            match t {
                Token::NumberLiteral(n) => args.push(Expr::Number(n)),
                Token::OpenParenthesis => args.push(Expr::FnExprPtr(Box::new(self.parse_fn_expression()))),
                Token::CloseParenthesis => break,
                other => syntax_error!("unexpected token: {:?}", other)
            }
        }

        FnExpr { op, args }
    }

    fn expect_token(&mut self, expected: Token) {
        let token = self.lexer.next_token();
        if token != Some(expected.clone()) {
            syntax_error!("Expected token: {:?}\nGot instead: {:?}", expected, token);
        }
    }

    fn expect_identifier(&mut self) -> Identifier {
        match self.lexer.next_token() {
            //FIXME: this feels redundant
            Some(Token::Identifier(id)) => Identifier(id),
            other => syntax_error!("Expected identifier\nGot instead: {:?}", other),
        }
    }
}
