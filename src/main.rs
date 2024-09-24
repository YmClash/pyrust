#![allow(dead_code)]
#![allow(unused)]
//use pyrust::parser::parser::Parser;

use pyrust::lexer::lex::{Lexer, Token};
use pyrust::lexer::lex::SyntaxMode;
use pyrust::parser::parser::Parser;
use pyrust::parser::ast::{ASTNode, Declaration, VariableDeclaration, FunctionDeclaration, ConstanteDeclaration};

fn main() {
    println!("Pyrust Compiler Test");
    println!("===================\n");

    let mode = ["Braces","Indentation"];

    let source_code = "class MyClass(BaseClass):
    let x: int = 10
    fn method1(self, y: int) -> void:
        pass";

    println!("Source Code:\n{}\n", source_code);

    let mut lexer = Lexer::new(source_code, SyntaxMode::Indentation);

    let tokens = lexer.tokenize();
    for token in &tokens {
        println!("{:?}", token);
    }

    println!("\n");

    let mut parser = Parser::new(tokens,SyntaxMode::Indentation);
    match parser.parse_class_declaration(){
        Ok(ast) => {
            println!("AST For fonction Declaration:");
            println!("\n");
            println!("AST mode :{} Parsing OK ",mode[1]);
            println!("{:?}", ast);
            println!("\n");
        }
        Err(e) => println!("Error parsing: {} at position {}", e.message, e.position.index),
    }



    // let test_cases = [
    //     ("Simple Function mode Braces", "fn add(a: int, b: float) -> float { return a + b; }",true),
    //     ("simple Funtion mode Indentation parser error if syntax braces", r#"fn add(a: int, b: int) -> int:
    //        return a + b"#,true),
    //
    //     ("Simple Function 2 mode indentation parser error if syntax braces  ", "fn add(a: int, b: float):
    //         let mut x:int = 10 + a
    //         return x + 10 ",true),
    //     ("Function with Multiple Statements mode Braces", r#"fn calculate(x: int, y: int) -> int {
    //             let  result:int  = x * y;
    //             return result + 10;
    //         }
    //     "#,true),
    //     ("Function without Return Type", "fn greet(name: str) { print(\"Hello, \" + name); }",true),
    //     ("Variable Declaration", "let  x:int = 5;",false),
    //     ("Variable Declaration mutable ", "let mut x:float = 5.5;",false),
    // ];
    //
    // for (test_name, source_code,is_function) in test_cases.iter() {
    //     println!("Test Case: {}", test_name);
    //     println!("Source Code:\n{}\n", source_code);
    //
    //     // Lexer Test
    //     println!("Lexer Output:");
    //     match run_lexer(source_code) {
    //         Ok(tokens) => {
    //             for token in &tokens {
    //                 println!("{:?}", token);
    //             }
    //             println!("Lexer completed successfully.\n");
    //
    //             // Parser Test
    //             println!("Parser Output:");
    //             match run_parser(&tokens,*is_function) {
    //                 Ok(ast) => {
    //                     print_ast(ast);
    //                     println!("Parser completed successfully.\n");
    //                 },
    //                 Err(e) => println!("Parser Error: {}\n", e),
    //             }
    //         },
    //         Err(e) => println!("Lexer Error: {}\n", e),
    //     }
    //
    //     println!("----------------------------------------------------\n");
    // }

    println!("Pyrust Compiler Test Completed");
}

fn run_lexer(source_code: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Lexer::new(source_code, SyntaxMode::Indentation);
    Ok(lexer.tokenize())
}

fn run_parser(tokens: &[Token], is_function: bool) -> Result<ASTNode, String> {
    let mut parser = Parser::new(tokens.to_vec(), SyntaxMode::Indentation);
    let result = if is_function {
        parser.parse_function_declaration()
    } else {
        parser.parse_variable_declaration()

    };

    result
        .map(ASTNode::Declaration)
        .map_err(|e| format!("{} at position {}", e.message, e.position.index))
}


fn print_ast(ast: ASTNode) {
    match ast {
        ASTNode::Program(statements) => {
            println!("Program:");
            for statement in statements {
                print_ast(statement);
            }
        },
        ASTNode::Declaration(decl) => print_declaration(decl),
        ASTNode::Expression(expr) => println!("Expression: {:?}", expr),
        ASTNode::Statement(stmt) => println!("Statement: {:?}", stmt),
        ASTNode::Block(block) => println!("Block: {:?}", block),
        _ => println!("Other AST Node: {:?}", ast),
    }
}

fn print_declaration(decl: Declaration) {
    match decl {
        Declaration::Variable(var) => print_variable_declaration(var),
        Declaration::Function(func) => print_function_declaration(func),
        Declaration::Constante(constant) => print_constant_declaration(constant),
        _ => println!("Other Declaration: {:?}", decl),
    }
}

fn print_variable_declaration(var: VariableDeclaration) {
    println!("Variable Declaration:");
    println!("  Name: {}", var.name);
    println!("  Type: {:?}", var.variable_type);
    println!("  Value: {:?}", var.value);
    println!("  Mutable: {}", var.mutable);
}

fn print_function_declaration(func: FunctionDeclaration) {
    println!("Function Declaration:");
    println!("  Name: {}", func.name);
    println!("  Parameters: {:?}", func.parameters);
    println!("  Return Type: {:?}", func.return_type);
    println!("  Body: {:?}", func.body);
}

fn print_constant_declaration(constant: ConstanteDeclaration) {
    println!("Constant Declaration:");
    println!("  Name: {}", constant.name);
    println!("  Type: {:?}", constant.constant_type);
    println!("  Value: {:?}", constant.value);
}










/*

fn main() {

    println!("Start Lexer");

    let source_code = "let x:int = 5";
    let source_code2 = r#"fn add(a: int, b: int) -> int {
        return a + b;}
    "#;

    let mut lexer = Lexer::new(source_code, SyntaxMode::Braces);
    let tokens = lexer.tokenize();
    for token in &tokens {
        println!("{:?}", token);
    }

    //let mut parser = Parser::new(tokens, SyntaxMode::Indentation);
    let mut parser = Parser::new(tokens, SyntaxMode::Braces);
    // match parser.parse_function_declaration(){
    match parser.parse_variable_declaration(){
        Ok(ast) => {
            println!("AST For Function Declaration:");
            print_ast(ast)}
        Err(e) => println!("Error parsing: {} at position {}", e.message, e.position.index),

    }


    println!("Done");
    println!("Pyrust Compiler ");
    println!("By YmC")
}



fn print_ast(ast: ASTNode) {
    match ast {
        ASTNode::Program(statements) => {
            println!("Program:");
            for statement in statements {
                print_ast(statement);
            }
        },
        ASTNode::Declaration(decl) => print_declaration(decl),
        ASTNode::Expression(expr) => println!("Expression: {:?}",expr),

        _ => println!("Unexpected node type"),
    }
}



fn print_declaration(decl: Declaration) {
    match decl {
        Declaration::Variable(var) => print_variable_declaration(var),
        Declaration::Function(func) => print_function_declaration(func),
        _ => println!("Unexpected declaration type"),
    }
}



fn print_variable_declaration(var: VariableDeclaration){
    let variable_keyword = "let".to_string();
    println!("Variable Declaration ");
    println!("Keyword: {}", variable_keyword);
    println!("Name: {}", var.name);
    println!("Type: {:?}", var.variable_type);
    println!("Value: {:?}", var.value);
    println!("Mutable: {}", var.mutable);

}
fn print_function_declaration(func: FunctionDeclaration) {
    let function_keyword = "fn".to_string();
    println!("Function Declaration ");
    println!("Keyword: {}", function_keyword);
    println!("Name: {}", func.name);
    println!("Parameters: {:?}", func.parameters);
    println!("Return Type: {:?}", func.return_type);
    println!("Body: {:?}", func.body);

}
*/





//
// fn print_ast(ast: ASTNode) {
//     match ast {
//         ASTNode::Declaration(decl) => {
//             match decl {
//                 Declaration::Function(func) => print_function_declaration(func),
//                 _ => println!("Not a function declaration."),
//             }
//         }
//         _ => println!("Unexpected AST Node."),
//     }
// }
//
// // Fonction pour afficher les détails d'une fonction dans l'AST
// fn print_function_declaration_2(func: FunctionDeclaration) {
//     println!("Function name: {}", func.name);
//     println!("Parameters:");
//     for (name, param_type) in &func.parameters {
//         println!("  {}: {:?}", name, param_type);
//     }
//     println!("Return type: {:?}", func.return_type);
//     println!("Body: {:?}", func.body); // Affiche les statements dans le corps de la fonction
// }
//
//
//


/*
// Exemple de code source
    // //let syntax_mode = SyntaxMode::Braces; // Ou SyntaxMode::Indentation
    // let mut lexer = Lexer::new(source_code, SyntaxMode::Braces);
    // let tokens = lexer.tokenize();
    // //let mut parser = Parser::new(tokens, syntax_mode);
    //
    // match parser.parse() {
    //     Ok(ast) => println!("{:?}", ast),
    //     Err(e) => eprintln!("Error parsing: {}", e),
    // }*/


