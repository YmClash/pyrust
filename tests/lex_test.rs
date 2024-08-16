use pyrust::lexer::lex::Lexer;
use pyrust::lexer::token::{TokenType, Keywords, Literal, Operator, Delimiter, };
use pyrust::lexer::error::LexerError;
use pyrust::lexer::lex::Token;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_basic() {
        let input = r#"
        def my_function(x):
            if x > 10:
                return x + 1
            else:
                return x - 1
        "#;

        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        let expected_tokens = vec![
            Token::new("def".to_string(), TokenType::(Keywords::DEF)),
            Token::new("my_function".to_string(), TokenType::LITERALS(Literal::IDENTIFIER("my_function".to_string()))),
            Token::new("(".to_string(), TokenType::DELIMITERS(Delimiter::LPAR)),
            Token::new("x".to_string(), TokenType::LITERALS(Literal::IDENTIFIER("x".to_string()))),
            Token::new(")".to_string(), TokenType::DELIMITERS(Delimiter::RPAR)),
            Token::new(":".to_string(), TokenType::DELIMITERS(Delimiter::COLON)),
            Token::new("NEWLINE".to_string(), TokenType::NEWLINE),
            Token::new("INDENT".to_string(), TokenType::INDENT),
            Token::new("if".to_string(), TokenType::KEYWORD(Keywords::IF)),
            Token::new("x".to_string(), TokenType::LITERALS(Literal::IDENTIFIER("x".to_string()))),
            Token::new(">".to_string(), TokenType::OPERATOR(Operator::GREATER)),
            Token::new("10".to_string(), TokenType::LITERALS(Literal::INTEGER(10))),
            Token::new(":".to_string(), TokenType::DELIMITERS(Delimiter::COLON)),
            Token::new("NEWLINE".to_string(), TokenType::NEWLINE),
            Token::new("INDENT".to_string(), TokenType::INDENT),
            Token::new("return".to_string(), TokenType::KEYWORD(Keywords::RETURN)),
            Token::new("x".to_string(), TokenType::LITERALS(Literal::IDENTIFIER("x".to_string()))),
            Token::new("+".to_string(), TokenType::OPERATOR(Operator::PLUS)),
            Token::new("1".to_string(), TokenType::LITERALS(Literal::INTEGER(1))),
            Token::new("NEWLINE".to_string(), TokenType::NEWLINE),
            Token::new("DEDENT".to_string(), TokenType::DEDENT),
            Token::new("else".to_string(), TokenType::KEYWORD(Keywords::ELSE)),
            Token::new(":".to_string(), TokenType::DELIMITERS(Delimiter::COLON)),
            Token::new("NEWLINE".to_string(), TokenType::NEWLINE),
            Token::new("INDENT".to_string(), TokenType::INDENT),
            Token::new("return".to_string(), TokenType::KEYWORD(Keywords::RETURN)),
            Token::new("x".to_string(), TokenType::LITERALS(Literal::IDENTIFIER("x".to_string()))),
            Token::new("-".to_string(), TokenType::OPERATOR(Operator::MINUS)),
            Token::new("1".to_string(), TokenType::LITERALS(Literal::INTEGER(1))),
            Token::new("NEWLINE".to_string(), TokenType::NEWLINE),
            Token::new("DEDENT".to_string(), TokenType::DEDENT),
            Token::new("DEDENT".to_string(), TokenType::DEDENT),
        ];

        assert_eq!(tokens.len(), expected_tokens.len());

        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.text, expected_tokens[i].text);
            assert_eq!(format!("{:?}", token.kind), format!("{:?}", expected_tokens[i].kind));
        }
    }
}
