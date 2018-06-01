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
    ForwardSlash,
    Number(f64),
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
        self.skip_whitespace();

        match self.source.cur_char() {
            None => None,
            Some(c) => Some(match c {
                b'0'...b'9' => Token::Number(self.read_number()),
                b'-' => {
                    self.source.advance();
                    match self.source.cur_char() {
                        Some(b'0'...b'9') => Token::Number(-self.read_number()),
                        _ => Token::Minus,
                    }
                }
                other => {
                    let ret = match other {
                        b'+' => Token::Plus,
                        b'*' => Token::Asterisk,
                        b'/' => Token::ForwardSlash,
                        b'(' => Token::OpenParenthesis,
                        b')' => Token::CloseParenthesis,
                        other => Token::Unknown(other as char),
                    };
                    self.source.advance();
                    ret
                }
            }),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.source.cur_char() {
            if !c.is_ascii_whitespace() {
                break;
            }

            self.source.advance();
        }
    }

    fn read_number(&mut self) -> f64 {
        let mut num = 0.0;
        while let Some(c @ b'0'...b'9') = self.source.cur_char() {
            num *= 10.0;
            num += f64::from(c - b'0');
            self.source.advance();
        }
        //TODO: do I want to disallow representation like 1. ?
        if let Some(b'.') = self.source.cur_char() {
            self.source.advance();
            let mut factor = 10.0;
            while let Some(c @ b'0'...b'9') = self.source.cur_char() {
                num += f64::from(c - b'0') / factor;
                factor *= 10.0;
                self.source.advance();
            }   
        }
        num
    }
}
