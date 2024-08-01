use crate::lex::{Lexer, TokenType};

mod lex;

fn main() {
    let source_code = "let x = 5";


    let mut lexer = Lexer::new(source_code);
    loop {
        let token = lexer.get_token();
        println!("{:?}", token.kind);
        println!("Token text : {}", token.text);
        if token.kind == TokenType::EOF {
            break;
        }
    }


    // let tokens = Lexer(source_code);
    // println!("{:?}",tokens);


    println!("Pyrust Compiler ");
    println!("By YmC")
}

