//mod lex;

use std::str::Chars;
use std::iter::Peekable;
use std::process::exit;
use std::collections::HashMap;
use crate::lexer::token::{TokenType, Keywords};
use crate::lexer::error::LexerError;
use std::error::Error;





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
    pub fn new(code_source: &'a str) -> Lexer<'a> {
        let mut lexer = Lexer {
            source: code_source.chars().peekable(),
            current_char: None,
            syntax_mode: SyntaxMode::Indentation,    //mode par defaut
            keywords: Lexer::init_keyword(),
            current_indent: 0,
        };
        lexer.next_char();
        lexer.detect_syntax();
        lexer
    }

    fn init_keywords() {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), Keywords::AND);
        keywords.insert("as".to_string(), Keywords::AS);
        keywords.insert("async".to_string(), Keywords::ASYNC);
        keywords.insert("await".to_string(), Keywords::AWAIT);
        keywords.insert("break".to_string(), Keywords::BREAK);
        keywords.insert("const".to_string(), Keywords::CONST);
        keywords.insert("class".to_string(), Keywords::CLASS);
        keywords.insert("continue".to_string(), Keywords::CONTINUE);
        keywords.insert("def".to_string(), Keywords::DEF);
        keywords.insert("del".to_string(), Keywords::DEL);
        keywords.insert("elif".to_string(), Keywords::ELIF);
        keywords.insert("else".to_string(), Keywords::ELSE);
        keywords.insert("enum".to_string(), Keywords::ENUM);
        keywords.insert("except".to_string(), Keywords::EXCEPT);
        keywords.insert("false".to_string(), Keywords::FALSE);
        keywords.insert("fn".to_string(), Keywords::FN);
        keywords.insert("for".to_string(), Keywords::FOR);
        keywords.insert("from".to_string(), Keywords::FROM);
        keywords.insert("if".to_string(), Keywords::IF);
        keywords.insert("impl".to_string(), Keywords::IMPL);
        keywords.insert("import".to_string(), Keywords::IMPORT);
        keywords.insert("in".to_string(), Keywords::IN);
        keywords.insert("is".to_string(), Keywords::IS);
        keywords.insert("lambda".to_string(), Keywords::LAMBDA);
        keywords.insert("let".to_string(), Keywords::LET);
        keywords.insert("loop".to_string(), Keywords::LOOP);
        keywords.insert("match".to_string(), Keywords::MATCH);
        keywords.insert("mod".to_string(), Keywords::MOD);
        keywords.insert("mut".to_string(), Keywords::MUT);
        keywords.insert("none".to_string(), Keywords::NONE);
        keywords.insert("not".to_string(), Keywords::NOT);
        keywords.insert("or".to_string(), Keywords::OR);
        keywords.insert("pub".to_string(), Keywords::PUB);
        keywords.insert("pass".to_string(), Keywords::PASS);
        keywords.insert("raise".to_string(), Keywords::RAISE);
        keywords.insert("return".to_string(), Keywords::RETURN);
        keywords.insert("self".to_string(), Keywords::SELF);
        keywords.insert("static".to_string(), Keywords::STATIC);
        keywords.insert("struct".to_string(), Keywords::STRUCT);
        keywords.insert("super".to_string(), Keywords::SUPER);
        keywords.insert("true".to_string(), Keywords::TRUE);
        keywords.insert("try".to_string(), Keywords::TRY);
        keywords.insert("type".to_string(), Keywords::TYPE);
        keywords.insert("typeof".to_string(), Keywords::TYPEOF);
        keywords.insert("use".to_string(), Keywords::USE);
        keywords.insert("with".to_string(), Keywords::WITH);
        keywords.insert("while".to_string(), Keywords::WHILE);
        keywords.insert("yield".to_string(), Keywords::YIELD);
    }
    fn detect_syntax(&mut self) {
        let mut peek_iter = self.source.clone();
        while let Some(c) = peek_iter.next() {
            if !c.is_whitespace() {
                if c == '{' {
                    self.syntax_mode = SyntaxMode::Braces;
                }
                break;
            }
        }
    }


    pub fn handle_indentation(&mut self) -> Result<Option<Token>, LexerError> {
        match self.syntax_mode{
        SyntaxMode::Indentation => self.handdle_python_indentation(),
        SyntaxMode::Braces => self.handle_rust_indentation(),
        }
    }
    // fonction pour gerer l'indentation en mode python
    fn handle_python_indentation(&mut self) -> Result<Option<Token>, LexerError> {
        if self.current_char == Some('\n') {
            self.next_char(); // Consommer le '\n'
            let mut indent_level = 0;
            while let Some(c) = self.current_char {
                match c {
                    ' ' => indent_level += 1,
                    '\t' => indent_level += 4, // On considère qu'une tabulation vaut 4 espaces
                    _ => break,
                }
                self.next_char();
            }

            if indent_level > self.current_indent {
                self.current_indent = indent_level;
                Ok(Some(Token::new("INDENT".to_string(), TokenType::INDENT)))
            } else if indent_level < self.current_indent {
                self.current_indent = indent_level;
                Ok(Some(Token::new("DEDENT".to_string(), TokenType::DEDENT)))
            } else {
                Ok(Some(Token::new("NEWLINE".to_string(), TokenType::NEWLINE)))
            }
        } else {
            Ok(None)
        }
    }

    fn handle_rust_indentation(&mut self) -> Result<Option<Token>, LexerError> {
        match self.current_char {
            Some('{') => {
                self.next_char();
                Ok(Some(Token::new("{".to_string(), TokenType::LCURBRACE)))
            },
            Some('}') => {
                self.next_char();
                Ok(Some(Token::new("}".to_string(), TokenType::RCURBRACE)))
            },
            Some(';') => {
                self.next_char();
                Ok(Some(Token::new(";".to_string(), TokenType::SEMICOLON)))
            },
            _ => Ok(None), // Aucun token d'indentation spécifique trouvé
        }
    }






    pub fn next_char(&mut self) {
        self.current_char = self.source.next()
    }
    pub fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }
    pub fn abort() {}
    pub fn get_token(&mut self) {}
    fn get_identifier_or_keyword() {}
    fn get_number() {}
    fn get_string() {}
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        loop {
            match self.get_token() {
                Ok(token) => {
                    if token.kind == TokenType::EOF {
                        break;
                    }
                    tokens.push(token);
                },
                Err(e) => return Err(e),
            }
        }
        Ok(tokens)
    }


    pub fn skip_whitespace(&mut self){
        while let Some(c) = self.current_char{
            if !c.is_whitespace() || c == '\n'{
                break;
            }
            self.next_char();
        }

    }
    pub fn skip_comment(&mut self) {
        match self.current_char {
            Some('#') => {
                // Commentaire style Python
                while self.current_char != Some('\n') && self.current_char.is_some() {
                    self.next_char();
                }
            },
            Some('/') if self.peek() == Some(&'/') => {
                // Commentaire style Rust
                self.next_char(); // Consommer le deuxième '/'
                while self.current_char != Some('\n') && self.current_char.is_some() {
                    self.next_char();
                }
            },
            _ => {} // Pas un commentaire, ne rien faire
        }
    }

}

