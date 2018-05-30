#![feature(range_contains, box_patterns, iterator_repeat_with)]

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Token {
    Unknown(char),
    OpenParenthesis,
    CloseParenthesis,
    Whitespace,
    Plus,
    Minus,
    Asterisk,
    Int(i64),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
}

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

pub struct Tokenizer<'a> {
    source: &'a [u8],
    idx: usize,
    last_token: Token,
    rewind: bool,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &str) -> Tokenizer {
        Tokenizer {
            source: source.as_bytes(),
            idx: 0,
            last_token: Token::Unknown('\0'),
            rewind: false,
        }
    }

    pub fn parse_expression(&mut self) -> Expr {
        self.skip_whitespace();

        match self.next_token() {
            Token::Int(i) => Expr::Number(i),
            Token::OpenParenthesis => {
                let e = Expr::BinExprPtr(Box::new(self.parse_binary_expression()));
                self.expect_token(&Token::CloseParenthesis);
                e
            }
            other => {
                panic!(
                    "Expected token: Int | OpenParenthesis\nGot instead: {:?}",
                    other
                );
            }
        }
    }

    fn cur_char(&self) -> u8 {
        self.source[self.idx]
    }

    fn read_uint(&mut self) -> i64 {
        let mut num: i64 = 0;
        while self.idx < self.source.len() //TODO: make a global OOB guard as well
            && (b'0'..=b'9').contains(&self.cur_char())
        {
            num *= 10;
            num += i64::from(self.cur_char() - b'0');
            self.idx += 1;
        }
        self.idx -= 1; //FIXME: rewinding because of the global stepping below, fragile?
        num
    }

    fn next_token(&mut self) -> Token {
        if self.rewind {
            self.rewind = false;
            return self.last_token;
        }

        let token = match self.cur_char() {
            b'0'...b'9' => Token::Int(self.read_uint()),
            b'+' => Token::Plus,
            b'-' => {
                if self.idx + 1 < self.source.len()
                    && (b'0'..=b'9').contains(&self.source[self.idx + 1])
                {
                    self.idx += 1;
                    Token::Int(-self.read_uint())
                } else {
                    Token::Minus
                }
            }
            b'*' => Token::Asterisk,
            b'(' => Token::OpenParenthesis,
            b')' => Token::CloseParenthesis,
            b' ' => Token::Whitespace,
            other => Token::Unknown(other as char),
        };

        self.idx += 1;
        self.last_token = token;
        token
    }

    fn skip_whitespace(&mut self) {
        while let Token::Whitespace = self.next_token() {}
        self.rewind = true;
    }

    fn expect_token(&mut self, token_kind: &Token) {
        if *token_kind != Token::Whitespace {
            self.skip_whitespace();
        }
        let token = self.next_token();
        if token != *token_kind {
            panic!("Expected token: {:?}\nGot instead: {:?}", token_kind, token);
        }
    }

    fn expect_operator(&mut self) -> Operator {
        //TODO: since this is a lisp, make hardcoded operators into function names for lookup
        self.skip_whitespace();
        let token = self.next_token();
        match token {
            Token::Plus => Operator::Add,
            Token::Minus => Operator::Subtract,
            Token::Asterisk => Operator::Multiply,
            _ => {
                panic!(
                    "Expected token: Plus | Minus | Asterisk\nGot instead: {:?}",
                    token
                );
            }
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
}

pub fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::BinExprPtr(box expr) => match expr.op {
            Operator::Add => eval(&expr.left) + eval(&expr.right),
            Operator::Multiply => eval(&expr.left) * eval(&expr.right),
            Operator::Subtract => eval(&expr.left) - eval(&expr.right),
        },
    }
}
