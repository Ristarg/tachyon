use lexer::*;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct BinExpr {
    pub op: Operator,
    pub left: Expr,
    pub right: Expr,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i64),
    BinExprPtr(Box<BinExpr>),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
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
        self.lexer.skip_whitespace();

        match self.lexer.next_token() {
            Token::Int(i) => Expr::Number(i),
            Token::OpenParenthesis => {
                let expr = self.parse_binary_expression();
                self.expect_token(&Token::CloseParenthesis);
                Expr::BinExprPtr(Box::new(expr))
            }
            other => panic!(
                "Expected token: Int | OpenParenthesis\nGot instead: {:?}",
                other
            ),
        }
    }

    fn parse_binary_expression(&mut self) -> BinExpr {
        let op = self.expect_operator();
        self.expect_token(&Token::Whitespace);
        let left = self.parse_expression();
        self.expect_token(&Token::Whitespace);
        let right = self.parse_expression();

        BinExpr { op, left, right }
    }

    fn expect_token(&mut self, expected: &Token) {
        if *expected != Token::Whitespace {
            self.lexer.skip_whitespace();
        }

        let token = self.lexer.next_token();
        if token != *expected {
            panic!("Expected token: {:?}\nGot instead: {:?}", expected, token);
        }
    }

    fn expect_operator(&mut self) -> Operator {
        self.lexer.skip_whitespace();

        match self.lexer.next_token() {
            //FIXME: this feels redundant
            Token::Plus => Operator::Add,
            Token::Minus => Operator::Subtract,
            Token::Asterisk => Operator::Multiply,
            other => panic!(
                "Expected token: Plus | Minus | Asterisk\nGot instead: {:?}",
                other
            ),
        }
    }
}
