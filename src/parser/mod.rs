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

    pub fn parse_all(&mut self) -> Vec<Expr> {
        let mut exprs = vec![];
        loop {
            match self.parse_expression() {
                Ok(Some(expr)) => exprs.push(expr),
                Ok(None) => error!("unexpected closing delimiter"),
                Err(_) => break
            }
        }
        exprs
    }

    pub fn parse_expression(&mut self) -> Result<Option<Expr>, ()> {
        match self.lexer.next_token() {
            Some(Token::NumberLiteral(i)) => Ok(Some(Expr::Number(i))),
            Some(Token::OpenParenthesis) => {
                Ok(Some(Expr::FnExprPtr(Box::new(self.parse_fn_expression()))))
            }
            Some(Token::CloseParenthesis) => Ok(None),
            None => Err(()),
            other => error!("Unexpected token: {:?}", other),
        }
    }

    fn parse_fn_expression(&mut self) -> FnExpr {
        let op = match self.lexer.next_token() {
            Some(Token::Identifier(id)) => Identifier(id),
            other => error!("Expected identifier, got instead: {:?}", other),
        };

        let mut args = vec![];
        loop {
            match self.parse_expression() {
                Ok(Some(t)) => args.push(t),
                Ok(None) => break,
                Err(_) => error!("missing closing delimiter"),
            }
        }

        FnExpr { op, args }
    }
}
