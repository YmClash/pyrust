use pyrust::lexer::lex::Lexer;
use pyrust::lexer::tok::{TokenType, Keywords, Operators, Delimiters, StringKind};
//use pyrust::error::{LexerError, Position, LexerErrorType};
//use num_bigint::BigInt;
//use std::time::Instant;
use pyrust::lex::SyntaxMode;



// #[cfg(test)]
// mod tests {
    use num_bigint::BigInt;

//
//     #[test]
//     fn test_lexer_indentation() {
//         let source_code = r#"
// def example():
//     let x = 10
//     if x > 5:
//         print("Greater than 5")
//     else:
//         print("Less or equal to 5")
//     let y = 20
// "#;
//         let mut lexer = Lexer::new(source_code, SyntaxMode::Indentation);
//         let tokens = lexer.tokenize();
//
//         // Définir les tokens attendus
//         let expected_tokens = vec![
//             TokenType::new("def".to_string(), TokenType::KEYWORD(Keywords::DEF), 2, 1),
//             Token::new("example".to_string(), TokenType::IDENTIFIER { name: "example".to_string() }, 2, 5),
//             Token::new("(".to_string(), TokenType::DELIMITER(Delimiters::LPAR), 2, 12),
//             Token::new(")".to_string(), TokenType::DELIMITER(Delimiters::RPAR), 2, 13),
//             Token::new(":".to_string(), TokenType::DELIMITER(Delimiters::COLON), 2, 14),
//             Token::new("".to_string(), TokenType::NEWLINE, 2, 15),
//             Token::new("".to_string(), TokenType::INDENT, 3, 5), // Indentation niveau 1
//             Token::new("let".to_string(), TokenType::KEYWORD(Keywords::LET), 3, 5),
//             Token::new("x".to_string(), TokenType::IDENTIFIER { name: "x".to_string() }, 3, 9),
//             Token::new("=".to_string(), TokenType::OPERATOR(Operators::EQUAL), 3, 11),
//             Token::new("10".to_string(), TokenType::INTEGER { value: BigInt::from(10) }, 3, 13),
//             Token::new("".to_string(), TokenType::NEWLINE, 3, 15),
//             Token::new("if".to_string(), TokenType::KEYWORD(Keywords::IF), 4, 5),
//             Token::new("x".to_string(), TokenType::IDENTIFIER { name: "x".to_string() }, 4, 8),
//             Token::new(">".to_string(), TokenType::OPERATOR(Operators::GREATER), 4, 10),
//             Token::new("5".to_string(), TokenType::INTEGER { value: BigInt::from(5) }, 4, 12),
//             Token::new(":".to_string(), TokenType::DELIMITER(Delimiters::COLON), 4, 13),
//             Token::new("".to_string(), TokenType::NEWLINE, 4, 14),
//             Token::new("".to_string(), TokenType::INDENT, 5, 9), // Indentation niveau 2
//             Token::new("print".to_string(), TokenType::IDENTIFIER { name: "print".to_string() }, 5, 9),
//             Token::new("(".to_string(), TokenType::DELIMITER(Delimiters::LPAR), 5, 14),
//             Token::new("\"Greater than 5\"".to_string(), TokenType::STRING { value: "Greater than 5".to_string(), kind: StringKind::NORMAL }, 5, 15),
//             Token::new(")".to_string(), TokenType::DELIMITER(Delimiters::RPAR), 5, 31),
//             Token::new("".to_string(), TokenType::NEWLINE, 5, 32),
//             Token::new("".to_string(), TokenType::DEDENT, 6, 5), // Retour à l'indentation niveau 1
//             Token::new("else".to_string(), TokenType::KEYWORD(Keywords::ELSE), 6, 5),
//             Token::new(":".to_string(), TokenType::DELIMITER(Delimiters::COLON), 6, 9),
//             Token::new("".to_string(), TokenType::NEWLINE, 6, 10),
//             Token::new("".to_string(), TokenType::INDENT, 7, 9), // Indentation niveau 2
//             Token::new("print".to_string(), TokenType::IDENTIFIER { name: "print".to_string() }, 7, 9),
//             Token::new("(".to_string(), TokenType::DELIMITER(Delimiters::LPAR), 7, 14),
//             Token::new("\"Less or equal to 5\"".to_string(), TokenType::STRING { value: "Less or equal to 5".to_string(), kind: StringKind::NORMAL }, 7, 15),
//             Token::new(")".to_string(), TokenType::DELIMITER(Delimiters::RPAR), 7, 35),
//             Token::new("".to_string(), TokenType::NEWLINE, 7, 36),
//             Token::new("".to_string(), TokenType::DEDENT, 8, 5), // Retour à l'indentation niveau 1
//             Token::new("let".to_string(), TokenType::KEYWORD(Keywords::LET), 8, 5),
//             Token::new("y".to_string(), TokenType::IDENTIFIER { name: "y".to_string() }, 8, 9),
//             Token::new("=".to_string(), TokenType::OPERATOR(Operators::EQUAL), 8, 11),
//             Token::new("20".to_string(), TokenType::INTEGER { value: BigInt::from(20) }, 8, 13),
//             Token::new("".to_string(), TokenType::NEWLINE, 8, 15),
//             Token::new("".to_string(), TokenType::DEDENT, 9, 1), // Retour à l'indentation de base (0)
//             Token::new("".to_string(), TokenType::DEDENT, 9, 1), // Fin de toutes les indentations restantes
//             Token::new("".to_string(), TokenType::EOF, 9, 1),
//         ];
//
//         // Comparer les tokens générés avec les tokens attendus
//         assert_eq!(tokens, expected_tokens);
//     }

    // #[test]
    // fn test_tab_indentation() {
    //     let source_code = "def test():\n\tprint('Hello')\n\t\tprint('Indented')\n\tprint('Back')\n";
    //     let mut lexer = Lexer::new(source_code, SyntaxMode::Indentation);
    //     let tokens = lexer.tokenize();
    //
    //     println!("Nombre total de tokens: {}", tokens.len());
    //     for (i, token) in tokens.iter().enumerate() {
    //         println!("Token {}: {:?}", i, token);
    //     }
    //
    //     let indent_count = tokens.iter().filter(|t| matches!(t.token_type, TokenType::INDENT)).count();
    //     let dedent_count = tokens.iter().filter(|t| matches!(t.token_type, TokenType::DEDENT)).count();
    //     let newline_count = tokens.iter().filter(|t| matches!(t.token_type, TokenType::NEWLINE)).count();
    //
    //     println!("Nombre de INDENT: {}", indent_count);
    //     println!("Nombre de DEDENT: {}", dedent_count);
    //     println!("Nombre de NEWLINE: {}", newline_count);
    //
    //     assert_eq!(indent_count, 2, "Nombre incorrect de tokens INDENT");
    //     assert_eq!(dedent_count, 2, "Nombre incorrect de tokens DEDENT");
    //     assert!(newline_count > 0, "Aucun token NEWLINE trouvé");
    //
    //     // Vérifier l'ordre des tokens d'indentation
    //     let indentation_tokens: Vec<_> = tokens.iter()
    //         .filter(|t| matches!(t.token_type, TokenType::INDENT | TokenType::DEDENT))
    //         .collect();
    //
    //     assert_eq!(indentation_tokens.len(), 4, "Nombre total incorrect de tokens d'indentation");
    //     assert!(matches!(indentation_tokens[0].token_type, TokenType::INDENT), "Premier token d'indentation incorrect");
    //     assert!(matches!(indentation_tokens[1].token_type, TokenType::INDENT), "Deuxième token d'indentation incorrect");
    //     assert!(matches!(indentation_tokens[2].token_type, TokenType::DEDENT), "Troisième token d'indentation incorrect");
    //     assert!(matches!(indentation_tokens[3].token_type, TokenType::DEDENT), "Quatrième token d'indentation incorrect");
    // }


//     #[test]
//     fn simplified_lexer_test() {
//         let source_code = r#"
// def test():
//     print("Hello")
//     if True:
//         print("Indented")
// "#;
//
//         let mut lexer = Lexer::new(source_code, SyntaxMode::Indentation);
//         let tokens = lexer.tokenize();
//
//         for token in &tokens {
//             println!("{:?}", token.token_type);
//         }
//
//         // Vérifiez manuellement que les tokens INDENT, DEDENT et NEWLINE sont présents
//         assert!(tokens.iter().any(|t| matches!(t.token_type, TokenType::INDENT)), "Pas de token INDENT trouvé");
//         assert!(tokens.iter().any(|t| matches!(t.token_type, TokenType::DEDENT)), "Pas de token DEDENT trouvé");
//         assert!(tokens.iter().any(|t| matches!(t.token_type, TokenType::NEWLINE)), "Pas de token NEWLINE trouvé");
//     }

//     #[test]
//     fn test_lexer_indentation() {
//         let source_code = r#"
// def example_function():
//     if True:
//         print("Indented")
//     else:
//         print("Also indented")
//         if False:
//             print("More indentation")
//     print("Back to first indentation level")
//
// print("No indentation")
// "#;
//
//         let mut lexer = Lexer::new(source_code, SyntaxMode::Indentation);
//         let tokens = lexer.tokenize();
//
//         // Vérifiez les tokens un par un
//         let expected_tokens = vec![
//             TokenType::NEWLINE,
//             TokenType::KEYWORD(Keywords::DEF),
//             TokenType::IDENTIFIER { name: "example_function".to_string() },
//             TokenType::DELIMITER(Delimiters::LPAR),
//             TokenType::DELIMITER(Delimiters::RPAR),
//             TokenType::DELIMITER(Delimiters::COLON),
//             TokenType::NEWLINE,
//             TokenType::INDENT,
//             TokenType::KEYWORD(Keywords::IF),
//             TokenType::KEYWORD(Keywords::TRUE),
//             TokenType::DELIMITER(Delimiters::COLON),
//             TokenType::NEWLINE,
//             TokenType::INDENT,
//             TokenType::IDENTIFIER { name: "print".to_string() },
//             TokenType::DELIMITER(Delimiters::LPAR),
//             TokenType::STRING { value: "Indented".to_string(), kind: StringKind::NORMAL },
//             TokenType::DELIMITER(Delimiters::RPAR),
//             TokenType::NEWLINE,
//             TokenType::DEDENT,
//             TokenType::KEYWORD(Keywords::ELSE),
//             TokenType::DELIMITER(Delimiters::COLON),
//             TokenType::NEWLINE,
//             TokenType::INDENT,
//             TokenType::IDENTIFIER { name: "print".to_string() },
//             TokenType::DELIMITER(Delimiters::LPAR),
//             TokenType::STRING { value: "Also indented".to_string(), kind: StringKind::NORMAL },
//             TokenType::DELIMITER(Delimiters::RPAR),
//             TokenType::NEWLINE,
//             TokenType::KEYWORD(Keywords::IF),
//             TokenType::KEYWORD(Keywords::FALSE),
//             TokenType::DELIMITER(Delimiters::COLON),
//             TokenType::NEWLINE,
//             TokenType::INDENT,
//             TokenType::IDENTIFIER { name: "print".to_string() },
//             TokenType::DELIMITER(Delimiters::LPAR),
//             TokenType::STRING { value: "More indentation".to_string(), kind: StringKind::NORMAL },
//             TokenType::DELIMITER(Delimiters::RPAR),
//             TokenType::NEWLINE,
//             TokenType::DEDENT,
//             TokenType::DEDENT,
//             TokenType::IDENTIFIER { name: "print".to_string() },
//             TokenType::DELIMITER(Delimiters::LPAR),
//             TokenType::STRING { value: "Back to first indentation level".to_string(), kind: StringKind::NORMAL },
//             TokenType::DELIMITER(Delimiters::RPAR),
//             TokenType::NEWLINE,
//             TokenType::DEDENT,
//             TokenType::NEWLINE,
//             TokenType::IDENTIFIER { name: "print".to_string() },
//             TokenType::DELIMITER(Delimiters::LPAR),
//             TokenType::STRING { value: "No indentation".to_string(), kind: StringKind::NORMAL },
//             TokenType::DELIMITER(Delimiters::RPAR),
//             TokenType::NEWLINE,
//             TokenType::EOF,
//         ];
//
//         assert_eq!(tokens.len(), expected_tokens.len(), "Le nombre de tokens ne correspond pas");
//
//         for (i, (token, expected)) in tokens.iter().zip(expected_tokens.iter()).enumerate() {
//             assert_eq!(token.token_type, *expected, "Le token à l'index {} ne correspond pas. Attendu: {:?}, Obtenu: {:?}", i, expected, token.token_type);
//         }
//     }
// }