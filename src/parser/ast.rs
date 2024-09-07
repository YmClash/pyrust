use std::fmt;
use std::fmt::Formatter;
use num_bigint::BigInt;
use crate::lexer::lex::Token;
use crate::parser::parser_error::ParserError;
use crate::SyntaxMode;

#[allow(dead_code)]
#[derive(Debug,Clone,)]
pub enum ASTNode{
    Program(Vec<ASTNode>),
    Block(Block),
    Declaration(Declaration),
    Expression(Expression),
    Statement(Statement),
    Function(Function),
    Error(ParserError),
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Block{
    pub statements: Vec<Statement>,
    pub syntax_mode: SyntaxMode,
    pub indent_level: Option<usize>,  // Pour le mode Indentation
    pub braces: Option<(Token, Token)>,  // Pour le mode Braces (ouverture, fermeture)
    // pub opening_brace: Option<Token>,  // pour le mode syntaxe Brace
    // pub closing_brace: Option<Token>,
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct ParseError {
    pub message: String,
    pub position: Position,
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Position{
    pub line: usize,
    pub column: usize,
}
#[allow(dead_code)]
#[derive(Debug,PartialEq,Clone)]
pub enum Operator{
    Addition,
    Substraction,
    Multiplication,
    Division,
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    And,
    Or,
    LesshanOrEqual,
    GreaterThanOrEqual,
}
#[derive(Debug,Clone)]
pub enum UnaryOperator{
    Negate,
    Not,
    Increment,
    Decrement,
    Reference,
    Dereference,
    BitwiseNot,
    LogicalNot,
    Positive,
    Negative,


}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub enum Declaration{
    Variable(VariableDeclaration),
    Function(FunctionDeclaration),
    Constante(ConstanteDeclaration),
    Structure(StructDeclaration),
    Class(ClassDeclaration),
    Enum(EnumDeclaration),
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct VariableDeclaration{
    pub name: String,
    pub variable_type: Option<String>,
    pub value: Option<Expression>,
    pub mutable: bool,
}
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct FunctionDeclaration{
    pub name:String,
    pub parameter: Vec<Parameters>,
    pub return_type: Option<String>,
    pub block: Block,
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
pub struct ClassDeclaration{
    pub name: String,
    pub fields: Vec<Parameters>,
    pub methods: Vec<FunctionDeclaration>,
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct EnumDeclaration{
    pub name: String,
    pub variants: Vec<String>,
}


#[allow(dead_code)]
#[derive(Debug,Clone)]
pub enum Expression{
    Literal(Literal),
    Identifier(String),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    FunctionCall(FunctionCall),
    ArrayAccess(ArrayAccess),
    MemberAccess(MemberAccess),
    LambdaExpression(LambdaExpression),
    MatchExpression(MatchExpression),
    MatchArms(Box<MatchArms>),


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
    pub parameter_type: Option<String>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BinaryOperation{
    pub left: Box<Expression>,
    pub operator: Operator,                 ///////////////////// a changer
    pub right: Box<Expression>,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct UnaryOperation{
    pub operator: UnaryOperator,
    pub operand: Box<Expression>,
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
    Use(UseStatement),
    Import(ImportStatement),
    Raise(RaiseStatement),
    Del(DelStatement),
    If(IfStatement),
    ElseIf(ElifStatement),
    While(WhileStatement),
    For(ForStatement),
    Break,
    Continue,
    Try(TryStatement),
    With(WithStatement),
    Yield(YieldStatement),


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
    pub block: Block,
    pub elif_blocks: Vec<(Expression, Block)>,
    pub else_block:Option<Block>,

}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ElifStatement{
    pub condition: Expression,
    pub block: Block
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WhileStatement{
    pub condition: Expression,
    pub block: Block
}
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct ForStatement{
    pub variable_iter: String,
    pub iterable: Expression,
    pub block: Block
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct UseStatement{
    pub module: String,
    pub alias: Option<String>,
}
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct ImportStatement{
    pub module: String,
    pub alias: Option<String>,
}
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct RaiseStatement{
    pub exception: Expression,
}
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct DelStatement{
    pub target: Expression,
}
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct TryStatement{
    pub block: Block,
    pub except: Vec<(Option<String>, Block)>,
    pub else_block: Option<Block>,
    pub finally_block: Option<Block>,
}
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct WithStatement{
    pub target: Expression,
    pub block: Block,
}
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct YieldStatement{
    pub value: Option<Expression>,
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Function{
    pub declaration: FunctionDeclaration,
    pub block: Block,
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Identifier{
    pub name: String,
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct LambdaExpression{
    pub parameters: Vec<Parameters>,
    pub block: Block
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct MatchExpression{
    pub expression: Box<Expression>,
    pub arms: Vec<MatchArms>
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct MatchArms{
    pub pattern: Pattern,
    pub expression: Box<Expression>
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub enum Pattern{
    Literal(Literal),
    Identifier(String),
    Wildcard,
}



impl fmt::Display for ASTNode{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ASTNode::Program(statements) => {
                for statement in statements{
                    write!(f, "{}", statement)?;
                }
                Ok(())
            }
            ASTNode::Declaration(decl) => write!(f, "{:?}", decl),
            ASTNode::Expression(expr) => write!(f, "{:?}", expr),
            ASTNode::Statement(stmt) => write!(f, "{:?}", stmt),
            ASTNode::Function(func) => write!(f, "{:?}", func),
            ASTNode::Block(block) => write!(f, "{:?}", block),
            // ASTNode::IfStatement(ifstmt) => write!(f, "{}", ifstmt),
            // ASTNode::ForStatement(forstmt) => write!(f, "{}", forstmt),
            // ASTNode::WhileStatement(whilestmt) => write!(f, "{}", whilestmt),
            // ASTNode::ReturnStatement(retstmt) => write!(f, "{}", retstmt),
            // ASTNode::BinaryOperation(binop) => write!(f, "{}", binop),
            // ASTNode::UnaryOperation(unop) => write!(f, "{}", unop),
            // ASTNode::Identifier(ident) => write!(f, "{}", ident),
            // ASTNode::Literal(lit) => write!(f, "{}", lit),
            // ASTNode::Operator(op) => write!(f, "{}", op),
            ASTNode::Error(err) => write!(f, "{}", err),

        }
    }
}


// by YmC

////////////////////////////////////////////////////////////////

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