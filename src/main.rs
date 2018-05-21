#[derive(Debug, PartialEq)]
enum Token {
    Int(i64),
}

fn main() {
    let source = load_source();
    let tokens = tokenize_source(&source);
    println!("{:?}", tokens);
}

fn load_source() -> String {
    "1".to_owned()
}

fn tokenize_source(source: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut reader = source.chars().peekable();

    while let Some(&next_char) = reader.peek() {
        match next_char {
            '0'...'9' | '-' => {
                let digits: String = (&mut reader)
                    .take_while(|c| c.is_ascii_digit() || *c == '-') //FIXME: only take the first minus
                    .collect();
                let token = Token::Int(digits.parse().unwrap());
                tokens.push(token);
            }
            _ => {
                println!("Unrecognized token: \"{}\", skipping...", next_char);
                reader.next();
            }
        };
    }

    tokens
}

#[test]
fn test_lexer_single_digit_ints() {
    assert_eq!(tokenize_source("0"), vec![Token::Int(0)]);
    assert_eq!(tokenize_source("1"), vec![Token::Int(1)]);
    assert_eq!(tokenize_source("2"), vec![Token::Int(2)]);
    assert_eq!(tokenize_source("3"), vec![Token::Int(3)]);
    assert_eq!(tokenize_source("4"), vec![Token::Int(4)]);
    assert_eq!(tokenize_source("5"), vec![Token::Int(5)]);
    assert_eq!(tokenize_source("6"), vec![Token::Int(6)]);
    assert_eq!(tokenize_source("7"), vec![Token::Int(7)]);
    assert_eq!(tokenize_source("8"), vec![Token::Int(8)]);
    assert_eq!(tokenize_source("9"), vec![Token::Int(9)]);
}

#[test]
fn test_lexer_multiple_digit_ints() {
    assert_eq!(tokenize_source("10"), vec![Token::Int(10)]);
    assert_eq!(tokenize_source("1456"), vec![Token::Int(1456)]);
    assert_eq!(tokenize_source("234234"), vec![Token::Int(234234)]);
}

#[test]
fn test_lexer_negative_ints() {
    assert_eq!(tokenize_source("-10"), vec![Token::Int(-10)]);
    assert_eq!(tokenize_source("-1456"), vec![Token::Int(-1456)]);
    assert_eq!(tokenize_source("-234234"), vec![Token::Int(-234234)]);
}
