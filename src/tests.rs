use super::*;

#[test]
fn test_lexer_integers() {
    let mut rdr = Tokenizer::new("1234567890");

    assert_eq!(rdr.next_token(), Token::Int(1234567890));
}

#[test]
fn test_lexer_singletons() {
    assert_eq!(Tokenizer::new("+").next_token(), Token::Plus);
    assert_eq!(Tokenizer::new("-").next_token(), Token::Minus);
    assert_eq!(Tokenizer::new("*").next_token(), Token::Asterisk);
    assert_eq!(Tokenizer::new("(").next_token(), Token::OpenParenthesis);
    assert_eq!(Tokenizer::new(")").next_token(), Token::CloseParenthesis);
    assert_eq!(Tokenizer::new(" ").next_token(), Token::Whitespace);
}

#[test]
fn test_lexer_unknown_characters() {
    assert_eq!(Tokenizer::new("a").next_token(), Token::Unknown('a'));
    assert_eq!(Tokenizer::new("$").next_token(), Token::Unknown('$'));
}

#[test]
fn test_lexer_expressions() {
    impl<'a> Tokenizer<'a> {
        fn assert_tokens_eq(&mut self, tokens: &[Token]) {
            tokens
                .iter()
                .zip(std::iter::repeat_with(|| self.next_token()))
                .for_each(|(a, b)| assert_eq!(*a, b));
        }
    }

    Tokenizer::new("(+ 123 245)").assert_tokens_eq(&[
        Token::OpenParenthesis,
        Token::Plus,
        Token::Whitespace,
        Token::Int(123),
        Token::Whitespace,
        Token::Int(245),
        Token::CloseParenthesis,
    ]);

    Tokenizer::new("(* 398 4788)").assert_tokens_eq(&[
        Token::OpenParenthesis,
        Token::Asterisk,
        Token::Whitespace,
        Token::Int(398),
        Token::Whitespace,
        Token::Int(4788),
        Token::CloseParenthesis,
    ]);

    Tokenizer::new("(- (+ 98 4) 788)").assert_tokens_eq(&[
        Token::OpenParenthesis,
        Token::Minus,
        Token::Whitespace,
        Token::OpenParenthesis,
        Token::Plus,
        Token::Whitespace,
        Token::Int(98),
        Token::Whitespace,
        Token::Int(4),
        Token::CloseParenthesis,
        Token::Whitespace,
        Token::Int(788),
        Token::CloseParenthesis,
    ]);
}

#[test]
fn test_parser_binary_expressions() {
    let expr = Tokenizer::new("+ 1 2").parse_binary_expression();
    assert_eq!(
        expr,
        BinExpr {
            op: Operator::Add,
            left: Expr::Number(1),
            right: Expr::Number(2)
        }
    );

    let expr = Tokenizer::new("* 345 478").parse_binary_expression();
    assert_eq!(
        expr,
        BinExpr {
            op: Operator::Multiply,
            left: Expr::Number(345),
            right: Expr::Number(478)
        }
    );
}

#[test]
fn test_parser_expressions() {
    let expr = Tokenizer::new("1").parse_expression();
    assert_eq!(expr, Expr::Number(1));

    let expr = Tokenizer::new("(+ 1 2)").parse_expression();
    assert_eq!(
        expr,
        Expr::BinExprPtr(Box::new(BinExpr {
            op: Operator::Add,
            left: Expr::Number(1),
            right: Expr::Number(2)
        }))
    );

    let expr = Tokenizer::new("(* (+ 123 565) (* (+ 12 3) 134))").parse_expression();
    assert_eq!(
        expr,
        Expr::BinExprPtr(Box::new(BinExpr {
            op: Operator::Multiply,
            left: Expr::BinExprPtr(Box::new(BinExpr {
                op: Operator::Add,
                left: Expr::Number(123),
                right: Expr::Number(565)
            })),
            right: Expr::BinExprPtr(Box::new(BinExpr {
                op: Operator::Multiply,
                left: Expr::BinExprPtr(Box::new(BinExpr {
                    op: Operator::Add,
                    left: Expr::Number(12),
                    right: Expr::Number(3)
                })),
                right: Expr::Number(134)
            }))
        }))
    );
}

#[test]
fn test_eval() {
    assert_eq!(eval(&Tokenizer::new("(+ 9 0)").parse_expression()), 9);
    assert_eq!(eval(&Tokenizer::new("(+ 4 1)").parse_expression()), 5);
    assert_eq!(eval(&Tokenizer::new("(+ 321 48)").parse_expression()), 369);

    assert_eq!(eval(&Tokenizer::new("(* 4 6)").parse_expression()), 24);
    assert_eq!(eval(&Tokenizer::new("(* 24 10)").parse_expression()), 240);

    assert_eq!(
        eval(&Tokenizer::new("(+ (* 23 11) (* 2 (+ 3 4)))").parse_expression()),
        267
    );
}
