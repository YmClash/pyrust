#[allow(dead_code)]
use std::fmt::{Display, Formatter};
use std::fmt;
use crate::tok::TokenType;

#[allow(dead_code)]
#[derive(Debug,PartialEq,Clone)]
pub struct Position{
    pub index: usize,
}

#[allow(dead_code)]
#[derive(Debug,PartialEq,Clone)]
pub struct ParserError{
    pub error: ParserErrorType,
    pub message: String,
    pub position: Position,
}

#[allow(dead_code)]
#[derive(Debug,PartialEq,Clone)]
pub enum  ParserErrorType{
    UnexpectedToken{ expected: TokenType, found: TokenType },
    UnexpectedEOF,
    IndentationError,
    BraceError,
    InvalidAssignmentTarget,
    ExpectedExpression,
    InvalidVariableDeclaration,
}

// #[allow(dead_code)]
// #[derive(Debug,PartialEq,Clone)]
// struct TokExpected{
//     expected: TokenType,
//     found: TokenType,
// }

// implement de la position
#[allow(dead_code)]
impl Position{
    fn new() -> Self{ Position{index: 0}}
    fn advance(&mut self, ch:char){
        self.index += ch.len_utf8();
    }
    fn move_left(&mut self){
        self.index -= 1;
    }
}

//implement de l'affichage de la position

impl  Display for Position{
    fn fmt(&self, f:&mut Formatter<'_>) -> fmt::Result{
        write!(f, "Position index {}", self.index)
    }
}

//implement de l'affichage de l'erreur

impl Display for ParserError{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"ParserError: {} at {}",self.message,self.position)
    }
}

//implement de l'affichage du type d'erreur du parseur

impl Display for ParserErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParserErrorType::UnexpectedToken{expected,found} => write!(f, "Expected Toke {:?}, but Found: {:?}", expected, found),
            ParserErrorType::UnexpectedEOF => write!(f, "UnexpectedEOF"),
            ParserErrorType::IndentationError => write!(f, "IndentationError"),
            ParserErrorType::BraceError => write!(f, "BraceError"),
            ParserErrorType::InvalidAssignmentTarget => write!(f, "InvalidAssignmentTarget"),
            ParserErrorType::ExpectedExpression => write!(f, "ExpectedExpression"),
            ParserErrorType::InvalidVariableDeclaration => write!(f, "InvalidVariableDeclaration"),
        }
    }
}


//implementation du message d'erreur du parseur
impl ParserError {
    pub fn new(error: ParserErrorType, position: Position) -> Self {
        let message = match &error {
            ParserErrorType::UnexpectedToken { expected, found } =>
                format!("Expected {:?}, but found {:?}", expected, found),
            ParserErrorType::UnexpectedEOF => "Unexpected end of file".to_string(),
            ParserErrorType::IndentationError => "Indentation error".to_string(),
            ParserErrorType::BraceError => "Mismatched braces".to_string(),
            ParserErrorType::InvalidAssignmentTarget => "Invalid assignment target".to_string(),
            ParserErrorType::ExpectedExpression => "Expected expression".to_string(),
            //ParserErrorType::InvalidExpression => "Invalid expression".to_string(),
            ParserErrorType::InvalidVariableDeclaration => "Invalid variable declaration".to_string(),
        };

        ParserError {
            error,
            message,
            position,
        }
    }
}