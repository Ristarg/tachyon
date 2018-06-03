use source_stream::*;
use std::iter::FromIterator;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    OpenParenthesis,
    CloseParenthesis,
    Identifier(String),
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

        self.source.cur_char().and_then(|c| {
            Some(match c {
                '0'...'9' => Token::Number(self.read_number()),
                '-' => {
                    self.source.advance();
                    match self.source.cur_char() {
                        Some('0'...'9') => Token::Number(-self.read_number()),
                        _ => Token::Identifier("-".to_owned()), //TODO: read ident
                    }
                }
                other => {
                    let ret = match other {
                        '(' => Token::OpenParenthesis,
                        ')' => Token::CloseParenthesis,
                        _ => Token::Identifier(self.read_identifier()),
                    };
                    self.source.advance();
                    ret
                }
            })
        })
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.source.cur_char() {
            if !c.is_whitespace() {
                break;
            }

            self.source.advance();
        }
    }

    fn read_number(&mut self) -> f64 {
        let mut num = 0.0;

        // read whole part
        while let Some(c @ '0'...'9') = self.source.cur_char() {
            num *= 10.0;
            num += f64::from(c.to_digit(10).unwrap());
            self.source.advance();
        }

        //TODO: do I want to disallow representation like 1. ?
        // read fractional part
        if let Some('.') = self.source.cur_char() {
            self.source.advance();
            let mut factor = 10.0;
            while let Some(c @ '0'...'9') = self.source.cur_char() {
                num += f64::from(c.to_digit(10).unwrap()) / factor;
                factor *= 10.0;
                self.source.advance();
            }
        }

        num
    }

    fn read_identifier(&mut self) -> String {
        let mut chars = vec![];
        while let Some(c) = self.source.cur_char() {
            if c.is_whitespace() {
                break;
            }

            chars.push(c);
            self.source.advance();
        }

        String::from_iter(chars)
    }
}
