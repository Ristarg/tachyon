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
    Number(f64),
    BinExprPtr(Box<BinExpr>),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
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
            Some(Token::Number(i)) => Expr::Number(i),
            Some(Token::OpenParenthesis) => {
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
        let left = self.parse_expression();
        let right = self.parse_expression();

        BinExpr { op, left, right }
    }

    fn expect_token(&mut self, expected: &Token) {
        let token = self.lexer.next_token();
        if token != Some(*expected) {
            panic!("Expected token: {:?}\nGot instead: {:?}", expected, token);
        }
    }

    fn expect_operator(&mut self) -> Operator {
        match self.lexer.next_token() {
            //FIXME: this feels redundant
            Some(Token::Plus) => Operator::Add,
            Some(Token::Minus) => Operator::Subtract,
            Some(Token::Asterisk) => Operator::Multiply,
            Some(Token::ForwardSlash) => Operator::Divide,
            other => panic!(
                "Expected token: Plus | Minus | Asterisk\nGot instead: {:?}",
                other
            ),
        }
    }
}
