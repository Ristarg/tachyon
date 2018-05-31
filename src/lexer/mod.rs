#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Unknown(char),
    OpenParenthesis,
    CloseParenthesis,
    Whitespace,
    Plus,
    Minus,
    Asterisk,
    Int(i64),
}

pub struct Lexer {
    source: Vec<u8>,
    idx: usize,
    last_token: Token,
    rewind: bool,
}

impl Lexer {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            source: source.to_owned().into_bytes(),
            idx: 0,
            last_token: Token::Unknown('\0'),
            rewind: false,
        }
    }

    pub fn next_token(&mut self) -> Token {
        if self.rewind {
            self.rewind = false;
            return self.last_token;
        }

        let token = match self.cur_char() {
            b'0'...b'9' => Token::Int(self.read_uint()),
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
            b'+' => Token::Plus,
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

    pub fn skip_whitespace(&mut self) {
        while let Token::Whitespace = self.next_token() {}
        self.rewind = true;
    }
    
    fn cur_char(&self) -> u8 {
        self.source[self.idx]
    }

    fn read_uint(&mut self) -> i64 {
        let mut num: i64 = 0;
        while self.idx < self.source.len() && (b'0'..=b'9').contains(&self.cur_char()) {
            num *= 10;
            num += i64::from(self.cur_char() - b'0');
            self.idx += 1;
        }
        self.idx -= 1;
        num
    }
}
