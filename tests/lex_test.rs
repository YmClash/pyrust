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
        assert_eq!(lexer.get_token(), Some(TokenType::INTEGER { value: BigInt::from(123) }));
        assert_eq!(lexer.get_token(), Some(TokenType::FLOAT { value: 3.14 }));
    }
    #[test]
    fn test_lex_string_with_escapes() {
        let mut lexer = Lexer::new(r#""Hello, \"world\"""#);
        assert_eq!(
            lexer.get_token(),
            Some(TokenType::STRING {
                value: r#"Hello, "world""#.to_string(),  // La version déséchappée de la chaîne
                kind: StringKind::NORMAL
            })
        );
    }


    // test identifier or keyword
        #[test]
    fn test_lex_identifier_and_keyword() {
        let mut lexer = Lexer::new("variable if");
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "variable".to_string() }));
        assert_eq!(lexer.get_token(), Some(TokenType::KEYWORD(Keywords::IF)));
    }

    #[test]
    fn test_lex_multi_line_comment() {
        let mut lexer = Lexer::new("/* comment */ code");
        assert_eq!(lexer.get_token(), Some(TokenType::COMMENT(" comment ".to_string())));
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "code".to_string() }));
    }

    #[test]
    fn test_lex_comment_with_code_inside() {
        let mut lexer = Lexer::new("/* this is not code: let x = 42; */ actual_code");
        assert_eq!(lexer.get_token(), Some(TokenType::COMMENT(" this is not code: let x = 42; ".to_string())));
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "actual_code".to_string() }));
    }
    #[test]
    fn test_lex_comment() {
        let mut lexer = Lexer::new("# This is a comment\ncode");
        assert_eq!(lexer.get_token(), Some(TokenType::COMMENT(" This is a comment".to_string())));
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "code".to_string() }));
    }
    // #[test]
    // fn test_lex_multiline_string() {
    //     let mut lexer = Lexer::new(r#""This is a \
    //                                    multi-line string""#);
    //     assert_eq!(
    //         lexer.get_token(),
    //         Some(TokenType::STRING {
    //             value: "This is a multi-line string".to_string(),
    //             kind: StringKind::NORMAL
    //         })
    //     );
    // }
    #[test]
    fn test_lex_complex_operator() {
        let mut lexer = Lexer::new("a += 1 && b == c || d != e");
        assert_eq!(
            lexer.get_token(),
            Some(TokenType::IDENTIFIER {
                name: "a".to_string()
            })
        );
        assert_eq!(
            lexer.get_token(),
            Some(TokenType::OPERATOR(Operators::PLUSEQUAL))
        );
        assert_eq!(lexer.get_token(), Some(TokenType::INTEGER { value: 1.into() }));
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::AND)));
        assert_eq!(
            lexer.get_token(),
            Some(TokenType::IDENTIFIER {
                name: "b".to_string()
            })
        );
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::EQEQUAL)));
        assert_eq!(
            lexer.get_token(),
            Some(TokenType::IDENTIFIER {
                name: "c".to_string()
            })
        );
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::OR)));
        assert_eq!(
            lexer.get_token(),
            Some(TokenType::IDENTIFIER {
                name: "d".to_string()
            })
        );
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::NOTEQUAL)));
        assert_eq!(
            lexer.get_token(),
            Some(TokenType::IDENTIFIER {
                name: "e".to_string()
            })
        );
    }


    #[test]
    fn test_lex_delimiters() {
        let mut lexer = Lexer::new("( { [ ] } )");
        assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::LPAR)));
        assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::LCURBRACE)));
        assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::LSBRACKET)));
        assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::RSBRACKET)));
        assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::RCURBRACE)));
        assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::RPAR)));
    }

    #[test]
    fn test_lex_nested_delimiters() {
        let mut lexer = Lexer::new("{[(())]}");
        assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::LCURBRACE)));
        assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::LSBRACKET)));
        assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::LPAR)));
        assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::LPAR)));
        assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::RPAR)));
        assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::RPAR)));
        assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::RSBRACKET)));
        assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::RCURBRACE)));
    }


    #[test]
    fn test_lex_unexpected_character() {
        let mut lexer = Lexer::new("@ $");
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::AT)));
        assert_eq!(lexer.get_token(), Some(TokenType::UNKNOWN));
    }

    #[test]
    fn test_mixed_operators_and_unknown() {
        let mut lexer = Lexer::new("@$");
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::AT)));
        assert_eq!(lexer.get_token(), Some(TokenType::UNKNOWN));
    }

    #[test]
    fn test_arobase() {
        let mut lexer = Lexer::new("@");
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::AT)));
    }
    #[test]
    fn test_lex_mixed_input() {
        let mut lexer = Lexer::new(r#"let sum = a + b * (c - d) / e;"#);
        assert_eq!(lexer.get_token(), Some(TokenType::KEYWORD(Keywords::LET)));
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "sum".to_string() }));
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::EQUAL)));
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "a".to_string() }));
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::PLUS)));
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "b".to_string() }));
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::STAR)));
        assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::LPAR)));
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "c".to_string() }));
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::MINUS)));
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "d".to_string() }));
        assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::RPAR)));
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::SLASH)));
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "e".to_string() }));
        assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::SEMICOLON)));
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

//by YmC