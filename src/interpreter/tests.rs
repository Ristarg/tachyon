use super::*;

#[test]
fn test_eval_expr() {
    assert_eq!(eval_expr(&Parser::new("(+ 9 0)").parse_expression()), 9.0);
    assert_eq!(eval_expr(&Parser::new("(+ 4 1)").parse_expression()), 5.0);
    assert_eq!(
        eval_expr(&Parser::new("(+ 321 48)").parse_expression()),
        369.0
    );

    assert_eq!(eval_expr(&Parser::new("(* 4 6)").parse_expression()), 24.0);
    assert_eq!(
        eval_expr(&Parser::new("(* 24 10)").parse_expression()),
        240.0
    );

    assert_eq!(
        eval_expr(&Parser::new("(+ (* 23 11) (* 2 (- 3 4)))").parse_expression()),
        251.0
    );
}
