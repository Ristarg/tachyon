#[derive(Debug, PartialEq)]
enum Token {
    Unknown,
    Int(u8),
    PlusSign,
    OpenBracket,
    CloseBracket,
    Space
}

struct SourceReader<'a> {
    source: &'a [u8],
    idx: usize,
}

impl<'a> SourceReader<'a> {
    fn new(source: &str) -> SourceReader {
        SourceReader {
            source: source.as_bytes(),
            idx: 0
        }
    }

    fn next_token(&mut self) -> Token {
        let cur_char = self.source[self.idx];
        let token = match cur_char {
            b'0'...b'9' => Token::Int(cur_char - b'0'),
            b'+' => Token::PlusSign,
            b'(' => Token::OpenBracket,
            b')' => Token::CloseBracket,
            b' ' => Token::Space,
            _ => Token::Unknown,
        };

        self.idx += 1;
        token
    }
}

fn main() {
    let mut rdr = SourceReader::new("(+ 1 2)");

    while rdr.idx < rdr.source.len() {
        let token = rdr.next_token();
        println!("Token: {:?}", token);
    }
}

#[test]
fn test_lexer_integers() {
    let mut rdr = SourceReader::new("0123456789");

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
    assert_eq!(SourceReader::new("+").next_token(), Token::PlusSign);
    assert_eq!(SourceReader::new("(").next_token(), Token::OpenBracket);
    assert_eq!(SourceReader::new(")").next_token(), Token::CloseBracket);
    assert_eq!(SourceReader::new(" ").next_token(), Token::Space);
}

#[test]
fn test_lexer_unknown_characters() {
    assert_eq!(SourceReader::new("a").next_token(), Token::Unknown);
    assert_eq!(SourceReader::new("$").next_token(), Token::Unknown);
}
