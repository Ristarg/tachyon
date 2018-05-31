use super::{Expr::*, *};

#[test]
fn test_parser_binary_expressions() {
    assert_eq!(
        Parser::new("+ 1 2").parse_binary_expression(),
        BinExpr {
            op: Operator::Add,
            left: Number(1),
            right: Number(2)
        }
    );

    assert_eq!(
        Parser::new("* 345 478").parse_binary_expression(),
        BinExpr {
            op: Operator::Multiply,
            left: Number(345),
            right: Number(478)
        }
    );
}

#[test]
fn test_parser_expressions() {
    assert_eq!(Parser::new("1").parse_expression(), Number(1));

    assert_eq!(
        Parser::new("(+ 1 2)").parse_expression(),
        BinExprPtr(Box::new(BinExpr {
            op: Operator::Add,
            left: Number(1),
            right: Number(2)
        }))
    );

    assert_eq!(
        Parser::new("(* (+ 123 565) (* (- 12 3) 134))").parse_expression(),
        BinExprPtr(Box::new(BinExpr {
            op: Operator::Multiply,
            left: BinExprPtr(Box::new(BinExpr {
                op: Operator::Add,
                left: Number(123),
                right: Number(565)
            })),
            right: BinExprPtr(Box::new(BinExpr {
                op: Operator::Multiply,
                left: BinExprPtr(Box::new(BinExpr {
                    op: Operator::Subtract,
                    left: Number(12),
                    right: Number(3)
                })),
                right: Number(134)
            }))
        }))
    );
}

#[test]
fn test_eval() {
    assert_eq!(eval(&Parser::new("(+ 9 0)").parse_expression()), 9);
    assert_eq!(eval(&Parser::new("(+ 4 1)").parse_expression()), 5);
    assert_eq!(eval(&Parser::new("(+ 321 48)").parse_expression()), 369);

    assert_eq!(eval(&Parser::new("(* 4 6)").parse_expression()), 24);
    assert_eq!(eval(&Parser::new("(* 24 10)").parse_expression()), 240);

    assert_eq!(
        eval(&Parser::new("(+ (* 23 11) (* 2 (- 3 4)))").parse_expression()),
        251
    );
}
