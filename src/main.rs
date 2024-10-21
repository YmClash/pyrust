#![allow(dead_code)]
#![allow(unused)]
//use pyrust::parser::parser::Parser;

//use ymcrust::lexxer;
use pyrust::lexer::lex::{Lexer, Token};
use pyrust::lexer::lex::SyntaxMode;
use pyrust::parser::parser::Parser;
use pyrust::parser::ast::{ASTNode, Declaration, VariableDeclaration, FunctionDeclaration, ConstDeclaration,Expression,Literal};

fn main() {
    println!("Pyrust Compiler Test");
    println!("===================\n");

    let code_source = r#"let x = 5;const v = 100;pub const pi = 3.14;"#;

    let binary_code = "(10 - 4) / 2;";

    let mut lexer = Lexer::new(binary_code, SyntaxMode::Braces);
    let tokens = lexer.tokenize();

    // Affichage des tokens pour vérification
    for (i, tok) in tokens.iter().enumerate() {
        println!("{}:{:?}", i, tok);
    }
    println!("\n");

    let mut parser = Parser::new(tokens, SyntaxMode::Braces);

    // while !parser.is_at_end() {
    //     match parser.parse_declaration() {
    //         Ok(ast) => {
    //             println!("AST généré pour la déclaration :");
    //             println!("{:#?}", ast);
    //         }
    //         Err(e) => {
    //             println!("Erreur lors du parsing : {}", e);
    //             break;
    //         }
    //     }
    // }



    match parser.parse_expression(0) {
        Ok(ast) => {
            println!("AST généré pour l'expression :");
            println!("{:#?}", ast);
        }
        Err(e) => {
            println!("Erreur lors du parsing : {}", e);
        }
    }

    println!("\n");
    println!("===================\n");
    println!("Pyrust Compiler By YmC");
    println!("===================\n");
    println!("\n");

}



/*
  // while !parser.is_at_end() {
    //     match parser.parse_declaration() {
    //         Ok(ast) => {
    //             println!("AST généré pour la déclaration :");
    //             println!("{:#?}", ast);
    //         }
    //         Err(e) => {
    //             println!("Erreur lors du parsing : {}", e);
    //             break;
    //         }
    //     }
    // }
*/