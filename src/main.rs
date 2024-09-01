#![allow(dead_code)]
#![allow(unused)]
//use pyrust::parser::parser::Parser;

use pyrust::lexer::lex::Lexer;
use pyrust::lexer::lex::SyntaxMode;

fn main() {

    println!("Start Lexer");

    let source_code = "fn example() { ... }"; // Exemple de code source
    // //let syntax_mode = SyntaxMode::Braces; // Ou SyntaxMode::Indentation
    // let mut lexer = Lexer::new(source_code, SyntaxMode::Braces);
    // let tokens = lexer.tokenize();
    // //let mut parser = Parser::new(tokens, syntax_mode);
    //
    // match parser.parse() {
    //     Ok(ast) => println!("{:?}", ast),
    //     Err(e) => eprintln!("Error parsing: {}", e),
    // }




    let mut  nova = Lexer::new(source_code,SyntaxMode::Indentation);
    let tokens = Lexer::tokenize(&mut nova);
    for token in tokens {
        println!("{:?}", token);
    }

    println!("Done");
    println!("Pyrust Compiler ");
    println!("By YmC")
}