#![allow(dead_code)]
#![allow(unused)]
//use pyrust::parser::parser::Parser;

use pyrust::lexer::lex::{Lexer, Token};
use pyrust::lexer::lex::SyntaxMode;
use pyrust::parser::parser::Parser;
use pyrust::parser::ast::{ASTNode, Declaration, VariableDeclaration, FunctionDeclaration, ConstDeclaration,Expression,Literal};

fn main() {


    println!("Pyrust Compiler Test");
    println!("===================\n");

    let code_source = "let mut x: int = 5;";
    let mut lexer = Lexer::new(code_source, SyntaxMode::Braces);
    let tokens = lexer.tokenize();

    println!("Tokens:");
    for (i, tok) in tokens.iter().enumerate() {
        println!("{}:{:?}", i, tok);
    }
    println!();

    let mut parser = Parser::new(tokens, SyntaxMode::Braces);
    match parser.parse_variable_declaration() {
        Ok(ast) => {
            println!("AST for Variable Declaration:");
            print_ast(&ast, 0);
        }
        Err(e) => println!("Error parsing: {}", e),
    }
}

fn print_ast(node: &ASTNode, indent: usize) {
    let indent_str = "  ".repeat(indent);
    match node {
        ASTNode::Declaration(Declaration::Variable(var)) => {
            println!("{}VariableDeclaration:", indent_str);
            println!("{}  name: {}", indent_str, var.name);
            println!("{}  mutability: {:?}", indent_str, var.mutability);
            if let Some(typ) = &var.variable_type {
                println!("{}  type: {:?}", indent_str, typ);
            }
            if let Some(value) = &var.value {
                println!("{}  value:", indent_str);
                print_expression(value, indent + 2);
            }
        }
        _ => println!("{}Unexpected node type", indent_str),
    }
}

fn print_expression(expr: &Expression, indent: usize) {
    let indent_str = "  ".repeat(indent);
    match expr {
        Expression::Literal(lit) => match lit {
            Literal::Integer { value } => println!("{}Integer: {}", indent_str, value),
            Literal::Float { value } => println!("{}Float: {}", indent_str, value),
            Literal::String(s) => println!("{}String: \"{}\"", indent_str, s),
            Literal::Boolean(b) => println!("{}Boolean: {}", indent_str, b),
            _ => println!("{}Other literal", indent_str),
        },
        Expression::Identifier(name) => println!("{}Identifier: {}", indent_str, name),
        Expression::BinaryOperation(bin_op) => {
            println!("{}BinaryOperation:", indent_str);
            println!("{}  operator: {:?}", indent_str, bin_op.operator);
            println!("{}  left:", indent_str);
            print_expression(&bin_op.left, indent + 2);
            println!("{}  right:", indent_str);
            print_expression(&bin_op.right, indent + 2);
        }
        _ => println!("{}Other expression type", indent_str),
    }
}