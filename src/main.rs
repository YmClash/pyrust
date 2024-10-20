#![allow(dead_code)]
#![allow(unused)]
//use pyrust::parser::parser::Parser;

use ymcrust::lexxer;
use pyrust::lexer::lex::{Lexer, Token};
use pyrust::lexer::lex::SyntaxMode;
use pyrust::parser::parser::Parser;
use pyrust::parser::ast::{ASTNode, Declaration, VariableDeclaration, FunctionDeclaration, ConstDeclaration,Expression,Literal};

fn main() {


    println!("Pyrust Compiler Test");
    println!("===================\n");

    let code_source = r#"let mut x: float = 5.5;"#;
    let code = "const x: float = 5.5;";

    let code_source_2 = r#"let mut x: int = 5;let y: float = 3.14;let mut z: str = "hello";"#;

    let mut lexer = Lexer::new(code_source_2,SyntaxMode::Braces);
    //let mut const_lexer = Lexer::new(code,SyntaxMode::Braces);

    let tokens = lexer.tokenize();
    //let const_tokens = const_lexer.tokenize();

    for (i,tok) in tokens.iter().enumerate(){
        println!("{}:{:?}",i,tok);
    }
    println!("\n");

    // for (i,tok) in const_tokens.iter().enumerate(){
    //     println!("{}:{:?}",i,tok);
    // }
    // println!("\n");


    let mut parser = Parser::new(tokens,SyntaxMode::Braces);
    //let mut const_parser = Parser::new(const_tokens,SyntaxMode::Braces);

    while!parser.is_at_end(){
        match parser.parse_variable_declaration(){
            Ok(ast) => {
                println!("AST for Variable Declaration:");
                println!("{:#?}",ast);
            }
            Err(e) =>{
                println!("Error parsing: {}",e);
                break;
            }


        }
    }

}