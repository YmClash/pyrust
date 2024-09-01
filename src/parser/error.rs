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
    UnexpectedToken(String),
    UnexpectedEOF,
    IndentationError,
    BraceError,
}

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
            ParserErrorType::UnexpectedToken(t) => write!(f, "UnexpectedToken {}", t),
            ParserErrorType::UnexpectedEOF => write!(f, "UnexpectedEOF"),
            ParserErrorType::IndentationError => write!(f, "IndentationError"),
            ParserErrorType::BraceError => write!(f, "BraceError"),
        }
    }
}


//implementation du message d'erreur du parseur
impl ParserError{
    pub fn new(error: ParserErrorType, message: String, position: Position) -> Self{
        ParserError{
            error,
            message,
            position
        }
    }

    pub fn unexpected_token(t:&str ,position: Position) -> Self {
        Self::new(ParserErrorType::UnexpectedToken(t.to_string()), format!("Unexpected token: {}", t), position)
    }
    pub fn unexpected_eof(position: Position) -> Self {
        Self::new(ParserErrorType::UnexpectedEOF, "Unexpected EOF".to_string(), position)
    }
    pub fn indentation_error(position: Position) -> Self {
        Self::new(ParserErrorType::IndentationError, "Indentation Error".to_string(), position)
    }
    pub fn brace_error(position: Position) -> Self {
        Self::new(ParserErrorType::BraceError, "Brace Error".to_string(), position)
    }
}