//use ymcrust::{type_of};



//mod lex;

use pyrust::lex::{Lexer, Token};
use pyrust::lexer::token::TokenType;

fn main() {

    println!("Start Lexer");

    let code = r#"
    def add(a, b):
        return a + b

    x = 5
    y = 10
    result = add(x, y)
    "#;

    let mut lexer = Lexer::new(code);
    match lexer.tokenize() {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(e) => {
            eprintln!("Lexer error: {:?}", e);
        }
    }


    println!("Pyrust Compiler ");
    println!("By YmC")
}

