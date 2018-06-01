#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Unknown(char),
    OpenParenthesis,
    CloseParenthesis,
    Plus,
    Minus,
    Asterisk,
    Int(i64),
}

struct SourceStream {
    source: Vec<u8>,
    idx: usize,
}

// this is literally a peekable iterator
// I am literally retarded
impl SourceStream {
    fn new(source: &str) -> SourceStream {
        let bytes = source.to_owned().into_bytes();
        SourceStream {
            source: bytes,
            idx: 0,
        }
    }

    fn cur_char(&self) -> Option<u8> {
        if self.idx == self.source.len() {
            return None;
        }

        Some(self.source[self.idx])
    }

    fn advance(&mut self) {
        if self.idx < self.source.len() {
            self.idx += 1;
        }
    }
}

pub struct Lexer {
    source: SourceStream,
}

impl Lexer {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            source: SourceStream::new(source),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.source.cur_char() {
            if !c.is_ascii_whitespace() {
                break;
            }

            self.source.advance();
        }

        if let Some(c) = self.source.cur_char() {
            Some(match c {
                b'0'...b'9' => Token::Int(self.read_uint()),
                b'-' => {
                    self.source.advance();
                    if let Some(b'0'...b'9') = self.source.cur_char() {
                        Token::Int(-self.read_uint())
                    } else {
                        Token::Minus
                    }
                }
                rest => {
                    let ret = match rest {
                        b'+' => Token::Plus,
                        b'*' => Token::Asterisk,
                        b'(' => Token::OpenParenthesis,
                        b')' => Token::CloseParenthesis,
                        other => Token::Unknown(other as char),
                    };
                    self.source.advance();
                    ret
                }
            })
        } else {
            None
        }
    }

    fn read_uint(&mut self) -> i64 {
        let mut num: i64 = 0;
        while let Some(c @ b'0'...b'9') = self.source.cur_char() {
            num *= 10;
            num += i64::from(c - b'0');
            self.source.advance();
        }
        num
    }
}
