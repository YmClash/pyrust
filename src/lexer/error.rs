//#![allow(dead_code)]


use std::fmt::{Display, Formatter};
use std::fmt;
use std::fmt::Write;



/// Enumeration des erreurs du compilateur
// #[derive(Debug, PartialEq,Clone)]
// pub enum CompilerError{
//     Lexer(LexerError),
//     ParserError,
//     SemanticError,
//     CodegenError,
//
// }

#[derive(Debug, PartialEq,Clone)]
pub struct Position {
    line: usize,
    column: usize,
}


#[derive(Debug, PartialEq,Clone)]
pub struct LexerError{
    pub error: LexerErrorType,
    pub message: String,
    position: Position,
}

//
#[derive(Debug, PartialEq,Clone)]
pub enum LexerErrorType{
    InvalidCharacter(char),
    InvalidToken(String),
    InvalidFloat(String),
    InvalidInteger(String),
    InvalidHexadecimal(String),
    UnterminatedString,
    UnterminatedComment,
}

impl Position{
    fn new() -> Self{
        Position{line:1,column:1}
    }
    fn advance(&mut self, ch:char){
        self.column += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        }
    }
    fn mouve_left(&mut self) {
        self.column -= 1;
    }
}

impl Display for Position{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"line {}, column {}",self.line,self.column)
    }
}

impl Display for LexerError{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"LexerError: {} at {}",self.message,self.position)
    }
}

impl Display for LexerErrorType{
    fn fmt(&self,f:&mut Formatter<'_>) -> fmt::Result{
        if let LexerErrorType::InvalidCharacter(c) = self {
            write!(f, "Invalid character: {}", c)
        } else if let LexerErrorType::InvalidToken(t) = self {
            write!(f, "Invalid token: {}", t)
        } else if let LexerErrorType::InvalidFloat(f) = self {
            write!(f, "Invalid float: {}", f)
        } else if let LexerErrorType::InvalidInteger(i) = self {
            write!(f, "Invalid integer: {}", i)
        } else if let LexerErrorType::InvalidHexadecimal(h) = self {
            write!(f, "Invalid hexadecimal: {}", h)
        } else if let LexerErrorType::UnterminatedString = self {
            write!(f, "Unterminated string")
        } else {
            write!(f, "Unterminated comment")
        }
    }
}


impl LexerError{
    pub fn new(error: LexerErrorType, message: String,position: Position) -> Self{
        LexerError{
            error,
            message,
            position,
        }

    }
    pub fn invalid_character(c: char, position: Position) -> Self{
        Self::new(LexerErrorType::InvalidCharacter(c),format!("Invalid character: {}",c),position)
    }
    pub fn invalid_token(t: &str, position: Position) -> Self{
        Self::new(LexerErrorType::InvalidToken(t.to_string()),format!("Invalid token: {}",t),position)
    }
    pub fn invalid_integer(i: &str, position: Position) -> Self{
        Self::new(LexerErrorType::InvalidInteger(i.to_string()),format!("Invalid integer: {}",i),position)
    }
    pub fn invalid_float(f: &str, position: Position) -> Self{
        Self::new(LexerErrorType::InvalidFloat(f.to_string()),format!("Invalid float: {}",f),position)
    }

    pub fn invalid_hexadecimal(h: &str, position: Position) -> Self{
        Self::new(LexerErrorType::InvalidHexadecimal(h.to_string()),format!("Invalid hexadecimal: {}",h),position)
    }
    pub fn unterminated_string(position: Position) -> Self{
        Self::new(LexerErrorType::UnterminatedString,"Unterminated string".to_string(),position)
    }
    pub fn unterminated_comment(position: Position) -> Self{
        Self::new(LexerErrorType::UnterminatedComment,"Unterminated comment".to_string(),position)
    }

}




//use std::fmt;
// use std::fmt::{Display, Formatter};
//
//
// #[derive(Debug,PartialEq)]
// pub  struct LexerError{
//     pub(crate) error: LexerErrorType,
//    // location: Location,
// }
//
// #[derive(Debug,PartialEq)]
// pub enum LexerErrorType {
//     StringError,
//     CommentError,
//     IndentationError,
//     NestingError,
//     TabError,
//     TabAndSpaceError,
//     DefaultArgumentError,
//     PositionalArgumentError,
//     DuplicateArgumentError,
//     InvalidTokenError {token: char},
//     FStringError(FStringErrorType),
//     LineContinuationError,
//     EofError,
//     OtherError(String),
//
//     InvalidNumber,
//     UnterminatedString,
//     InvalidCharacter,
// }
//
// ///Implementation de la structure LexerErrorType
//
// impl Display for LexerErrorType {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         match self {
//             LexerErrorType::StringError => write!(f, "String Error"),
//             LexerErrorType::CommentError => write!(f, "Comment Error"),
//             LexerErrorType::IndentationError => write!(f, "Indentation Error"),
//             LexerErrorType::NestingError => write!(f, "Nesting Error"),
//             LexerErrorType::TabError => write!(f, "Tab Error"),
//             LexerErrorType::TabAndSpaceError => write!(f, "Tab and Space Error"),
//             LexerErrorType::DefaultArgumentError => write!(f, "Default Argument Error"),
//             LexerErrorType::PositionalArgumentError => write!(f, "Positional Argument Error"),
//             LexerErrorType::DuplicateArgumentError => write!(f, "Duplicate Argument Error"),
//             LexerErrorType::InvalidTokenError { token } => write!(f, "Invalid Token Error: {}", token),
//             LexerErrorType::FStringError(err) => write!(f, "FString Error: {:?}", err),
//             LexerErrorType::LineContinuationError => write!(f, "Line Continuation Error"),
//             LexerErrorType::EofError => write!(f, "EOF Error"),
//             LexerErrorType::OtherError(msg) => write!(f, "Other Error: {}", msg),
//             LexerErrorType::InvalidNumber => write!(f, "Invalid Number"),
//             LexerErrorType::UnterminatedString => write!(f, "Unterminated String"),
//             LexerErrorType::InvalidCharacter => write!(f, "Invalid Character"),
//         }
//     }
// }
// #[derive(Debug,PartialEq)]
// pub struct FStringError{
//     pub error: FStringErrorType,
//    // pub location: Location,
// }
//
// #[derive(Debug,PartialEq)]
// pub enum FStringErrorType{
//     UncolosedLbrace,
//     UnopenedRbrace,
//     ExpectedRbrace,
//     InvalidExpression,
//     InvalidConversion,
//     EmptyExpression,
//     MismatchDelimiters,
//     ExpressionNestedTooDeep,
//
// }
//
// impl Display for FStringErrorType {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         match self {
//             FStringErrorType::UncolosedLbrace => write!(f, "Unclosed Left Brace"),
//             FStringErrorType::UnopenedRbrace => write!(f, "Unopened Right Brace"),
//             FStringErrorType::ExpectedRbrace => write!(f, "Expected Right Brace"),
//             FStringError Type::InvalidExpression => write!(f, "Invalid Expression"),
//             FStringErrorType::InvalidConversion => write!(f, "Invalid Conversion"),
//             FStringErrorType::EmptyExpression => write!(f, "Empty Expression"),
//             FStringErrorType::MismatchDelimiters => write!(f, "Mismatch Delimiters"),
//             FStringErrorType::ExpressionNestedTooDeep => write!(f, "Expression Nested Too Deep"),
//
//         }
//     }
// }
//
//
// ////////////////////////////////////////IMPLEMENTATION Position /////////////////////////
//
// #[derive(Debug, PartialEq,Copy,Clone,Eq)]
// pub struct Position {
//     pub(crate) line: usize,
//     pub (crate) column: usize,
// }
//
// impl Display for Position {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "line {}, column {}", self.line, self.column)
//     }
// }
//
// impl Position{
//     pub fn visualize<'a> (&self,line: &'a str,
//     desc: impl Display + 'a) {
//         struct Visualise<'a,D: Display>{
//             loc:Position,
//             line:&'a str,
//             desc:D,
//         }
//         impl <D: Display> Display for Visualise<'_,D>{
//             fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//                 write!(f,"{}\n{}{arrow:>pad$}",self.desc,self.line,pad=self.loc.column,arrow="^",)
//             }
//         }
//         Visualise {
//             loc: *self,
//             line,
//             desc,
//         };
//     }
// }
//
// // implementation de Position {
// impl Position{
//     pub fn new(line:usize,column:usize) -> Self{
//         Position{line,column}
//     }
//     pub fn line(&self) -> usize{
//         self.line
//     }
//     pub fn column(&self) -> usize{
//         self.column
//     }
//     pub fn reset(&mut self){
//         self.line = 1;
//         self.column = 1;
//     }
//     pub fn right(&mut self){
//         self.column += 1;
//     }
//     pub fn left(&mut self){
//         self.column -= 1;
//     }
//     pub fn newline(&mut self){
//         self.line += 1;
//         self.column = 1;
//     }
// }
//


///////////////////////////////////////////////////////////////////////////////////























/////////////////////////////////ERROR LEXER #2////////////////////////////////////////

/*
/// Enumeration des erreurs du compilateur
////////////////////////ENUM COMPILER ERROR POUR UTILISATION ULTERIEURE /////////////////////////
#[derive(Debug)]
pub enum CompilerError {
    Lexer(LexerError),
    // ParserError,
    // SemanticError,
    // CodegenError,
}

// structure  de la position Elle permettra de suivre précisément la position dans le code source.
pub struct Position{
    pub(crate) line: usize,
    pub(crate) column: usize,
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
    InvalidFloat(String),
    InvalidInterger(String),
    UnterminatedString,
    UnterminatedComment,


}

/// Implementation de la structure Position pour actualise ses valeurs
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


////////////////////////////////////////IMPLEMENTATION FOR DISPLAY/////////////////////////

/// implemente display pour POSITION

impl fmt::Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

/// Implementation du trait Display pour LexerError
impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "LexerError: {} at line {} column {}", self.message, self.line, self.column)
    }
}

/// Implementation du  displax pour CompilerError
impl fmt::Display for CompilerError{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::Lexer(err) => write!(f,"LexerError: {}",err),
            // ajoute d'autre  cas    d'erreur du compilateur
        }
    }
}

/////////////////////////////////////////////IMPLEMENTATION DES ERREUR///////////////////////////

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


    ///  méthode with_position à LexerError pour mettre à jour
    /// la position après la création de l'erreur :

    pub fn with_position(mut self, position: &Position) -> Self{
        self.line = position.line;
        self.column = position.column;
        self
    }

    /// Methode pour creer une erreur de caractere invalide
    pub fn invalid_character(c: char, line: usize, column: usize) -> Self {
        Self::new(LexerErrorKind::InvalidCharacter(c),
                  format!("Invalid character: {}", c), line,column)
    }
    /// Methode pour creer une erreur de token invalide
    pub fn invalid_token(t: &str, line: usize, column: usize) -> Self {
        Self::new(LexerErrorKind::InvalidToken(t.to_string().clone()), format!("Invalid token: {}", t), line, column)
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
    /// Methode pour creer une erreur de float invalide
    pub fn invalid_float(value: String,line: usize,column:usize) ->Self{
        Self::new(LexerErrorKind::InvalidToken(value.clone()),format!("Invalid float: {}",value),line,column)
    }
    /// Methode pour creer une erreur de hex invalide
    pub fn invalid_interger(value:String,line:usize,column:usize) ->Self{
        LexerError::new(
            LexerErrorKind::InvalidToken(value.clone()),
            format!("Invalid integer: {}", value),line,column)
    }
}


/// Implementation de Error pour CompilerError
impl Error for CompilerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CompilerError::Lexer(err) => Some(err),
            // Ajouter d'autres cas lorsque vous les implémentez
        }
    }
}


///Conversion de LexerErrorKind en LexerError
impl From<LexerErrorKind> for LexerError {
    fn from(kind: LexerErrorKind) -> Self {
        let message = match &kind {
            LexerErrorKind::InvalidCharacter(c) => format!("Invalid character: {}", c),
            LexerErrorKind::InvalidToken(t) => format!("Invalid token: {}", t),
            LexerErrorKind::InvalidFloat(f) => format!("Invalid float: {}", f),
            LexerErrorKind::InvalidInterger(i) => format!("Invalid integer: {}", i),
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
            LexerErrorKind::InvalidFloat(_) => "Invalid Float",
            LexerErrorKind::InvalidInterger(_) => "Invalid Integer",
            LexerErrorKind::UnterminatedString => "Unterminated String",
            LexerErrorKind::UnterminatedComment => "Unterminated Comment",
        }
    }
    //une méthode pour obtenir un message d'erreur formaté :
    pub fn error_message(&self) -> String{
        match self {
            LexerErrorKind::InvalidCharacter(c) => format!("Invalid character: {}", c),
            LexerErrorKind::InvalidToken(t) => format!("Invalid token: {}", t),
            LexerErrorKind::InvalidFloat(f) => format!("Invalid float: {}", f),
            LexerErrorKind::InvalidInterger(i) => format!("Invalid integer: {}", i),
            LexerErrorKind::UnterminatedString => "Unterminated string".to_string(),
            LexerErrorKind::UnterminatedComment => "Unterminated comment".to_string(),
        }
    }
}

// impl Error for LexerError {}


 */