#![allow(dead_code)]
#![allow(unused)]
//use pyrust::parser::parser::Parser;

use pyrust::lexer::lex::Lexer;
use pyrust::lexer::lex::SyntaxMode;
use pyrust::parser::parser::Parser;
use pyrust::parser::ast::{ASTNode,Declaration,VariableDeclaration,FunctionDeclaration};

fn main() {

    println!("Start Lexer");

    let source_code = "let mut x:float = 10.5";
    let source_code2 = r#"fn add(a:int, b:int) {
 return a + b;
    }
    "#;

    let mut lexer = Lexer::new(source_code2, SyntaxMode::Braces);
    let tokens = lexer.tokenize();
    for token in &tokens {
        println!("{:?}", token);
    }

    //let mut parser = Parser::new(tokens, SyntaxMode::Indentation);
    let mut parser = Parser::new(tokens, SyntaxMode::Braces);
    // match parser.parse_function_declaration(){
    match parser.parse_function_declaration(){
        Ok(ast) => {
            println!("AST For Function Declaration:");
            print_ast(ast)}
        Err(e) => println!("Error parsing: {} at position {}", e.message, e.position.index),

    }


    println!("Done");
    println!("Pyrust Compiler ");
    println!("By YmC")
}



// fn print_ast(ast: ASTNode) {
//     match ast {
//         ASTNode::Program(statements) => {
//             println!("Program:");
//             for statement in statements {
//                 print_ast(statement);
//             }
//         },
//         ASTNode::Declaration(decl) => print_declaration(decl),
//         ASTNode::Expression(expr) => println!("Expression: {:?}",expr),
//
//         _ => println!("Unexpected node type"),
//     }
// }

// fn print_ast(ast: ASTNode) {
//     match ast {
//         ASTNode::Declaration(decl) => {
//             match decl {
//                 Declaration::Function(func) => print_function_declaration(func),
//                 _ => println!("Not a function declaration."),
//             }
//         }
//         _ => println!("Unexpected AST Node."),
//     }
// }
//
// fn print_declaration(decl: Declaration) {
//     match decl {
//         Declaration::Variable(var) => print_variable_declaration(var),
//         Declaration::Function(func) => print_function_declaration(func),
//         _ => println!("Unexpected declaration type"),
//     }
// }
//
//
//
// fn print_variable_declaration(var: VariableDeclaration){
//     let variable_keyword = "let".to_string();
//     println!("Variable Declaration ");
//     println!("Keyword: {}", variable_keyword);
//     println!("Name: {}", var.name);
//     println!("Type: {:?}", var.variable_type);
//     println!("Value: {:?}", var.value);
//     println!("Mutable: {}", var.mutable);
//
// }
// fn print_function_declaration(func: FunctionDeclaration) {
//     let function_keyword = "fn".to_string();
//     println!("Function Declaration ");
//     println!("Keyword: {}", function_keyword);
//     println!("Name: {}", func.name);
//     println!("Parameters: {:?}", func.parameters);
//     println!("Return Type: {:?}", func.return_type);
//     println!("Body: {:?}", func.body);
//
// }


fn print_ast(ast: ASTNode) {
    match ast {
        ASTNode::Declaration(decl) => {
            match decl {
                Declaration::Function(func) => print_function_declaration(func),
                _ => println!("Not a function declaration."),
            }
        }
        _ => println!("Unexpected AST Node."),
    }
}

// Fonction pour afficher les détails d'une fonction dans l'AST
fn print_function_declaration(func: FunctionDeclaration) {
    println!("Function name: {}", func.name);
    println!("Parameters:");
    for (name, param_type) in &func.parameters {
        println!("  {}: {:?}", name, param_type);
    }
    println!("Return type: {:?}", func.return_type);
    println!("Body: {:?}", func.body); // Affiche les statements dans le corps de la fonction
}





/*
// Exemple de code source
    // //let syntax_mode = SyntaxMode::Braces; // Ou SyntaxMode::Indentation
    // let mut lexer = Lexer::new(source_code, SyntaxMode::Braces);
    // let tokens = lexer.tokenize();
    // //let mut parser = Parser::new(tokens, syntax_mode);
    //
    // match parser.parse() {
    //     Ok(ast) => println!("{:?}", ast),
    //     Err(e) => eprintln!("Error parsing: {}", e),
    // }*/