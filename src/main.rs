#![allow(dead_code)]
#![allow(unused)]
//use pyrust::parser::parser::Parser;

use pyrust::lexer::lex::{Lexer, Token};
use pyrust::lexer::lex::SyntaxMode;
use pyrust::parser::parser::Parser;
use pyrust::parser::ast::{ASTNode, Declaration, VariableDeclaration, FunctionDeclaration, ConstDeclaration};

fn main() {


    //////////////////////************/////////
    println!("Pyrust Compiler Test");
    println!("===================\n");

    let code_source = "let mut x = 5";
    let mut lexer = Lexer::new(code_source,SyntaxMode::Braces);
    let token = lexer.tokenize();
    for (i,tok) in token.iter().enumerate(){
        println!("{}:{:?}",i,tok);
    }
    println!();

    let mut parser = Parser::new(token, SyntaxMode::Braces);
    match parser.parse_variable_declaration(){
        Ok(ast) => {
            println!("AST for  Variable delaration");
            println!("{:?}",ast);

        }
        Err(e) => println!("Error parsing: {}", e),
    }








}