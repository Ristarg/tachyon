use lexer::*;
use std;

#[cfg(test)]
mod tests;

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

    pub fn parse_expression(&mut self) -> Option<Expr> {
        self.lexer.next_token().and_then(|t| match t {
            Token::NumberLiteral(i) => Some(Expr::Number(i)),
            Token::OpenParenthesis => Some(Expr::FnExprPtr(Box::new(self.parse_fn_expression()))),
            Token::CloseParenthesis => None, //FIXME: "(pi))))))))))" doesn't raise any error
            other => error!("Unexpected token: {:?}", other),
        })
    }

    fn parse_fn_expression(&mut self) -> FnExpr {
        let op = match self.lexer.next_token() {
            Some(Token::Identifier(id)) => Identifier(id),
            other => error!("Expected identifier, got instead: {:?}", other),
        };

        let mut args = vec![];
        while let Some(t) = self.parse_expression() {
            args.push(t);
        }

        FnExpr { op, args }
    }
}
