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

pub struct Position{
    line: usize,
    column: usize,
}

///Structure pour les erreurs du lexer
#[derive(Debug)]
pub struct LexerError {
    pub kind: LexerErrorKind,
    pub message: String,
    pub line: usize,
    pub column: usize,
}

/// Enumeration des erreurs du lexer
#[derive(Debug)]
pub enum LexerErrorKind {
    InvalidCharacter(char),
    InvalidToken(String),
    UnterminatedString,
    UnterminatedComment,

}


impl Position{
    fn new() -> Self {
        Position{line:1,column:1}
    }
    fn advance(&mut self , ch:char){
        self.column += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        }
    }
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
    /// Methode pour creer une erreur de caractere invalide
    pub fn invalid_character(c: char, line: usize, column: usize) -> Self {
        Self::new(LexerErrorKind::InvalidCharacter(c),
                  format!("Invalid character: {}", c), line,column)
    }
    /// Methode pour creer une erreur de token invalide
    pub fn invalid_token(t: &str, line: usize, column: usize) -> Self {
        Self::new(LexerErrorKind::InvalidToken(t.to_string()), format!("Invalid token: {}", t), line, column)
    }
    /// Methode pour creer une erreur de string non terminee
    pub fn unterminated_string(line: usize, column: usize) -> Self {
        Self::new(LexerErrorKind::UnterminatedString, "Unterminated string".to_string(), line, column
        )
    }
    /// Methode pour creer une erreur de commentaire non terminee
    pub fn unterminated_comment(line: usize, column: usize) -> Self {
        Self::new(LexerErrorKind::UnterminatedComment, "Unterminated comment".to_string(), line, column)
    }
}


///Conversion de LexerErrorKind en LexerError
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

/// /// Conversion de LexerErrorKind en chaîne de caractères
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