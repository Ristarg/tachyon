use super::*;

impl Lexer {
    fn assert_tokens_eq(&mut self, tokens: &[Token]) {
        use std::iter;
        tokens
            .iter()
            .zip(iter::repeat_with(|| self.next_token()))
            .for_each(|(a, b)| assert_eq!(Some(*a), b));
    }
}

#[test]
fn test_lexer_singletons() {
    Lexer::new("+-*/()").assert_tokens_eq(&[
        Token::Identifier('+'),
        Token::Identifier('-'),
        Token::Identifier('*'),
        Token::Identifier('/'),
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
fn test_lexer_numbers() {
    assert_eq!(
        Lexer::new("1234567890").next_token(),
        Some(Token::Number(1234567890.0))
    );
    assert_eq!(
        Lexer::new("456456.5565").next_token(),
        Some(Token::Number(456456.5565))
    );
    assert_eq!(
        Lexer::new("-123132").next_token(),
        Some(Token::Number(-123132.0))
    );
}

#[test]
fn test_lexer_with_spaces() {
    Lexer::new(" \t\r\n ( 123 456.112 - 789 -789").assert_tokens_eq(&[
        Token::OpenParenthesis,
        Token::Number(123.0),
        Token::Number(456.112),
        Token::Identifier('-'),
        Token::Number(789.0),
        Token::Number(-789.0),
    ]);
}

#[test]
fn test_lexer_expressions() {
    Lexer::new("(+ 123 -245)").assert_tokens_eq(&[
        Token::OpenParenthesis,
        Token::Identifier('+'),
        Token::Number(123.0),
        Token::Number(-245.0),
        Token::CloseParenthesis,
    ]);

    Lexer::new("(* 398.489 4788)").assert_tokens_eq(&[
        Token::OpenParenthesis,
        Token::Identifier('*'),
        Token::Number(398.489),
        Token::Number(4788.0),
        Token::CloseParenthesis,
    ]);

    Lexer::new("(- (/ 98 4) 788)").assert_tokens_eq(&[
        Token::OpenParenthesis,
        Token::Identifier('-'),
        Token::OpenParenthesis,
        Token::Identifier('/'),
        Token::Number(98.0),
        Token::Number(4.0),
        Token::CloseParenthesis,
        Token::Number(788.0),
        Token::CloseParenthesis,
    ]);
}
