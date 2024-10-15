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

    let code_source = "let mut x:int = 5";
    let mut lexer = Lexer::new(code_source,SyntaxMode::Indentation);
    let tokens = lexer.tokenize();

    for token in tokens {
        println!("{:?}", token);
    }








}