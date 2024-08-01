
#[test]
fn test_lexer_identifiers() {
    use pyrust::lex::{Lexer, Token, TokenType};
    let input = "let five = 5;
                 let ten = 10;
                 let add = fn(x, y) {
                     x + y;
                 };
                 let result = add(five, ten);";

    let mut lexer = Lexer::new(input);
    let tokens = vec![
        Token::new("let".to_string(), TokenType::LET),
        Token::new("five".to_string(), TokenType::IDENT),
        Token::new("=".to_string(), TokenType::EQ),
        Token::new("5".to_string(), TokenType::NUMBER),
        Token::new(";".to_string(), TokenType::SEMICOLON),
        Token::new("let".to_string(), TokenType::LET),
        Token::new("ten".to_string(), TokenType::IDENT),
        Token::new("=".to_string(), TokenType::EQ),
        Token::new("10".to_string(), TokenType::NUMBER),
        Token::new(";".to_string(), TokenType::SEMICOLON),
        Token::new("let".to_string(), TokenType::LET),
        Token::new("add".to_string(), TokenType::IDENT),
        Token::new("=".to_string(), TokenType::EQ),
        Token::new("fn".to_string(), TokenType::FN),
        Token::new("(".to_string(), TokenType::LPAREN),
        Token::new("x".to_string(), TokenType::IDENT),
        Token::new(",".to_string(), TokenType::COMMA),
        Token::new("y".to_string(), TokenType::IDENT),
        Token::new(")".to_string(), TokenType::RPAREN),
        Token::new("{".to_string(), TokenType::LCURBRACET),
        Token::new("x".to_string(), TokenType::IDENT),
        Token::new("+".to_string(), TokenType::PLUS),
        Token::new("y".to_string(), TokenType::IDENT),
        Token::new(";".to_string(), TokenType::SEMICOLON),
        Token::new("}".to_string(), TokenType::RCURBRACET),
        Token::new(";".to_string(), TokenType::SEMICOLON),
        Token::new("let".to_string(), TokenType::LET),
        Token::new("result".to_string(), TokenType::IDENT),
        Token::new("=".to_string(), TokenType::EQ),
        Token::new("add".to_string(), TokenType::IDENT),
        Token::new("(".to_string(), TokenType::LPAREN),
        Token::new("five".to_string(), TokenType::IDENT),
        Token::new(",".to_string(), TokenType::COMMA),
        Token::new("ten".to_string(), TokenType::IDENT),
        Token::new(")".to_string(), TokenType::RPAREN),
        Token::new(";".to_string(), TokenType::SEMICOLON),
        Token::new("".to_string(), TokenType::EOF),
    ];

    for expected_token in tokens {
        let token = lexer.get_token();
        assert_eq!(token, expected_token);
    }
}

#[test]
fn test_lexer_numbers() {
    use pyrust::lex::{Lexer, Token, TokenType};
    let input = "123 456 789";

    let mut lexer = Lexer::new(input);
    let tokens = vec![
        Token::new("123".to_string(), TokenType::NUMBER),
        Token::new("456".to_string(), TokenType::NUMBER),
        Token::new("789".to_string(), TokenType::NUMBER),
        Token::new("".to_string(), TokenType::EOF),
    ];

    for expected_token in tokens {
        let token = lexer.get_token();
        assert_eq!(token, expected_token);
    }
}


#[test]
fn text_numbers() {

}


