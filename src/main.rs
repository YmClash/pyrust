#![allow(dead_code)]
#![allow(unused)]

//use ymcrust::{type_of};



//mod lex;


use pyrust::lex::{lox, Lexer, SyntaxMode};

fn main() {

    println!("Start Lexer");

    let code = r#"
def fonction():
    print("Hello")
    if true:
        print("Indented")
    print("Back")
"#;



    let mut  nova = Lexer::new(code,SyntaxMode::Braces);
    let tokens = Lexer::tokenize(&mut nova);
    for token in tokens {
        println!("{:?}", token);
    }

    println!("Done");
    println!("Pyrust Compiler ");
    println!("By YmC")
}
//
//
//
//     let code2 = r#"
// def example():
//     let x = 10
//     if x > 5:
//         print("Greater than 5")
//     else:
//         print("Less or equal to 5")
//     let y = 20
// "#;