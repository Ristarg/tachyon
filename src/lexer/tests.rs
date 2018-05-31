use super::*;
use std::iter;

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
    assert_eq!(Lexer::new("+").next_token(), Token::Plus);
    assert_eq!(Lexer::new("-").next_token(), Token::Minus);
    assert_eq!(Lexer::new("*").next_token(), Token::Asterisk);
    assert_eq!(Lexer::new("(").next_token(), Token::OpenParenthesis);
    assert_eq!(Lexer::new(")").next_token(), Token::CloseParenthesis);
}

#[test]
fn test_lexer_unknown_characters() {
    assert_eq!(Lexer::new("a").next_token(), Token::Unknown('a'));
    assert_eq!(Lexer::new("$").next_token(), Token::Unknown('$'));
}

#[test]
fn test_lexer_expressions() {
    impl Lexer {
        fn assert_tokens_eq(&mut self, tokens: &[Token]) {
            tokens
                .iter()
                .zip(iter::repeat_with(|| self.next_token()))
                .for_each(|(a, b)| assert_eq!(*a, b));
        }
    }

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

// #[test]
