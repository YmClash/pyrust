// #[cfg(test)]
// mod tests {
//     use pyrust::lexer::lex::Token;
//     use pyrust::parser::ast::{Declaration, Expression, Literal, Type};
//     use pyrust::parser::parser::Parser;
//     use pyrust::{Lexer, SyntaxMode};
//     use pyrust::tok::{Delimiters, Keywords, Operators, TokenType};
//     use super::*;
//
//     #[test]
//     fn test_function_declaration_indentation() {
//         let code = "\
// def add(a: int, b: int) -> int:
//     return a + b
// ";
//         let mut lexer = Lexer::new(code, SyntaxMode::Indentation);
//         let tokens = lexer.tokenize();
//         let mut parser = Parser::new(tokens, SyntaxMode::Indentation);
//         let result = parser.parse_function_declaration();
//
//         assert!(result.is_ok());
//         let function_decl = result.unwrap();
//
//         if let Declaration::Function(func) = function_decl {
//             assert_eq!(func.name, "add");
//             assert_eq!(func.parameters.len(), 2);
//             assert_eq!(func.parameters[0].0, "a");
//             assert_eq!(func.parameters[1].0, "b");
//             if let Type::Int = func.parameters[0].1 {} else {
//                 panic!("Expected parameter 'a' to be of type 'int'");
//             }
//             if let Type::Int = func.parameters[1].1 {} else {
//                 panic!("Expected parameter 'b' to be of type 'int'");
//             }
//             if let Some(Type::Int) = func.return_type {} else {
//                 panic!("Expected return type to be 'int'");
//             }
//             // Vérifie le corps de la fonction
//             assert_eq!(func.body.statements.len(), 1);
//         } else {
//             panic!("Expected a FunctionDeclaration");
//         }
//     }
//
//
// }