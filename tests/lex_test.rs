

#[test]
fn test_lexer_identifiers() {
    use pyrust::lex::{Lexer,TokenType,Token};
    let input = "let five = 5;
                 let ten = 10;
                 let add = fn(x, y) {
                     x + y;
                 };
                 let result = add(five, ten);";

    let mut lexer = Lexer::new(input);
    let tokens = vec![
        Token::Ident("let".to_string()),
        Token::Ident("five".to_string()),
        Token::Assign,
        Token::Int(5),
        Token::Semicolon,
        Token::Ident("let".to_string()),
        Token::Ident("ten".to_string()),
        Token::Assign,
        Token::Int(10),
        Token::Semicolon,
        Token::Ident("let".to_string()),
        Token::Ident("add".to_string()),
        Token::Assign,
        Token::Ident("fn".to_string()),
        Token::LParen,
        Token::Ident("x".to_string()),
        Token::Comma,
        Token::Ident("y".to_string()),
        Token::RParen,
        Token::LBrace,
        Token::Ident("x".to_string()),
        Token::Plus,
        Token::Ident("y".to_string()),
        Token::Semicolon,
        Token::RBrace,
        Token::Semicolon,
        Token::Ident("let".to_string()),
        Token::Ident("result".to_string()),
        Token::Assign,
        Token::Ident("add".to_string()),
        Token::LParen,
        Token::Ident("five".to_string()),
        Token::Comma,
        Token::Ident("ten".to_string()),
        Token::RParen,
        Token::Semicolon,
        Token::Eof,
    ];

    for expected_token in tokens {
        let token = lexer.next_token();
        assert_eq!(token, expected_token);
    }
}



