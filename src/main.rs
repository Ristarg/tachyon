#![feature(range_contains, box_patterns)]

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Unknown,
    Int(u64),
    Plus,
    Asterisk,
    OpenParenthesis,
    CloseParenthesis,
    Whitespace,
}

#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, PartialEq)]
struct BinExpr {
    op: Operator,
    left: Expr,
    right: Expr,
}

#[derive(Debug, PartialEq)]
enum Expr {
    Number(u64),
    BinExpr(Box<BinExpr>) //FIXME: naming scheme! ugh
}

struct Tokenizer<'a> {
    source: &'a [u8],
    idx: usize,
    last_token: Token,
    rewind: bool,
}

impl<'a> Tokenizer<'a> {
    fn new(source: &str) -> Tokenizer {
        Tokenizer {
            source: source.as_bytes(),
            idx: 0,
            last_token: Token::Unknown,
            rewind: false,
        }
    }

    fn next_token(&mut self) -> Token {
        if self.rewind {
            self.rewind = false;
            return self.last_token.clone();
        }
        let cur_char = self.source[self.idx];
        let token = match cur_char {
            b'0'...b'9' => {
                let mut num: u64 = 0;
                while self.idx < self.source.len() //TODO: make a global OOB guard as well
                    && (b'0'..=b'9').contains(&self.source[self.idx])
                {
                    num *= 10;
                    num += (self.source[self.idx] - b'0') as u64;
                    self.idx += 1;
                }
                self.idx -= 1; //FIXME: rewinding because of the global stepping below, fragile?
                Token::Int(num)
            }
            b'+' => Token::Plus,
            b'*' => Token::Asterisk,
            b'(' => Token::OpenParenthesis,
            b')' => Token::CloseParenthesis,
            b' ' => Token::Whitespace,
            _ => Token::Unknown,
        };

        self.idx += 1;
        self.last_token = token.clone();
        token
    }

    fn rewind(&mut self) {
        self.rewind = true;
    }

    fn skip_whitespace(&mut self) {
        let mut token = self.next_token();
        while token == Token::Whitespace {
            token = self.next_token();
        }
        self.rewind();
    }

    fn expect_token(&mut self, token_kind: Token) {
        if token_kind != Token::Whitespace {
            self.skip_whitespace();
        }
        let token = self.next_token();
        if token != token_kind {
            panic!(
                "
            Expected token: \"{:?}\"
            Got instead: \"{:?}\"
            ",
                token_kind, token
            );
        }
    }

    fn expect_operator(&mut self) -> Operator {
        //TODO: since this is a lisp, make hardcoded operators into function names for lookup
        self.skip_whitespace();
        let token = self.next_token();
        match token {
            Token::Plus => Operator::Add,
            Token::Asterisk => Operator::Multiply,
            _ => {
                panic!(
                    "
            Expected token: \"Plus\" | \"Asterisk\"
            Got instead: \"{:?}\"
            ",
                    token
                );
            }
        }
    }

    fn parse_expression(&mut self) -> Expr {
        self.skip_whitespace();
        let token = self.next_token();
        match token {
            Token::Int(i) => Expr::Number(i),
            Token::OpenParenthesis => {
                let e = Expr::BinExpr(Box::new(self.parse_binary_expression()));
                self.expect_token(Token::CloseParenthesis);
                e
            }
            _ => {
                panic!(
                    "
            Expected token: \"Int\" | \"OpenParenthesis\"
            Got instead: \"{:?}\"
            ",
                    token
                );
            }
        }
    }

    fn parse_binary_expression(&mut self) -> BinExpr {
        let op = self.expect_operator();
        self.expect_token(Token::Whitespace);
        let left = self.parse_expression();
        self.expect_token(Token::Whitespace);
        let right = self.parse_expression();

        BinExpr { op, left, right }
    }
}

fn main() {
    use std::io::Write;

    let mut input_buf = String::new();
    loop {
        std::io::stdout().write(">>> ".as_bytes()).unwrap();
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input_buf).unwrap();

        {
            let mut rdr = Tokenizer::new(&input_buf);
            let e = rdr.parse_expression();
            let res = eval(&e);

            // println!(
            //     "Evaluated expression: ({:?} {} {})\nResult: {}",
            //     e.op, e.left, e.right, res
            // );
            println!("{}", res);
        }
        input_buf.clear();
    }
}

fn eval(expr: &Expr) -> u64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::BinExpr(box expr) => {
            match expr.op {
                Operator::Add => eval(&expr.left) + eval(&expr.right),
                Operator::Multiply => eval(&expr.left) * eval(&expr.right),
            }
        }
    }
}