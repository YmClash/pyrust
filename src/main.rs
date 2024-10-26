#![allow(dead_code)]
#![allow(unused)]
//use pyrust::parser::parser::Parser;

//use ymcrust::lexxer;
use pyrust::lexer::lex::{Lexer, Token};
use pyrust::lexer::lex::SyntaxMode;
use pyrust::parser::parser::Parser;
use pyrust::parser::ast::{ASTNode, Declaration, VariableDeclaration, FunctionDeclaration, ConstDeclaration,Expression,Literal};


fn mode(syntax_mode: SyntaxMode){
    match syntax_mode {
        SyntaxMode::Braces => println!("Braces"),
        SyntaxMode::Indentation => println!("Indentation"),
    }
}

fn main() {
    println!("Pyrust Compiler Test");
    println!("===================\n");

    let code_source = r#"let x = 5;const v = 100;"#;

    let binary_code = "-5 ;";

    let code_decl_braces = "let x = 10;let mut y:int = 3;const numb = 5;pub const x:int = 5;struct Point {x: int,y: int};pub struct Point {height: int,width: int};";
    let code_decl_indentation = "let x = 10\nlet mut y:int = 3\nconst numb = 5\npub const x:int = 5\nstruct Point {x: int,y: int}\npub struct Point {height: int,width: int};";

    let solo_decl = "let x = 10\nlet mut y:int = 3\nconst numb = 5\npub const x:int = 5\nstruct Point {x: int,y: int}}\n";


    let code_struct = "struct Point {pub x: int,pub y: int};pub struct Point {height: int,width: int};";

    let code_enum = "enum Color {pub x:int,y:float, pub z:str};";

    let mut lexer = Lexer::new(code_enum, SyntaxMode::Braces);
    let tokens = lexer.tokenize();

    // Affichage des tokens pour vérification
    for (i, tok) in tokens.iter().enumerate() {
        println!("{}:{:?}", i, tok);
    }
    println!("\n");

    let mut parser = Parser::new(tokens, SyntaxMode::Braces);

    while !parser.is_at_end() {
        match parser.parse_declaration() {
            Ok(ast) => {
                println!("AST généré pour la déclaration :");
                println!("{:#?}", ast);
            }
            Err(e) => {
                println!("Erreur lors du parsing : {}", e);
                break;
            }
        }
    }



    // match parser.parse_expression_statement() {
    //     Ok(ast) => {
    //         println!("AST généré pour l'expression :");
    //         println!("{:#?}", ast);
    //     }
    //     Err(e) => {
    //         println!("Erreur lors du parsing : {}", e);
    //     }
    // }

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