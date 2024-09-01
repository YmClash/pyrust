use num_bigint::BigInt;
use crate::lex::Token;

#[allow(dead_code)]
#[derive(Debug,Clone,)]
pub enum ASTNode{
    Program(Vec<ASTNode>),
    Block(Block),
    Declaration(Declaration),
    Expression(Expression),
    Statement(Statement),
    Function(Function),
    IfStatement(IfStatement),
    ForStatement(ForStatement),
    WhileStatement(WhileStatement),
    ReturnStatement(ReturnStatement),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    Identifier(Identifier),
}
#[allow(dead_code)]
#[derive(Debug,Clone)]
struct Block{
    statements: Vec<ASTNode>,
    // pour le mode syntaxe Indentation
    indent_level: Option<usize>,
    // pour le mode syntaxe Brace
    opening_brace: Option<Token>,
    closing_brace: Option<Token>,
}


#[allow(dead_code)]
#[derive(Debug,Clone)]
pub enum Declaration{
    Variable(VariableDeclaration),
    Function(FunctionDeclaration),
    Constante(ConstanteDeclaration),
    Structure(StructDeclaration),
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct VariableDeclaration{
    pub mutable: bool,
    pub name: String,
    pub variable_type: Option<String>,
    pub value: Option<Expression>,
}
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct FunctionDeclaration{
    pub name:String,
    pub parameter: Vec<Parameters>,
    pub return_type: Option<String>,
    pub body: Vec<Block>
}
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct ConstanteDeclaration{
    pub name: String,
    pub constant_type: Option<String>,
    pub value: Expression,
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct StructDeclaration{
    pub name: String,
    pub fields: Vec<Parameters>,
}


#[allow(dead_code)]
#[derive(Debug,Clone)]
pub enum Expression{
    Literal(Literal),
    Identifier(String),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    FunctioCall(FunctionCall),
    ArrayAccess(ArrayAccess),
    MemberAccess(MemberAccess),

}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub enum Literal{
    Integer{value:BigInt},
    Float{value:f64},
    String(String),
    Boolean(bool),
    Array(Vec<Expression>),
}

//fonction parametre
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Parameters{
    pub name: String,
    parameter_type: Option<String>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BinaryOperation{
    pub left: Box<ASTNode>,
    pub operator: String,////////////////////////// a changer
    pub right: Box<ASTNode>,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct UnaryOperation{
    pub operator: String,
    pub operand: Box<ASTNode>,
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct FunctionCall{
    pub name: String,
    pub arguments: Vec<Expression>
}
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct ArrayAccess{
    pub array: Box<Expression>,
    pub index: Box<Expression>
}
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct MemberAccess{
    pub object: Box<Expression>,
    pub member: String,
}



#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Statement{
    Expression(Expression),
    Return(ReturnStatement),
    If(IfStatement),
    ElseIf(ElifStatement),
    While(WhileStatement),
    For(ForStatement),
    Break,
    Continue,

}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ReturnStatement{
    pub value: Option<Expression>
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct IfStatement{
    pub condition: Expression,
    pub elif_blocks: Vec<(Expression, Block)>,
    pub else_block:Option<Box<Statement>>,
    body: Block,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ElifStatement{
    pub condition: Expression,
    pub body: Block,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WhileStatement{
    pub condition: Expression,
    pub body: Block,
}
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct ForStatement{
    pub variable_iter: String,
    pub iterable: Expression,
    pub body: Block,
}
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Function{
    pub declaration: FunctionDeclaration,
    pub body: Block,
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Identifier{
    pub name: String,
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