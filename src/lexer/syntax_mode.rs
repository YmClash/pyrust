use crate::lexer::token::TokenType;
use std::fmt::{Display, Formatter};
use std::str::FromStr;



#[derive(Copy, Clone,)]
pub enum SyntaxMode {
    Braces,
    Indentation,
}

#[derive(Copy, Clone,)]
pub enum Mode{
    Module,
    Interactive,
    Expression,
}

impl Mode{
    pub (crate) fn to_marker(self) -> TokenType{
        match self {
            Mode::Module => TokenType::NEWLINE,
            Mode::Interactive => TokenType::ELLIPSIS,
            Mode::Expression => TokenType::STARTEXPRESSION,
        }
    }
}

//Implement Display for Mode
impl FromStr for Mode{
    type Err =ModeParseError;
    fn from_str(s: &str) -> Result<Self, ModeParseError> {
        match s {
            "exec" | "single" => Ok(Mode::Module),
            "eval" => Ok(Mode::Expression),
            _=> Err(ModeParseError{_priv:()}),
        }
    }
}

// deine  la structure ModeParseError
#[derive(Debug)]
pub struct ModeParseError{
    _priv: (),
}
// Implement Display for ModeParseError
impl Display for ModeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid mode: Supported modes are 'exec', 'eval', and 'single'")
    }

}