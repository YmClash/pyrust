use num_bigint::BigInt;
use pyrust::lexer::lex::Lexer;
use pyrust::lexer::tok::{Delimiters, Keywords, Operators, StringKind, TokenType};
use pyrust::lexer_error::{LexerError, LexerErrorType, Position};
use pyrust::parser::ast::{ASTNode, Declaration, Expression, FunctionDeclaration, Statement, VariableDeclaration};




#[cfg(test)]
mod tests {
    use nom::Parser;
    use pyrust::parser::parser_error::ParserError;
    use pyrust::SyntaxMode;
    use super::*;
    use crate::Lexer;  // Assurez-vous que ce chemin est correct

    fn parse_code(code: &str, syntax_mode: SyntaxMode) -> Result<ASTNode, ParserError> {
        let mut lexer = Lexer::new(code, syntax_mode);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens, syntax_mode);
        parser.parse()
    }

    #[test]
    fn test_indentation_simple_function() {
        let code = r#"
fn add(a: int, b: int) -> int:
    return a + b
"#;
        let result = parse_code(code, SyntaxMode::Indentation);
        assert!(result.is_ok());
        // Ajoutez plus d'assertions pour vérifier la structure de l'AST
    }

    #[test]
    fn test_indentation_function_with_multiple_statements() {
        let code = r#"
fn complex(x: int) -> int:
    let y = x * 2
    if y > 10:
        return y - 5
    return y
"#;
        let result = parse_code(code, SyntaxMode::Indentation);
        assert!(result.is_ok());
        // Vérifiez la structure de l'AST, y compris les instructions if et let
    }

    #[test]
    fn test_braces_simple_function() {
        let code = r#"
fn add(a: int, b: int) -> int {
    return a + b;
}
"#;
        let result = parse_code(code, SyntaxMode::Braces);
        assert!(result.is_ok());
        // Vérifiez la structure de l'AST
    }

    #[test]
    fn test_braces_function_with_multiple_statements() {
        let code = r#"
fn complex(x: int) -> int {
    let y = x * 2;
    if (y > 10) {
        return y - 5;
    }
    return y;
}
"#;
        let result = parse_code(code, SyntaxMode::Braces);
        assert!(result.is_ok());
        // Vérifiez la structure de l'AST, y compris les accolades imbriquées
    }

    #[test]
    fn test_indentation_nested_blocks() {
        let code = r#"
fn nested():
    if true:
        if false:
            return 1
        else:
            return 2
    return 3
"#;
        let result = parse_code(code, SyntaxMode::Indentation);
        assert!(result.is_ok());
        // Vérifiez la structure des blocs imbriqués
    }

    #[test]
    fn test_braces_nested_blocks() {
        let code = r#"
fn nested() {
    if (true) {
        if (false) {
            return 1;
        } else {
            return 2;
        }
    }
    return 3;
}
"#;
        let result = parse_code(code, SyntaxMode::Braces);
        assert!(result.is_ok());
        // Vérifiez la structure des blocs imbriqués
    }

    #[test]
    fn test_indentation_error_handling() {
        let code = r#"
fn incorrect():
    let x = 5
  return x  # Incorrect indentation
"#;
        let result = parse_code(code, SyntaxMode::Indentation);
        assert!(result.is_err());
        // Vérifiez que l'erreur est liée à l'indentation
    }

    #[test]
    fn test_braces_error_handling() {
        let code = r#"
fn incorrect() {
    let x = 5;
    return x;
"#;  // Missing closing brace
        let result = parse_code(code, SyntaxMode::Braces);
        assert!(result.is_err());
        // Vérifiez que l'erreur est liée aux accolades manquantes
    }
}