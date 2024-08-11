use crate::lexer::token;
use num_bigint::BigInt;
use num_traits::identities::Zero;
use num_traits::Num;
use std::char;
use std::cmp::Ordering;
use std::str::FromStr;
use std::collections::HashMap;
use crate::error::{LexerError, LexerErrorType,Position};
use crate::token::TokenType;

#[derive(Debug, PartialEq, Clone,)]
struct IndentationLevel {
    tabs: usize,
    spaces: usize,
}

impl IndentationLevel {
    fn compare_strict(&self, other:&IndentationLevel) -> Result<Ordering,LexerError>{
        match self.tabs.cmp(&other.tabs) {
            Ordering::Less => {
                if self.spaces <= other.spaces{
                    Ok(Ordering::Less)
                }else {
                    Err(LexerError{
                        error: LexerErrorType::TabError})
                }
            }
            Ordering::Greater => {
                if self.spaces >= other.spaces{
                    Ok(Ordering::Greater)
                } else {
                    Err(LexerError{
                        error: LexerErrorType::TabError})
                }
            }
            Ordering::Equal => Ok(self.spaces.cmp(&other.spaces))
        }
    }
}
//
#[derive(Debug)]
pub struct Token {
    pub text: String,
    pub kind: token::TokenType,
}
impl Token{
    pub fn new(text:String, kind:token::TokenType) -> Self{
        Token{text, kind}
    }
}

pub struct Lexer<Spanned> {
    source: String,
    start_of_line: bool,
    current: char,
    indentation_stack: Vec<IndentationLevel>,
    peending: Vec<Spanned>,
    chr0: Option<char>,
    char1: Option<char>,
    char2: Option<char>,
    keywords: HashMap<String, TokenType>,
    position: Position,
}
