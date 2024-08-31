use num_bigint::BigInt;
use crate::lexer::tok::TokenType;
//use crate::parser::parser::LiteralValue;
use crate::tok::Operators;

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub enum ASTNode{
    Programm(Vec<ASTNode>),
    Declaration(Declaration),
    Expression(Expression),
    Statement(Statement),
    Block(Block),
    Function(Function),
    IfStatement(IfStatement),
    ForStatement(ForStatement),
    WhileStatement(WhileStatement),
    ReturnStatement(ReturnStatement),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    Identifier(String),
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub enum Declaration{
    VariableDeclaration{
        mutable: bool,
        name: String,
        variable_type: Option<String>,
        value: Expression,
    },
    FunctionDeclaration{
        name: String,
        parameters: Vec<Parameter>,
        return_type: Option<String>,
        body: Box<ASTNode>, //   on va utiliser BOx pour une allocation dynamique
    },

}







// #[allow(dead_code)]
// #[derive(Debug)]
// pub enum ASTNode{
//     Program(Vec<ASTNode>),
//     FunctionDeclaration{
//         name: String,
//         parameters: Vec<ASTNode>,
//         body: Vec<ASTNode>
//     },
//     VariableDeclaration{
//         name: String,
//         value: Box<ASTNode>
//     },
//     IfStatement{
//         condition: Box<ASTNode>,
//         then_block: Vec<ASTNode>,
//         elif_blocks: Vec<(Box<ASTNode>, Vec<ASTNode>)>,
//         else_block: Option<Box<ASTNode>>,
//     },
//     ElifStatement{
//         condition: Box<ASTNode>,
//         block:Vec<ASTNode>
//     },
//     ElseStatement{
//         block: Vec<ASTNode>
//     },
//     WhiileStatment{
//         condition: Box<ASTNode>,
//         body: Vec<ASTNode>
//     },
//     ForStatement{
//         variable: String,
//         iterable: Box<ASTNode>,
//         body: Vec<ASTNode>
//     },
//     ReturnStatement{
//         value: Option<Box<ASTNode>>
//     },
//     Block(Vec<ASTNode>),
//     BinaryOperation{
//         left: Box<ASTNode>,
//         operators: Operators,
//         right: Box<ASTNode>
//     },
//     Identifier(String),
//     Literal(LiteralValue),
//     //UnaryOperation { operator: , operand: Box<ASTNode> },
// }
//
// #[allow(dead_code)]
// #[derive(Debug)]
// pub enum LiteralValue{
//     Integer{value:BigInt},
//     Float{value:f64},
//     String(String),
//     Boolean(bool),
// }
//
//








//
//
//
//
// enum ASTNode {
//     Program(Vec<Box<ASTNode>>),
//     Statement(Box<ASTNode>),
//     FunctionDef{
//         name: String,
//         params: Vec<String,String>,
//         return_type: Option<String>,
//         body:Box<ASTNode>
//     },
//     StructDef{
//         name: String,
//         fields: Vec<String,String>      // name, type
//     },
//
//     // Statements
//     LetStatement{
//         name: String,
//         mutable: bool,
//         type_annotation: Option<String>,
//         value: Box<ASTNode>
//     },
//     ifStatement{
//         condition: Box<ASTNode>,
//         then_branch: Box<ASTNode>,
//         else_branch: Option<Box<ASTNode>>,
//     },
//     WhileStatement{
//         condition: Box<ASTNode>,
//         body: Box<ASTNode>,
//     },
// }