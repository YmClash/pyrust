#![allow(dead_code)]
#![allow(unused)]

//use ymcrust::{type_of};



//mod lex;


use pyrust::lex::{lox, Lexer, SyntaxMode};

fn main() {

    println!("Start Lexer");

    let code = r#"
def example_function():
    if True:
        print("Indented")
    else:
        print("Also indented")
        if False:
            print("More indentation")
    print("Back to first indentation level")

print("No indentation")
"#;



    let mut  nova = Lexer::new(code,SyntaxMode::Indentation);
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