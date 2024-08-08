use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct LexerError {
    pub kind: LexerErrorKind,
    pub message: String,
    pub line: usize,
    pub column: usize,
}


#[derive(Debug)]
pub enum LexerErrorKind {
    InvalidCharacter(char),
    InvalidToken(String),
    UnterminatedString,
    UnterminatedComment,

}




impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "LexerError: {} at line {} column {}", self.message, self.line, self.column)

    }
    // fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //     match self {
    //         LexerError::InvalidCharacter(c) => write!(f, "Invalid character: {}", c),
    //         LexerError::InvalidToken(t) => write!(f, "Invalid token: {}", t),
    //         LexerError::UnterminatedString => write!(f, "Unterminated string"),
    //         LexerError::UnterminatedComment => write!(f, "Unterminated comment"),
    //     }
    // }
}

