#[cfg(test)]
mod tests {

    use pyrust::parser::ast::{Declaration, Expression, Literal, Type};
    use pyrust::parser::parser::Parser;
    use pyrust::{Lexer, SyntaxMode};
    use pyrust::tok::{Delimiters, Keywords, Operators, TokenType};
    use super::*;


    fn create_parser(source: &str, syntax_mode: SyntaxMode) -> Parser {
        let mut lexer = Lexer::new(source, syntax_mode);
        let tokens = lexer.tokenize();
        Parser::new(tokens, syntax_mode)
    }

    #[test]
    fn test_variable_declaration_indentation() {
        let source = "let x = 5\n";
        let mut parser = create_parser(source, SyntaxMode::Indentation);
        let result = parser.parse_variable_declaration();
        assert!(result.is_ok());
        if let Ok(Declaration::Variable(var_decl)) = result {
            assert_eq!(var_decl.name, "x");
            assert_eq!(var_decl.mutable, false);
            assert!(matches!(var_decl.value, Some(Expression::Literal(Literal::Integer { value })) if value == 5.into()));
        } else {
            panic!("Expected variable declaration");
        }
    }

    #[test]
    fn test_variable_declaration_braces() {
        let source = "let x = 5;";
        let mut parser = create_parser(source, SyntaxMode::Braces);
        let result = parser.parse_variable_declaration();
        assert!(result.is_ok());
        if let Ok(Declaration::Variable(var_decl)) = result {
            assert_eq!(var_decl.name, "x");
            assert_eq!(var_decl.mutable, false);
            assert!(matches!(var_decl.value, Some(Expression::Literal(Literal::Integer { value })) if value == 5.into()));
        } else {
            panic!("Expected variable declaration");
        }
    }

    #[test]
    fn test_function_declaration_indentation() {
        let source = "fn test_func():\n    return 0\n";
        let mut parser = create_parser(source, SyntaxMode::Indentation);
        let result = parser.parse_function_declaration();
        assert!(result.is_ok());
        if let Ok(Declaration::Function(func_decl)) = result {
            assert_eq!(func_decl.name, "test_func");
            assert!(func_decl.parameters.is_empty());
            assert!(func_decl.return_type.is_none());
            assert_eq!(func_decl.body.statements.len(), 1);
        } else {
            panic!("Expected function declaration");
        }
    }

    #[test]
    fn test_function_declaration_braces() {
        let source = "fn test_func() { return 0; }";
        let mut parser = create_parser(source, SyntaxMode::Braces);
        let result = parser.parse_function_declaration();
        assert!(result.is_ok());
        if let Ok(Declaration::Function(func_decl)) = result {
            assert_eq!(func_decl.name, "test_func");
            assert!(func_decl.parameters.is_empty());
            assert!(func_decl.return_type.is_none());
            assert_eq!(func_decl.body.statements.len(), 1);
        } else {
            panic!("Expected function declaration");
        }
    }

    #[test]
    fn test_variable_declaration_with_type_annotation() {
        let source = "let x: int = 5;";
        let mut parser = create_parser(source, SyntaxMode::Braces);
        let result = parser.parse_variable_declaration();
        assert!(result.is_ok());
        if let Ok(Declaration::Variable(var_decl)) = result {
            assert_eq!(var_decl.name, "x");
            assert_eq!(var_decl.mutable, false);
            assert!(matches!(var_decl.variable_type, Some(Type::Int)));
            assert!(matches!(var_decl.value, Some(Expression::Literal(Literal::Integer { value })) if value == 5.into()));
        } else {
            panic!("Expected variable declaration with type annotation");
        }
    }

    #[test]
    fn test_function_declaration_with_parameters_and_return_type() {
        let source = "fn add(a: int, b: int) -> int:\n    return a + b\n";
        let mut parser = create_parser(source, SyntaxMode::Indentation);
        let result = parser.parse_function_declaration();
        assert!(result.is_ok());
        if let Ok(Declaration::Function(func_decl)) = result {
            assert_eq!(func_decl.name, "add");
            assert_eq!(func_decl.parameters.len(), 2);
            assert!(matches!(func_decl.return_type, Some(Type::Int)));
            assert_eq!(func_decl.body.statements.len(), 1);
        } else {
            panic!("Expected function declaration with parameters and return type");
        }
    }

    #[test]
    fn test_variable_declaration_error() {
        let source = "let 5 = x;";
        let mut parser = create_parser(source, SyntaxMode::Braces);
        let result = parser.parse_variable_declaration();
        assert!(result.is_err());
    }

    #[test]
    fn test_function_declaration_error() {
        let source = "fn () { return 0; }";
        let mut parser = create_parser(source, SyntaxMode::Braces);
        let result = parser.parse_function_declaration();
        assert!(result.is_err());
    }


}