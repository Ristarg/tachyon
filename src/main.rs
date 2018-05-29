#[derive(Clone, Debug, PartialEq)]
enum Token {
    Unknown,
    Int(u8),
    PlusSign,
    OpenParenthesis,
    CloseParenthesis,
    Whitespace,
}

#[derive(Debug, PartialEq)]
struct Expr {
    left: u8,
    right: u8,
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
            let out = self.last_token.clone();
            self.rewind = false;
            self.last_token = Token::Unknown;
            out
        } else {
            let cur_char = self.source[self.idx];
            let token = match cur_char {
                b'0'...b'9' => Token::Int(cur_char - b'0'),
                b'+' => Token::PlusSign,
                b'(' => Token::OpenParenthesis,
                b')' => Token::CloseParenthesis,
                b' ' => Token::Whitespace,
                _ => Token::Unknown,
            };

            self.idx += 1;
            self.last_token = token.clone();
            token
        }
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

    fn expect_number(&mut self) -> u8 {
        self.skip_whitespace();
        let token = self.next_token();
        if let Token::Int(num) = token {
            num
        } else {
            panic!(
                "
            Expected token: Int
            Got instead: \"{:?}\"
            ",
                token
            ); // TODO: code duplication REEEEEEE
        }
    }

    fn parse_expression(&mut self) -> Expr {
        // FIXME: parse method on the Tokenizer?
        self.expect_token(Token::OpenParenthesis);
        self.expect_token(Token::PlusSign);
        self.expect_token(Token::Whitespace);
        let left = self.expect_number();
        self.expect_token(Token::Whitespace);
        let right = self.expect_number();
        self.expect_token(Token::CloseParenthesis);

        Expr { left, right }
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

            println!(
                "Evaluated expression: (+ {} {})\nResult: {}",
                e.left, e.right, res
            );
        }
        input_buf.clear();
    }
}

fn eval(expr: &Expr) -> u8 {
    expr.left + expr.right
}

// *********
// * TESTS *
// *********

#[test]
fn test_lexer_integers() {
    let mut rdr = Tokenizer::new("0123456789");

    assert_eq!(rdr.next_token(), Token::Int(0));
    assert_eq!(rdr.next_token(), Token::Int(1));
    assert_eq!(rdr.next_token(), Token::Int(2));
    assert_eq!(rdr.next_token(), Token::Int(3));
    assert_eq!(rdr.next_token(), Token::Int(4));
    assert_eq!(rdr.next_token(), Token::Int(5));
    assert_eq!(rdr.next_token(), Token::Int(6));
    assert_eq!(rdr.next_token(), Token::Int(7));
    assert_eq!(rdr.next_token(), Token::Int(8));
    assert_eq!(rdr.next_token(), Token::Int(9));
}

#[test]
fn test_lexer_singletons() {
    assert_eq!(Tokenizer::new("+").next_token(), Token::PlusSign);
    assert_eq!(Tokenizer::new("(").next_token(), Token::OpenParenthesis);
    assert_eq!(Tokenizer::new(")").next_token(), Token::CloseParenthesis);
    assert_eq!(Tokenizer::new(" ").next_token(), Token::Whitespace);
}

#[test]
fn test_lexer_unknown_characters() {
    assert_eq!(Tokenizer::new("a").next_token(), Token::Unknown);
    assert_eq!(Tokenizer::new("$").next_token(), Token::Unknown);
}

#[test]
fn test_lexer_expression() {
    let mut rdr = Tokenizer::new("(+ 1 2)");
    assert_eq!(rdr.next_token(), Token::OpenParenthesis);
    assert_eq!(rdr.next_token(), Token::PlusSign);
    assert_eq!(rdr.next_token(), Token::Whitespace);
    assert_eq!(rdr.next_token(), Token::Int(1));
    assert_eq!(rdr.next_token(), Token::Whitespace);
    assert_eq!(rdr.next_token(), Token::Int(2));
    assert_eq!(rdr.next_token(), Token::CloseParenthesis);
}

#[test]
fn test_parser_expression() {
    let mut rdr = Tokenizer::new("(+ 1 2)");
    let e = rdr.parse_expression();
    assert_eq!(e, Expr { left: 1, right: 2 });
}

#[test]
fn test_eval() {
    assert_eq!(eval(&Expr { left: 3, right: 5 }), 8);
    assert_eq!(eval(&Expr { left: 9, right: 0 }), 9);
    assert_eq!(eval(&Expr { left: 4, right: 1 }), 5);
}
