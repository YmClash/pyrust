use std::fmt;
use std::fmt::Formatter;
use std::error::Error;



/// Enumeration des erreurs du compilateur
////////////////////////ENUM COMPILER ERROR POUR UTILISATION ULTERIEURE /////////////////////////
#[derive(Debug)]
pub enum CompilerError {
    Lexer(LexerError),
    // ParserError,
    // SemanticError,
    // CodegenError,
}


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

/// Implementation du trait Display pour LexerError
impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "LexerError: {} at line {} column {}", self.message, self.line, self.column)
    }
}

/// Implementation pour les erreurs et methode de creation
impl LexerError {
    pub fn new(kind: LexerErrorKind, message: String, line: usize, column: usize) -> Self {
        LexerError {
            kind,
            message,
            line,
            column,
        }
    }

    pub fn invalid_character(c: char, line: usize, column: usize) -> Self {
        Self::new(LexerErrorKind::InvalidCharacter(c),
                  format!("Invalid character: {}", c), line,column)
    }

    pub fn invalid_token(t: &str, line: usize, column: usize) -> Self {
        Self::new(LexerErrorKind::InvalidToken(t.to_string()), format!("Invalid token: {}", t), line, column)
    }
    pub fn unterminated_string(line: usize, column: usize) -> Self {
        Self::new(LexerErrorKind::UnterminatedString, "Unterminated string".to_string(), line, column
        )
    }
    pub fn unterminated_comment(line: usize, column: usize) -> Self {
        Self::new(LexerErrorKind::UnterminatedComment, "Unterminated comment".to_string(), line, column)
    }
}


impl From<LexerErrorKind> for LexerError {
    fn from(kind: LexerErrorKind) -> Self {
        let message = match &kind {
            LexerErrorKind::InvalidCharacter(c) => format!("Invalid character: {}", c),
            LexerErrorKind::InvalidToken(t) => format!("Invalid token: {}", t),
            LexerErrorKind::UnterminatedString => "Unterminated string".to_string(),
            LexerErrorKind::UnterminatedComment => "Unterminated comment".to_string(),
        };
        LexerError {
            kind,
            message,
            line: 0,  // Ces valeurs devraient être mises à jour par l'appelant
            column: 0,
        }
    }
}

impl LexerErrorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            LexerErrorKind::InvalidCharacter(_) => "Invalid Character",
            LexerErrorKind::InvalidToken(_) => "Invalid Token",
            LexerErrorKind::UnterminatedString => "Unterminated String",
            LexerErrorKind::UnterminatedComment => "Unterminated Comment",
        }
    }
}

impl Error for LexerError {}