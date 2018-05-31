use super::*;
use std::iter;

impl Lexer {
    fn assert_tokens_eq(&mut self, tokens: &[Token]) {
        tokens
            .iter()
            .zip(iter::repeat_with(|| self.next_token()))
            .for_each(|(a, b)| assert_eq!(*a, b));
    }
}

#[test]
fn test_lexer_integers() {
    assert_eq!(
        Lexer::new("1234567890").next_token(),
        Token::Int(1234567890)
    );
    assert_eq!(Lexer::new("456456").next_token(), Token::Int(456456));
    assert_eq!(Lexer::new("-123132").next_token(), Token::Int(-123132));
}

#[test]
fn test_lexer_singletons() {
    Lexer::new("+-*()").assert_tokens_eq(&[
        Token::Plus,
        Token::Minus,
        Token::Asterisk,
        Token::OpenParenthesis,
        Token::CloseParenthesis,
    ]);
}

#[test]
fn test_lexer_unknown_characters() {
    Lexer::new("a$#~`").assert_tokens_eq(&[
        Token::Unknown('a'),
        Token::Unknown('$'),
        Token::Unknown('#'),
        Token::Unknown('~'),
        Token::Unknown('`'),
    ]);
}

#[test]
fn test_lexer_with_spaces() {
    Lexer::new(" \t\r\n ( 123 456 - 789 -789").assert_tokens_eq(&[
        Token::OpenParenthesis,
        Token::Int(123),
        Token::Int(456),
        Token::Minus,
        Token::Int(789),
        Token::Int(-789),
    ]);
}

#[test]
fn test_lexer_expressions() {
    Lexer::new("(+ 123 245)").assert_tokens_eq(&[
        Token::OpenParenthesis,
        Token::Plus,
        Token::Int(123),
        Token::Int(245),
        Token::CloseParenthesis,
    ]);

    Lexer::new("(* 398 4788)").assert_tokens_eq(&[
        Token::OpenParenthesis,
        Token::Asterisk,
        Token::Int(398),
        Token::Int(4788),
        Token::CloseParenthesis,
    ]);

    Lexer::new("(- (+ 98 4) 788)").assert_tokens_eq(&[
        Token::OpenParenthesis,
        Token::Minus,
        Token::OpenParenthesis,
        Token::Plus,
        Token::Int(98),
        Token::Int(4),
        Token::CloseParenthesis,
        Token::Int(788),
        Token::CloseParenthesis,
    ]);
}
