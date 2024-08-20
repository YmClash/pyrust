use pyrust::lexer::lex::Lexer;
use pyrust::lexer::tok::{TokenType, Keywords, Operators, Delimiters, StringKind};
use num_bigint::BigInt;

#[cfg(test)]
mod tests {
    use super::*;

    // Test pour les nombres
    #[test]
    fn test_lex_number() {
        let mut lexer = Lexer::new("123 3.14");
        assert_eq!(lexer.get_token(), Some(TokenType::INTEGER { value: BigInt::from(123) }));
        assert_eq!(lexer.get_token(), Some(TokenType::FLOAT { value: 3.14 }));
    }

    // Test pour les chaînes avec des séquences d'échappement
    #[test]
    fn test_lex_string_with_escapes() {
        let mut lexer = Lexer::new(r#""Hello, \"world\"""#);
        assert_eq!(
            lexer.get_token(),
            Some(TokenType::STRING {
                value: r#"Hello, "world""#.to_string(),
                kind: StringKind::NORMAL
            })
        );
    }

    // Test pour les identifiants et les mots-clés
    #[test]
    fn test_lex_identifier_and_keyword() {
        let mut lexer = Lexer::new("variable if");
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "variable".to_string() }));
        assert_eq!(lexer.get_token(), Some(TokenType::KEYWORD(Keywords::IF)));
    }

    // Test pour les commentaires multi-lignes
    #[test]
    fn test_lex_multi_line_comment() {
        let mut lexer = Lexer::new("/* comment */ code");
        assert_eq!(lexer.get_token(), Some(TokenType::COMMENT(" comment ".to_string())));
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "code".to_string() }));
    }

    // Test pour les commentaires avec du code à l'intérieur
    #[test]
    fn test_lex_comment_with_code_inside() {
        let mut lexer = Lexer::new("/* this is not code: let x = 42; */ actual_code");
        assert_eq!(lexer.get_token(), Some(TokenType::COMMENT(" this is not code: let x = 42; ".to_string())));
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "actual_code".to_string() }));
    }

    // Test pour les commentaires sur une seule ligne
    #[test]
    fn test_lex_comment() {
        let mut lexer = Lexer::new("# This is a comment\ncode");
        assert_eq!(lexer.get_token(), Some(TokenType::COMMENT(" This is a comment".to_string())));
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "code".to_string() }));
    }

    // Test pour les chaînes multi-lignes
    #[test]
    fn test_lex_multiline_string() {
        let mut lexer = Lexer::new(r#""This is a \
                                       multi-line string""#);
        assert_eq!(
            lexer.get_token(),
            Some(TokenType::STRING {
                value: "This is a multi-line string".to_string(),
                kind: StringKind::NORMAL
            })
        );
    }

    // Test pour les opérateurs complexes
    #[test]
    fn test_lex_complex_operator() {
        let mut lexer = Lexer::new("a += 1 && b == c || d != e");
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "a".to_string() }));
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::PLUSEQUAL)));
        assert_eq!(lexer.get_token(), Some(TokenType::INTEGER { value: BigInt::from(1) }));
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::AND)));
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "b".to_string() }));
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::EQEQUAL)));
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "c".to_string() }));
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::OR)));
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "d".to_string() }));
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::NOTEQUAL)));
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "e".to_string() }));
    }

    // Test pour les délimiteurs
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

    // Test pour les délimiteurs imbriqués
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

    // Test pour les caractères inattendus
    #[test]
    fn test_lex_unexpected_character() {
        let mut lexer = Lexer::new("@ $");
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::AT)));
        assert_eq!(lexer.get_token(), Some(TokenType::UNKNOWN));
    }

    // Test pour un mélange d'opérateurs et de caractères inconnus
    #[test]
    fn test_mixed_operators_and_unknown() {
        let mut lexer = Lexer::new("@$");
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::AT)));
        assert_eq!(lexer.get_token(), Some(TokenType::UNKNOWN));
    }

    // Test pour l'opérateur @ seul
    #[test]
    fn test_arobase() {
        let mut lexer = Lexer::new("@");
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::AT)));
    }

    // Test pour un mélange d'opérateurs et de délimiteurs
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

    // Test pour des tokens de base avec des commentaires et des chaînes multi-lignes
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

    // Test pour les chaînes vides
    #[test]
    fn test_empty_string() {
        let mut lexer = Lexer::new(r#""""#);
        assert_eq!(
            lexer.get_token(),
            Some(TokenType::STRING {
                value: "".to_string(),
                kind: StringKind::NORMAL
            })
        );
    }

    // Test pour les nombres négatifs
    #[test]
    fn test_negative_numbers() {
        let mut lexer = Lexer::new("-42 -3.14");
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::MINUS)));
        assert_eq!(lexer.get_token(), Some(TokenType::INTEGER { value: BigInt::from(42) }));
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::MINUS)));
        assert_eq!(lexer.get_token(), Some(TokenType::FLOAT { value: 3.14 }));
    }

    // Test pour les identifiants avec des underscores
    #[test]
    fn test_identifiers_with_underscores() {
        let mut lexer = Lexer::new("my_variable _private");
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "my_variable".to_string() }));
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "_private".to_string() }));
    }

    // Test pour les opérateurs composés
    #[test]
    fn test_compound_operators() {
        let mut lexer = Lexer::new("++ -- ** //");
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::PLUSEQUAL)));
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::MINEQUAL)));
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::DOUBLESTAR)));
        assert_eq!(lexer.get_token(), Some(TokenType::COMMENT("".to_string())));
    }

    // Test pour la gestion des espaces et des sauts de ligne
    #[test]
    fn test_whitespace_handling() {
        let mut lexer = Lexer::new("let   x\n=\t42");
        assert_eq!(lexer.get_token(), Some(TokenType::KEYWORD(Keywords::LET)));
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "x".to_string() }));
        assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::EQUAL)));
        assert_eq!(lexer.get_token(), Some(TokenType::INTEGER { value: BigInt::from(42) }));
    }

    // Test pour les caractères d'échappement dans les chaînes
    #[test]
    fn test_string_escape_sequences() {
        let mut lexer = Lexer::new(r#""Hello\nWorld\t\"Escaped\"""#);
        assert_eq!(
            lexer.get_token(),
            Some(TokenType::STRING {
                value: "Hello\nWorld\t\"Escaped\"".to_string(),
                kind: StringKind::NORMAL
            })
        );
    }

    // Test pour les chaînes multi-lignes complexes
    #[test]
    fn test_multiline_strings() {
        let input = r#"
        "This is a simple string"
        "This is a string \
         that spans multiple \
         lines"
        "This string has
         actual newlines"
        "This string has \n escaped newlines"
        "Mixed \
         newlines \n and \
         continuations"
    "#;

        let mut lexer = Lexer::new(input);

        assert_eq!(
            lexer.get_token(),
            Some(TokenType::STRING {
                value: "This is a simple string".to_string(),
                kind: StringKind::NORMAL
            })
        );

        assert_eq!(
            lexer.get_token(),
            Some(TokenType::STRING {
                value: "This is a string that spans multiple lines".to_string(),
                kind: StringKind::NORMAL
            })
        );

        assert_eq!(
            lexer.get_token(),
            Some(TokenType::STRING {
                value: "This string has\n         actual newlines".to_string(),
                kind: StringKind::NORMAL
            })
        );

        assert_eq!(
            lexer.get_token(),
            Some(TokenType::STRING {
                value: "This string has \n escaped newlines".to_string(),
                kind: StringKind::NORMAL
            })
        );

        assert_eq!(
            lexer.get_token(),
            Some(TokenType::STRING {
                value: "Mixed newlines \n and continuations".to_string(),
                kind: StringKind::NORMAL
            })
        );

        assert_eq!(lexer.get_token(), Some(TokenType::EOF));
    }

    // Test pour les commentaires et les commentaires de documentation
    #[test]
    fn test_comments_and_doc_comments() {
        let mut lexer = Lexer::new("// This is a regular comment\n/// This is a doc comment\n/* Multi-line comment */");
        assert_eq!(lexer.get_token(), Some(TokenType::COMMENT(" This is a regular comment".to_string())));
        assert_eq!(lexer.get_token(), Some(TokenType::DOCSTRING(" This is a doc comment".to_string())));
        assert_eq!(lexer.get_token(), Some(TokenType::COMMENT(" Multi-line comment ".to_string())));
    }

    #[test]
    fn test_invalid_identifier() {
        let mut lexer = Lexer::new("var$");
        assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "var".to_string() }));
        assert_eq!(lexer.get_token(), Some(TokenType::UNKNOWN)); // `$` n'est pas un caractère valide dans un identifiant
    }
}


//*************************************************************
// use pyrust::lexer::lex::Lexer;
// use pyrust::lexer::tok::{TokenType, Keywords, Operators, Delimiters, };
// use num_bigint::BigInt;
// #[cfg(test)]
// mod tests {
//     use pyrust::tok::StringKind;
//     use super::*;
//
//     // test pour les nombres
//     #[test]
//     fn test_lex_number() {
//         let mut lexer = Lexer::new("123 3.14");
//         assert_eq!(lexer.get_token(), Some(TokenType::INTEGER { value: BigInt::from(123) }));
//         assert_eq!(lexer.get_token(), Some(TokenType::FLOAT { value: 3.14 }));
//     }
//     // test pour les chaines avec des espaces
//     #[test]
//     fn test_lex_string_with_escapes() {
//         let mut lexer = Lexer::new(r#""Hello, \"world\"""#);
//         assert_eq!(
//             lexer.get_token(),
//             Some(TokenType::STRING {
//                 value: r#"Hello, "world""#.to_string(),  // La version déséchappée de la chaîne
//                 kind: StringKind::NORMAL
//             })
//         );
//     }
//
//
//     // test  pour les indentifiants et les mots clés
//         #[test]
//     fn test_lex_identifier_and_keyword() {
//         let mut lexer = Lexer::new("variable if");
//         assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "variable".to_string() }));
//         assert_eq!(lexer.get_token(), Some(TokenType::KEYWORD(Keywords::IF)));
//     }
//
//     //test pour les commentaires
//     #[test]
//     fn test_lex_multi_line_comment() {
//         let mut lexer = Lexer::new("/* comment */ code");
//         assert_eq!(lexer.get_token(), Some(TokenType::COMMENT(" comment ".to_string())));
//         assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "code".to_string() }));
//     }
//
//     //test pour les commentaires avec du code à l'intérieur
//
//     #[test]
//     fn test_lex_comment_with_code_inside() {
//         let mut lexer = Lexer::new("/* this is not code: let x = 42; */ actual_code");
//         assert_eq!(lexer.get_token(), Some(TokenType::COMMENT(" this is not code: let x = 42; ".to_string())));
//         assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "actual_code".to_string() }));
//     }
//     //test pour les commentaires sur une seule ligne
//     #[test]
//     fn test_lex_comment() {
//         let mut lexer = Lexer::new("# This is a comment\ncode");
//         assert_eq!(lexer.get_token(), Some(TokenType::COMMENT(" This is a comment".to_string())));
//         assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "code".to_string() }));
//     }
//     ///////////////////////////////////////
//     #[test]
//     fn test_lex_multiline_string() {
//         let mut lexer = Lexer::new(r#""This is a \
//                                        multi-line string""#);
//         assert_eq!(
//             lexer.get_token(),
//             Some(TokenType::STRING {
//                 value: "This is a multi-line string".to_string(),
//
//                 kind: StringKind::NORMAL
//             })
//         );
//     }
//     ////////////////////////
//     #[test]
//     fn test_lex_complex_operator() {
//         let mut lexer = Lexer::new("a += 1 && b == c || d != e");
//         assert_eq!(
//             lexer.get_token(),
//             Some(TokenType::IDENTIFIER {
//                 name: "a".to_string()
//             })
//         );
//         assert_eq!(
//             lexer.get_token(),
//             Some(TokenType::OPERATOR(Operators::PLUSEQUAL))
//         );
//         assert_eq!(lexer.get_token(), Some(TokenType::INTEGER { value: 1.into() }));
//         assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::AND)));
//         assert_eq!(
//             lexer.get_token(),
//             Some(TokenType::IDENTIFIER {
//                 name: "b".to_string()
//             })
//         );
//         assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::EQEQUAL)));
//         assert_eq!(
//             lexer.get_token(),
//             Some(TokenType::IDENTIFIER {
//                 name: "c".to_string()
//             })
//         );
//         assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::OR)));
//         assert_eq!(
//             lexer.get_token(),
//             Some(TokenType::IDENTIFIER {
//                 name: "d".to_string()
//             })
//         );
//         assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::NOTEQUAL)));
//         assert_eq!(
//             lexer.get_token(),
//             Some(TokenType::IDENTIFIER {
//                 name: "e".to_string()
//             })
//         );
//     }
//
//
//     #[test]
//     fn test_lex_delimiters() {
//         let mut lexer = Lexer::new("( { [ ] } )");
//         assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::LPAR)));
//         assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::LCURBRACE)));
//         assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::LSBRACKET)));
//         assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::RSBRACKET)));
//         assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::RCURBRACE)));
//         assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::RPAR)));
//     }
//
//     #[test]
//     fn test_lex_nested_delimiters() {
//         let mut lexer = Lexer::new("{[(())]}");
//         assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::LCURBRACE)));
//         assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::LSBRACKET)));
//         assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::LPAR)));
//         assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::LPAR)));
//         assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::RPAR)));
//         assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::RPAR)));
//         assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::RSBRACKET)));
//         assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::RCURBRACE)));
//     }
//
//
//     #[test]
//     fn test_lex_unexpected_character() {
//         let mut lexer = Lexer::new("@ $");
//         assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::AT)));
//         assert_eq!(lexer.get_token(), Some(TokenType::UNKNOWN));
//     }
//
//     #[test]
//     fn test_mixed_operators_and_unknown() {
//         let mut lexer = Lexer::new("@$");
//         assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::AT)));
//         assert_eq!(lexer.get_token(), Some(TokenType::UNKNOWN));
//     }
//
//     #[test]
//     fn test_arobase() {
//         let mut lexer = Lexer::new("@");
//         assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::AT)));
//     }
//     #[test]
//     fn test_lex_mixed_input() {
//         let mut lexer = Lexer::new(r#"let sum = a + b * (c - d) / e;"#);
//         assert_eq!(lexer.get_token(), Some(TokenType::KEYWORD(Keywords::LET)));
//         assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "sum".to_string() }));
//         assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::EQUAL)));
//         assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "a".to_string() }));
//         assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::PLUS)));
//         assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "b".to_string() }));
//         assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::STAR)));
//         assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::LPAR)));
//         assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "c".to_string() }));
//         assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::MINUS)));
//         assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "d".to_string() }));
//         assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::RPAR)));
//         assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::SLASH)));
//         assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "e".to_string() }));
//         assert_eq!(lexer.get_token(), Some(TokenType::DELIMITER(Delimiters::SEMICOLON)));
//     }
//
//
//
//     #[test]
//     fn test_basic_tokens() {
//         let input = r#"
//             let x = 42;
//             let y = 3.14;
//             if (x > y) {
//                 print("x is greater");
//             } else {
//                 print("y is greater or equal");
//             }
//
//             // This is a comment
//             /* This is a
//                multi-line comment */
//         "#;
//
//         let mut lexer = Lexer::new(input);
//         let tokens = lexer.tokenize();
//
//         // Imprimez les tokens pour le débogage
//         for token in &tokens {
//             println!("{:?}", token);
//         }
//
//         // Vérifions les tokens un par un
//         let expected_tokens = vec![
//             TokenType::KEYWORD(Keywords::LET),
//             TokenType::IDENTIFIER { name: "x".to_string() },
//             TokenType::OPERATOR(Operators::EQUAL),
//             TokenType::INTEGER { value: BigInt::from(42) },
//             TokenType::DELIMITER(Delimiters::SEMICOLON),
//             TokenType::KEYWORD(Keywords::LET),
//             TokenType::IDENTIFIER { name: "y".to_string() },
//             TokenType::OPERATOR(Operators::EQUAL),
//             TokenType::FLOAT { value: 3.14 },
//             TokenType::DELIMITER(Delimiters::SEMICOLON),
//             TokenType::KEYWORD(Keywords::IF),
//             TokenType::DELIMITER(Delimiters::LPAR),
//             TokenType::IDENTIFIER { name: "x".to_string() },
//             TokenType::OPERATOR(Operators::GREATER),
//             TokenType::IDENTIFIER { name: "y".to_string() },
//             TokenType::DELIMITER(Delimiters::RPAR),
//             TokenType::DELIMITER(Delimiters::LCURBRACE),
//             TokenType::IDENTIFIER { name: "print".to_string() },
//             TokenType::DELIMITER(Delimiters::LPAR),
//             TokenType::STRING { value: "x is greater".to_string(), kind: StringKind::NORMAL },
//             TokenType::DELIMITER(Delimiters::RPAR),
//             TokenType::DELIMITER(Delimiters::SEMICOLON),
//             TokenType::DELIMITER(Delimiters::RCURBRACE),
//             TokenType::KEYWORD(Keywords::ELSE),
//             TokenType::DELIMITER(Delimiters::LCURBRACE),
//             TokenType::IDENTIFIER { name: "print".to_string() },
//             TokenType::DELIMITER(Delimiters::LPAR),
//             TokenType::STRING { value: "y is greater or equal".to_string(), kind: StringKind::NORMAL },
//             TokenType::DELIMITER(Delimiters::RPAR),
//             TokenType::DELIMITER(Delimiters::SEMICOLON),
//             TokenType::DELIMITER(Delimiters::RCURBRACE),
//             TokenType::COMMENT(" This is a comment".to_string()),
//             TokenType::COMMENT(" This is a\n               multi-line comment ".to_string()),
//             TokenType::EOF,
//         ];
//
//         assert_eq!(tokens.len(), expected_tokens.len(), "Le nombre de tokens ne correspond pas");
//
//         for (token, expected_type) in tokens.iter().zip(expected_tokens.iter()) {
//             assert_eq!(&token.token_type, expected_type, "Type de token incorrect pour '{}'", token.text);
//         }
//     }
//
//     // test pour les chaines vides
//     #[test]
//     fn test_empty_string() {
//         let mut lexer = Lexer::new(r#""""#);
//         assert_eq!(
//             lexer.get_token(),
//             Some(TokenType::STRING {
//                 value: "".to_string(),
//                 kind: StringKind::NORMAL
//             })
//         );
//     }
//
//     // test nombre negatif
//     #[test]
//     fn test_negative_numbers() {
//         let mut lexer = Lexer::new("-42 -3.14");
//         assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::MINUS)));
//         assert_eq!(lexer.get_token(), Some(TokenType::INTEGER { value: BigInt::from(42) }));
//         assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::MINUS)));
//         assert_eq!(lexer.get_token(), Some(TokenType::FLOAT { value: 3.14 }));
//     }
//
//
//     //Test pour les identifiants avec des underscores :
//     #[test]
//     fn test_identifiers_with_underscores() {
//         let mut lexer = Lexer::new("my_variable _private");
//         assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "my_variable".to_string() }));
//         assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "_private".to_string() }));
//     }
//
//    //Test pour les opérateurs composés :
//     #[test]
//     fn test_compound_operators() {
//         let mut lexer = Lexer::new("++ -- ** //");
//         assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::PLUSEQUAL)));
//         assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::MINEQUAL)));
//         assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::DOUBLESTAR)));
//         assert_eq!(lexer.get_token(), Some(TokenType::COMMENT("".to_string())));
//     }
//
//     //Test pour la gestion des espaces et des sauts de ligne :
//
//     #[test]
//     fn test_whitespace_handling() {
//         let mut lexer = Lexer::new("let   x\n=\t42");
//         assert_eq!(lexer.get_token(), Some(TokenType::KEYWORD(Keywords::LET)));
//         assert_eq!(lexer.get_token(), Some(TokenType::IDENTIFIER { name: "x".to_string() }));
//         assert_eq!(lexer.get_token(), Some(TokenType::OPERATOR(Operators::EQUAL)));
//         assert_eq!(lexer.get_token(), Some(TokenType::INTEGER { value: BigInt::from(42) }));
//     }
//
//     //Test pour les caractères d'échappement dans les chaînes :
//
//     #[test]
//     fn test_string_escape_sequences() {
//         let mut lexer = Lexer::new(r#""Hello\nWorld\t\"Escaped\"""#);
//         assert_eq!(
//             lexer.get_token(),
//             Some(TokenType::STRING {
//                 value: "Hello\nWorld\t\"Escaped\"".to_string(),
//                 kind: StringKind::NORMAL
//             })
//         );
//     }
//
//     #[test]
//     fn test_multiline_strings() {
//         let input = r#"
//         "This is a simple string"
//         "This is a string \
//          that spans multiple \
//          lines"
//         "This string has
//          actual newlines"
//         "This string has \n escaped newlines"
//         "Mixed \
//          newlines \n and \
//          continuations"
//     "#;
//
//         let mut lexer = Lexer::new(input);
//
//         assert_eq!(
//             lexer.get_token(),
//             Some(TokenType::STRING {
//                 value: "This is a simple string".to_string(),
//                 kind: StringKind::NORMAL
//             })
//         );
//
//         assert_eq!(
//             lexer.get_token(),
//             Some(TokenType::STRING {
//                 value: "This is a string that spans multiple lines".to_string(),
//                 kind: StringKind::NORMAL
//             })
//         );
//
//         assert_eq!(
//             lexer.get_token(),
//             Some(TokenType::STRING {
//                 value: "This string has\n         actual newlines".to_string(),
//                 kind: StringKind::NORMAL
//             })
//         );
//
//         assert_eq!(
//             lexer.get_token(),
//             Some(TokenType::STRING {
//                 value: "This string has \n escaped newlines".to_string(),
//                 kind: StringKind::NORMAL
//             })
//         );
//
//         assert_eq!(
//             lexer.get_token(),
//             Some(TokenType::STRING {
//                 value: "Mixed newlines \n and continuations".to_string(),
//                 kind: StringKind::NORMAL
//             })
//         );
//
//         assert_eq!(lexer.get_token(), Some(TokenType::EOF));
//     }
//
//
//     #[test]
//     fn test_comments_and_doc_comments() {
//         let mut lexer = Lexer::new("// This is a regular comment\n/// This is a doc comment\n/* Multi-line comment */");
//         assert_eq!(lexer.get_token(), Some(TokenType::COMMENT(" This is a regular comment".to_string())));
//         assert_eq!(lexer.get_token(), Some(TokenType::DOCSTRING(" This is a doc comment".to_string())));
//         assert_eq!(lexer.get_token(), Some(TokenType::COMMENT(" Multi-line comment ".to_string())));
//     }
//
//
//
//
// }
//
// //by YmC