#[allow(dead_code)]
use std::fmt::{Display, Formatter};
use std::fmt;
use crate::tok::TokenType;

#[allow(dead_code)]
#[derive(Debug,PartialEq,Clone)]
pub struct Position{
    pub line: usize,
    pub column: usize,
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
    fn new() -> Self{ Position{line: 1, column: 1}}
    fn advance(&mut self, ch:char){
        self.column +=1;
        if ch == '\n'{
            self.line +=1;
            self.column = 1;
        }
    }
    fn move_left(&mut self){
        self.column -=1;
    }
}

//implement de l'affichage de la position

impl  Display for Position{
    fn fmt(&self, f:&mut Formatter<'_>) -> fmt::Result{
        write!(f, "line: {}, column: {}", self.line, self.column)
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
            //ParserErrorType::InvalidExpression => "Invalid expression".to_string(),
        };

        ParserError {
            error,
            message,
            position,
        }
    }
}