use source_stream::*;

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
