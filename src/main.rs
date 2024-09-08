#![allow(dead_code)]
#![allow(unused)]
//use pyrust::parser::parser::Parser;

use pyrust::lexer::lex::Lexer;
use pyrust::lexer::lex::SyntaxMode;
use pyrust::parser::parser::Parser;
use pyrust::parser::ast::{ASTNode,Declaration,VariableDeclaration};

fn main() {

    println!("Start Lexer");

    let source_code = "let x = 10 ";

    let mut lexer = Lexer::new(source_code, SyntaxMode::Indentation);
    let tokens = lexer.tokenize();
    for token in &tokens {
        println!("{:?}", token);
    }

    //let mut parser = Parser::new(tokens, SyntaxMode::Indentation);
    let mut parser = Parser::new(tokens, SyntaxMode::Indentation);

    match parser.parse_variable_declaration(){
        Ok(ast) => print_ast(ast),
        Err(e) => println!("Error parsing: {}", e),
        // Some(ast) => println!("AST: {:?}", ast),
        // None => println!("Error parsing: "),
    }


    println!("Done");
    println!("Pyrust Compiler ");
    println!("By YmC")
}



fn print_ast(ast: ASTNode) {
    match ast {
        ASTNode::Program(statements) => {
            println!("Program:");
            for statement in statements {
                print_ast(statement);
            }
        },
        ASTNode::Declaration(decl) => print_declaration(decl),
        _ => println!("Unexpected node type"),
    }
}


fn print_declaration(decl: Declaration) {
    match decl {
        Declaration::Variable(var) => print_variable_declaration(var),
        _ => println!("Unexpected declaration type"),
    }
}

fn print_variable_declaration(var: VariableDeclaration){
    println!("Variable Declaration");
    println!("Name: {}", var.name);
    println!("Type: {:?}", var.variable_type);
    println!("Value: {:?}", var.value);
    println!("Mutable: {}", var.mutable);

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