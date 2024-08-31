#[allow(dead_code)]
use std::fmt::{Display, Formatter};
use std::fmt;


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
    UnexpectedToken,
    UnexpectedEOF,

}



//implementation des  message d'ereur du parseur
impl ParserError{
    pub fn new(error: ParserErrorType, message: String, position: Position) -> Self{
        ParserError{
            error,
            message,
            position
        }
    }

    pub fn unexpected_token(t:&str ,position: Position) -> Self {
        Self::new(ParserErrorType::UnexpectedToken, format!("Unexpected token: {}", t), position)
    }
    pub fn unexpected_eof(position: Position) -> Self {
        Self::new(ParserErrorType::UnexpectedEOF, "Unexpected EOF".to_string(), position)
    }
}