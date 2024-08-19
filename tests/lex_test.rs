use pyrust::lexer::lex::Lexer;
use pyrust::lexer::tok::{TokenType, Keywords, Operators, Delimiters, };
use num_bigint::BigInt;
#[cfg(test)]
mod tests {
    use pyrust::tok::StringKind;
    use super::*;

    #[test]
    fn test_lex_number() {
        let mut lexer = Lexer::new("123 3.14");
        assert_eq!(lexer.get_token(), Some(TokenType::INTEGER { value: 123.into() }));
        assert_eq!(lexer.get_token(), Some(TokenType::FLOAT { value: 3.14 }));
    }

    #[test]
    fn test_lex_identifier_and_keyword() {
        let mut lexer = Lexer::new("variable if");
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "variable".to_string() }));
        assert_eq!(lexer.get_token(), Some(TokenType::KEYWORD(Keywords::IF)));
    }

    #[test]
    fn test_lex_comment() {
        let mut lexer = Lexer::new("# This is a comment\ncode");
        assert_eq!(lexer.get_token(), Some(TokenType::COMMENT(" This is a comment".to_string())));
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "code".to_string() }));
    }

    #[test]
    fn test_basic_tokens() {
        let input = r#"
            let x = 42;
            let y = 3.14;
            if (x > y) {
                print("x is greater");
            } else {
                print("y is greater or equal");
            }
            // This is a comment
            /* This is a
               multi-line comment */
        "#;

        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        // Imprimez les tokens pour le débogage
        for token in &tokens {
            println!("{:?}", token);
        }

        // Vérifions les tokens un par un
        let expected_tokens = vec![
            TokenType::KEYWORD(Keywords::LET),
            TokenType::IDENTIFIER { name: "x".to_string() },
            TokenType::OPERATOR(Operators::EQUAL),
            TokenType::INTEGER { value: BigInt::from(42) },
            TokenType::DELIMITER(Delimiters::SEMICOLON),
            TokenType::KEYWORD(Keywords::LET),
            TokenType::IDENTIFIER { name: "y".to_string() },
            TokenType::OPERATOR(Operators::EQUAL),
            TokenType::FLOAT { value: 3.14 },
            TokenType::DELIMITER(Delimiters::SEMICOLON),
            TokenType::KEYWORD(Keywords::IF),
            TokenType::DELIMITER(Delimiters::LPAR),
            TokenType::IDENTIFIER { name: "x".to_string() },
            TokenType::OPERATOR(Operators::GREATER),
            TokenType::IDENTIFIER { name: "y".to_string() },
            TokenType::DELIMITER(Delimiters::RPAR),
            TokenType::DELIMITER(Delimiters::LCURBRACE),
            TokenType::IDENTIFIER { name: "print".to_string() },
            TokenType::DELIMITER(Delimiters::LPAR),
            TokenType::STRING { value: "x is greater".to_string(), kind: StringKind::NORMAL },
            TokenType::DELIMITER(Delimiters::RPAR),
            TokenType::DELIMITER(Delimiters::SEMICOLON),
            TokenType::DELIMITER(Delimiters::RCURBRACE),
            TokenType::KEYWORD(Keywords::ELSE),
            TokenType::DELIMITER(Delimiters::LCURBRACE),
            TokenType::IDENTIFIER { name: "print".to_string() },
            TokenType::DELIMITER(Delimiters::LPAR),
            TokenType::STRING { value: "y is greater or equal".to_string(), kind: StringKind::NORMAL },
            TokenType::DELIMITER(Delimiters::RPAR),
            TokenType::DELIMITER(Delimiters::SEMICOLON),
            TokenType::DELIMITER(Delimiters::RCURBRACE),
            TokenType::COMMENT(" This is a comment".to_string()),
            TokenType::COMMENT(" This is a\n               multi-line comment ".to_string()),
            TokenType::EOF,
        ];

        assert_eq!(tokens.len(), expected_tokens.len(), "Le nombre de tokens ne correspond pas");

        for (token, expected_type) in tokens.iter().zip(expected_tokens.iter()) {
            assert_eq!(&token.token_type, expected_type, "Type de token incorrect pour '{}'", token.text);
        }
    }
}