use super::*;

#[test]
fn ast() {
    println!(
        "{:#?}",
        Parser::new("(+ (sqr 7) 1 (* 5.64 ").parse_expression()
    );
}

// #[test]
// fn test_parser_binary_expressions() {
//     assert_eq!(
//         Parser::new("+ 1 2").parse_binary_expression(),
//         BinExpr {
//             op: Operator::Add,
//             left: Number(1.0),
//             right: Number(2.0)
//         }
//     );

//     assert_eq!(
//         Parser::new("* 345 478").parse_binary_expression(),
//         BinExpr {
//             op: Operator::Multiply,
//             left: Number(345.0),
//             right: Number(478.0)
//         }
//     );
// }

// #[test]
// fn test_parser_expressions() {
//     assert_eq!(Parser::new("1").parse_expression(), Number(1.0));

//     assert_eq!(
//         Parser::new("(+ 1 2)").parse_expression(),
//         BinExprPtr(Box::new(BinExpr {
//             op: Operator::Add,
//             left: Number(1.0),
//             right: Number(2.0)
//         }))
//     );

//     assert_eq!(
//         Parser::new("(* (+ 123 565) (* (- 12 3) 134))").parse_expression(),
//         BinExprPtr(Box::new(BinExpr {
//             op: Operator::Multiply,
//             left: BinExprPtr(Box::new(BinExpr {
//                 op: Operator::Add,
//                 left: Number(123.0),
//                 right: Number(565.0)
//             })),
//             right: BinExprPtr(Box::new(BinExpr {
//                 op: Operator::Multiply,
//                 left: BinExprPtr(Box::new(BinExpr {
//                     op: Operator::Subtract,
//                     left: Number(12.0),
//                     right: Number(3.0)
//                 })),
//                 right: Number(134.0)
//             }))
//         }))
//     );
// }
