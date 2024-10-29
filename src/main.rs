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
    println!("Mode de syntaxe :Indentation\n");

    let code_source = r#"let x = 5;const v = 100;"#;

    let binary_code = "-5 ;";

    let code_decl_braces = "let x = 10;let mut y:int = 3;const numb = 5;pub const x:int = 5;pub struct Point {x: int,y: int};pub struct Point {height: int,width: int};enum Color {x:int,y:float,z:str};pub enum Color {pub x:int,y:float,z:str};pub fn add(x: int, y: int) -> int {return x + y};";
    let code_decl_indentation = "let x = 10\nlet mut y:int = 3\nconst numb = 5\npub const x:int = 5\nstruct Point {x: int,y: int}\npub struct Point {height: int,width: int}\nenum Color {x:int,y:float,z:str}\npub enum Color {pub x:int,y:float,z:str}\n";

    let solo_decl = "let x = 10\nlet mut y:int = 3\nconst numb = 5\npub const x:int = 5\nstruct Point {x: int,y: int}}\n";


    let code_struct = "struct Point {pub x: int,pub y: int};";

    let code_struct_indent = "pub struct Point {x: int,y: int}\nstruct Point {height: int,width: int}";

    //\npub struct Point {height: int,width: int}


    let code_enum_brace = "pub enum Color {pub x:int,y:float, z:str};";
    let code_enum_indent = "enum Color {x:int,y:float,z:str}\n";


    let code_func_braces = "pub fn add(x: int, y: int) -> int {return x + y};";

    let code_func_indent =
        r#"pub fn add(x: int, y: int) -> int:
        return x + y"#;

    let code_func_brace2 = r#"fn add(x: int, y: int) -> int {
    return x + y;
}"#;

    let code_func_brace3 = "fn add() ->int{return 5};";






    let mut lexer = Lexer::new(code_func_indent, SyntaxMode::Indentation);
    let tokens = lexer.tokenize();

    // Affichage des tokens pour vérification
    for (i, tok) in tokens.iter().enumerate() {
        println!("{}:{:?}", i, tok);
    }
    println!("\n");

    let mut parser = Parser::new(tokens, SyntaxMode::Indentation);

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
    println!("=========OK==========\n");
    println!("Pyrust Compiler By YmC");
    println!("===================\n");
    println!("\n");


}


/*
Braces mode
fn add(x: int, y: int) -> int {
    return x + y;
}

pub fn hello(name: str) {
    print(name);
}
*/


/*
Indentation mode
fn add(x: int, y: int) -> int
    return x + y

pub fn hello(name: str)
    print(name)
*/