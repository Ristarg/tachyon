use lexer::*;
use std;

#[cfg(test)]
mod tests;

macro_rules! syntax_error {
    ($($tt:tt)*) => ({
        eprintln!($($tt)*);
        std::process::exit(1)
    });
}

/// Wrapper over enum identifier variant for better type checking.
#[derive(Debug, PartialEq)]
pub struct Identifier(pub String);

#[derive(Debug, PartialEq)]
pub struct BinExpr {
    pub op: Identifier,
    pub left: Expr,
    pub right: Expr,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    BinExprPtr(Box<BinExpr>),
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
                let expr = self.parse_binary_expression();
                self.expect_token(Token::CloseParenthesis);
                Expr::BinExprPtr(Box::new(expr))
            }
            other => syntax_error!(
                "Expected token: NumberLiteral | OpenParenthesis\nGot instead: {:?}",
                other
            ),
        }
    }

    fn parse_binary_expression(&mut self) -> BinExpr {
        let op = self.expect_identifier();
        let left = self.parse_expression();
        let right = self.parse_expression();

        BinExpr { op, left, right }
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
            other => syntax_error!(
                "Expected identifier\nGot instead: {:?}",
                other
            ),
        }
    }
}
