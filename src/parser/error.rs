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
    error: ParserErrorType,
    message: String,
    position: Position,
}

#[allow(dead_code)]
#[derive(Debug,PartialEq,Clone)]
pub struct ParserErrorType{

}




impl ParserError{
    pub fn new(error: ParserErrorType, message: String, position: Position) -> Self{
        ParserError{
            error,
            message,
            position
        }
    }
}