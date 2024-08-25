#![allow(dead_code)]
#![allow(unused)]

//use ymcrust::{type_of};



//mod lex;


use pyrust::lex::{lox, Lexer};

fn main() {

    println!("Start Lexer");

    //let code = "( { [ ] } )";
    let code2 = r#"
            let x = 42.0;
            let y = x + 5.0;
            /* This is a
               multi-line comment */
            let z = y * 2;
            # This is a single-line comment
            if z > 10 {
                print("z is greater than 10");
            } else {
            def foo(x: int, y: int) -> int {
                return x + y;
            }
        "#;


    let mut  nova = Lexer::new(code2);
    let tokens = Lexer::tokenize(&mut nova);
    for token in tokens {
        println!("{:?}", token);
    }
    // while let Some(token) = nova.get_token() {
    //     println!("{:?}", token);
    // }


    println!("Done");
    println!("Pyrust Compiler ");
    println!("By YmC")
}


