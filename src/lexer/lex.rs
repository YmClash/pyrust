//mod lex;

use std::str::Chars;
use std::iter::Peekable;
use std::process::exit;
use std::collections::HashMap;
use std::error::Error;
use crate::lexer::token_type::TokenType;
use crate::lexer::error::LexerError;






/// reprente le mode de syntaxe
/// syntaxe mode like Rust  ou like Python
#[derive(PartialEq)]
pub enum SyntaxMode {
    Braces,
    Indentation
}

/// represente un token avec son texte et son type
#[derive(Debug)]
pub struct Token{
    pub text: String,
    pub kind: TokenType,
}

/// implementation de la structure Token pour cree un nouveau token
impl Token{
    pub fn new(text:String,kind:TokenType) -> Token {
        Token{text,kind}
    }
}

// la structure qui bas nous permettre de recuperer les tokens
pub struct Lexer<'a> {
    source:Peekable<Chars<'a>>,
    current_char: Option<char>,
    syntax_mode: SyntaxMode,
    keywords: HashMap<String,TokenType>,
    current_indent: usize,
}

// Implementation de la structure Lexer

impl Error for LexerError {}

impl <'a>Lexer<'a> {
    pub fn new(code_source:& 'a str) /*-> Lexer<'a>*/ {
        let mut lexer = Lexer{
            source:code_source.chars().peekable(),
            current_char:None,
            syntax_mode: SyntaxMode::Indentation,    //mode par defaut
            keywords:Lexer::init_keyword(),
            current_indent: 0,
        };

    }
    fn detect_syntax(){}
    fn handle_indentation(){}
    fn init_keywords(){}

    pub fn next_char(){}
    pub fn peek(){}
    pub fn abort(){}
    pub fn get_token(){}
    fn get_identifier_or_keyword(){}
    fn get_number(){}
    fn get_string(){}
    pub fn tokenize(){}
    pub fn skip_whitespace(){}
    pub fn skip_comment(){}




}

