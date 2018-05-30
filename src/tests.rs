use super::*;

#[test]
fn test_lexer_integers() {
    let mut rdr = Tokenizer::new("1234567890");

    assert_eq!(rdr.next_token(), Token::Int(1234567890));
}

#[test]
fn test_lexer_singletons() {
    assert_eq!(Tokenizer::new("+").next_token(), Token::Plus);
    assert_eq!(Tokenizer::new("*").next_token(), Token::Asterisk);
    assert_eq!(Tokenizer::new("(").next_token(), Token::OpenParenthesis);
    assert_eq!(Tokenizer::new(")").next_token(), Token::CloseParenthesis);
    assert_eq!(Tokenizer::new(" ").next_token(), Token::Whitespace);
}

#[test]
fn test_lexer_unknown_characters() {
    assert_eq!(Tokenizer::new("a").next_token(), Token::Unknown);
    assert_eq!(Tokenizer::new("$").next_token(), Token::Unknown);
}

#[test]
fn test_lexer_expressions() {
    let mut rdr = Tokenizer::new("(+ 123 245)");
    assert_eq!(rdr.next_token(), Token::OpenParenthesis);
    assert_eq!(rdr.next_token(), Token::Plus);
    assert_eq!(rdr.next_token(), Token::Whitespace);
    assert_eq!(rdr.next_token(), Token::Int(123));
    assert_eq!(rdr.next_token(), Token::Whitespace);
    assert_eq!(rdr.next_token(), Token::Int(245));
    assert_eq!(rdr.next_token(), Token::CloseParenthesis);

    let mut rdr = Tokenizer::new("(* 398 4788)");
    assert_eq!(rdr.next_token(), Token::OpenParenthesis);
    assert_eq!(rdr.next_token(), Token::Asterisk);
    assert_eq!(rdr.next_token(), Token::Whitespace);
    assert_eq!(rdr.next_token(), Token::Int(398));
    assert_eq!(rdr.next_token(), Token::Whitespace);
    assert_eq!(rdr.next_token(), Token::Int(4788));
    assert_eq!(rdr.next_token(), Token::CloseParenthesis);
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
    assert_eq!(expr, Expr::BinExpr {
        op: Operator::Add,
        left: Expr::Number(1),
        right: Expr::Number(2)
    });

    // let expr = Tokenizer::new("(* (+ 123 565) (* (+ 12 3) 134))").parse_expression();
    // assert_eq!(expr, Expr::BinExpr {
    //     op: Operator::Multiply,
    //     left: Expr::BinExpr {
    //         op: Operator::Add,
    //         left: Expr::Number(123),
    //         right: Expr::Number(565)
    //     },
    //     right: Expr::BinExpr {
    //         op: Operator::Multiply,
    //         left: BinExpr {
    //             op: Operator::Add,
    //             left: Expr::Number(12),
    //             right: Expr::Number(3)
    //         },
    //         right: Expr::Number(134)
    //     }
    // });
}

// #[test]
// fn test_eval() {
//     //FIXME: recursion broke everything, fix this shit
//     assert_eq!(
//         eval(&BinExpr {
//             op: Operator::Add,
//             left: 9,
//             right: 0
//         }),
//         9
//     );
//     assert_eq!(
//         eval(&BinExpr {
//             op: Operator::Add,
//             left: 4,
//             right: 1
//         }),
//         5
//     );
//     assert_eq!(
//         eval(&BinExpr {
//             op: Operator::Add,
//             left: 3,
//             right: 5
//         }),
//         8
//     );

//     assert_eq!(
//         eval(&BinExpr {
//             op: Operator::Multiply,
//             left: 4,
//             right: 1
//         }),
//         4
//     );
//     assert_eq!(
//         eval(&BinExpr {
//             op: Operator::Multiply,
//             left: 3,
//             right: 5
//         }),
//         15
//     );
//     assert_eq!(
//         eval(&BinExpr {
//             op: Operator::Multiply,
//             left: 9,
//             right: 0
//         }),
//         0
//     );
// }