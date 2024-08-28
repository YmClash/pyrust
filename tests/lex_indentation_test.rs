#[cfg(test)]
mod tests {
    use pyrust::lex::{Lexer, SyntaxMode};
    use pyrust::tok::{Delimiters, Keywords, StringKind, TokenType};

    #[test]
    fn test_lexer_indentation() {
        let source_code = r#"
def example_function():
    if True:
        print("Indented")
    else:
        print("Also indented")
        if False:
            print("More indentation")
    print("Back to first indentation level")

print("No indentation")
"#;

        let mut lexer = Lexer::new(source_code, SyntaxMode::Indentation);
        let tokens = lexer.tokenize();

        // Afficher les tokens générés pour le débogage
        for token in &tokens {
            println!("{:?}", token);
        }

        // Définir les tokens attendus
        let expected_tokens = vec![
            TokenType::NEWLINE, // Première ligne vide
            TokenType::KEYWORD(Keywords::DEF),
            TokenType::IDENTIFIER { name: "example_function".to_string() },
            TokenType::DELIMITER(Delimiters::LPAR),
            TokenType::DELIMITER(Delimiters::RPAR),
            TokenType::DELIMITER(Delimiters::COLON),
            TokenType::NEWLINE, // Fin de la définition de fonction
            TokenType::INDENT,  // Indentation de la fonction
            TokenType::KEYWORD(Keywords::IF),
            TokenType::IDENTIFIER { name: "True".to_string() },  // Note : devrait être un Keyword
            TokenType::DELIMITER(Delimiters::COLON),
            TokenType::NEWLINE, // Fin du `if`
            TokenType::INDENT,  // Indentation pour le bloc `if`
            TokenType::IDENTIFIER { name: "print".to_string() },
            TokenType::DELIMITER(Delimiters::LPAR),
            TokenType::STRING { value: "Indented".to_string(), kind: StringKind::NORMAL },
            TokenType::DELIMITER(Delimiters::RPAR),
            TokenType::NEWLINE, // Fin du print dans le `if`
            TokenType::DEDENT,  // Retour à l'indentation précédente après le `if`
            TokenType::KEYWORD(Keywords::ELSE),
            TokenType::DELIMITER(Delimiters::COLON),
            TokenType::NEWLINE, // Fin du `else`
            TokenType::INDENT,  // Indentation pour le bloc `else`
            TokenType::IDENTIFIER { name: "print".to_string() },
            TokenType::DELIMITER(Delimiters::LPAR),
            TokenType::STRING { value: "Also indented".to_string(), kind: StringKind::NORMAL },
            TokenType::DELIMITER(Delimiters::RPAR),
            TokenType::NEWLINE, // Fin du print dans le `else`
            TokenType::KEYWORD(Keywords::IF),
            TokenType::IDENTIFIER { name: "False".to_string() },  // Note : devrait être un Keyword
            TokenType::DELIMITER(Delimiters::COLON),
            TokenType::NEWLINE, // Fin du `if` dans le `else`
            TokenType::INDENT,  // Indentation pour le bloc `if` dans le `else`
            TokenType::IDENTIFIER { name: "print".to_string() },
            TokenType::DELIMITER(Delimiters::LPAR),
            TokenType::STRING { value: "More indentation".to_string(), kind: StringKind::NORMAL },
            TokenType::DELIMITER(Delimiters::RPAR),
            TokenType::NEWLINE, // Fin du print dans le `if` du `else`
            TokenType::DEDENT,  // Retour à l'indentation du `else`
            TokenType::DEDENT,  // Retour à l'indentation de la fonction
            TokenType::IDENTIFIER { name: "print".to_string() },
            TokenType::DELIMITER(Delimiters::LPAR),
            TokenType::STRING { value: "Back to first indentation level".to_string(), kind: StringKind::NORMAL },
            TokenType::DELIMITER(Delimiters::RPAR),
            TokenType::NEWLINE, // Fin du print après le bloc `else`
            TokenType::DEDENT,  // Retour à l'indentation initiale (niveau zéro)
            TokenType::NEWLINE, // Ligne vide
            TokenType::IDENTIFIER { name: "print".to_string() },
            TokenType::DELIMITER(Delimiters::LPAR),
            TokenType::STRING { value: "No indentation".to_string(), kind: StringKind::NORMAL },
            TokenType::DELIMITER(Delimiters::RPAR),
            TokenType::NEWLINE, // Fin du dernier print
            TokenType::EOF,
        ];

        // Vérifier le nombre de tokens générés
        assert_eq!(tokens.len(), expected_tokens.len(), "Le nombre de tokens ne correspond pas");

        // Comparer chaque token généré avec le token attendu
        for (i, (token, expected)) in tokens.iter().zip(expected_tokens.iter()).enumerate() {
            assert_eq!(token.token_type, *expected, "Le token à l'index {} ne correspond pas. Attendu: {:?}, Obtenu: {:?}", i, expected, token.token_type);
        }
    }
}
