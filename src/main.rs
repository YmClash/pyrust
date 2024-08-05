use crate::lex::{Lexer, TokenType};
//use ymcrust::{type_of};

mod lex;

fn main() {
    let source_code = "let x = 5+9;
                ";

    let mut lexer = Lexer::new(source_code);
   // let token = lexer.get_token().kind;
    loop {
        let token = lexer.get_token();
        println!("{:?}", token.text);
        println!("Token text : {:?}", token.kind);
        println!("Token kind : {:?}", token.kind);
        if token.kind == TokenType::EOF {
            break;
        }
        //println!("TYPE OF TOKEN : {:?}", type_of(&token));

    }
    println!();



    // let tokens = Lexer(source_code);
    // println!("{:?}",tokens);


    println!("Pyrust Compiler ");
    println!("By YmC")
}

