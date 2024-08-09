//mod lex;

use std::str::Chars;
use std::iter::Peekable;
//use std::process::exit;
use std::collections::HashMap;
use crate::lexer::token::{TokenType, Keywords};
use crate::lexer::error::{LexerError,Position};
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
    pub error: Option<LexerError>,
}

/// implementation de la structure Token pour cree un nouveau token
impl Token{
    pub fn new(text:String,kind:TokenType) -> Token {
        Token{text,kind,error}
    }
}

// la structure qui bas nous permettre de recuperer les tokens
pub struct Lexer<'a> {
    source:Peekable<Chars<'a>>,
    current_char: Option<char>,
    syntax_mode: SyntaxMode,
    keywords: HashMap<String,TokenType>,
    current_indent: usize,
    buffered_tokens: Vec<Token>, // pour la gestion des token multiple en syntaxe_mode : python
}

/// Implementation de la structure Lexer

impl Error for LexerError {}

impl <'a>Lexer<'a> {
    pub fn new(code_source: &'a str) -> Lexer<'a> {
        let mut lexer = Lexer {
            source: code_source.chars().peekable(),
            current_char: None,
            syntax_mode: SyntaxMode::Indentation,    //mode par defaut
            keywords: Lexer::init_keyword(),
            current_indent: 0,
            buffered_tokens: Vec::new(),
            // position: Position
        };
        lexer.next_char();
        lexer.detect_syntax();
        lexer
    }

    fn init_keywords() -> HashMap<String, TokenType> {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), TokenType::KEYWORD(Keywords::AND));
        keywords.insert("as".to_string(), TokenType::KEYWORD(Keywords::AS));
        keywords.insert("async".to_string(), TokenType::KEYWORD(Keywords::ASYNC));
        keywords.insert("await".to_string(), TokenType::KEYWORD(Keywords::AWAIT));
        keywords.insert("break".to_string(), TokenType::KEYWORD(Keywords::BREAK));
        keywords.insert("const".to_string(), TokenType::KEYWORD(Keywords::CONST));
        keywords.insert("class".to_string(), TokenType::KEYWORD(Keywords::CLASS));
        keywords.insert("continue".to_string(), TokenType::KEYWORD(Keywords::CONTINUE));
        keywords.insert("def".to_string(), TokenType::KEYWORD(Keywords::DEF));
        keywords.insert("del".to_string(), TokenType::KEYWORD(Keywords::DEL));
        keywords.insert("elif".to_string(), TokenType::KEYWORD(Keywords::ELIF));
        keywords.insert("else".to_string(), TokenType::KEYWORD(Keywords::ELSE));
        keywords.insert("enum".to_string(), TokenType::KEYWORD(Keywords::ENUM));
        keywords.insert("except".to_string(), TokenType::KEYWORD(Keywords::EXCEPT));
        keywords.insert("false".to_string(), TokenType::KEYWORD(Keywords::FALSE));
        keywords.insert("fn".to_string(), TokenType::KEYWORD(Keywords::FN));
        keywords.insert("for".to_string(), TokenType::KEYWORD(Keywords::FOR));
        keywords.insert("from".to_string(), TokenType::KEYWORD(Keywords::FROM));
        keywords.insert("if".to_string(), TokenType::KEYWORD(Keywords::IF));
        keywords.insert("impl".to_string(), TokenType::KEYWORD(Keywords::IMPL));
        keywords.insert("import".to_string(), TokenType::KEYWORD(Keywords::IMPORT));
        keywords.insert("in".to_string(), TokenType::KEYWORD(Keywords::IN));
        keywords.insert("is".to_string(), TokenType::KEYWORD(Keywords::IS));
        keywords.insert("lambda".to_string(), TokenType::KEYWORD(Keywords::LAMBDA));
        keywords.insert("let".to_string(), TokenType::KEYWORD(Keywords::LET));
        keywords.insert("loop".to_string(), TokenType::KEYWORD(Keywords::LOOP));
        keywords.insert("match".to_string(), TokenType::KEYWORD(Keywords::MATCH));
        keywords.insert("mod".to_string(), TokenType::KEYWORD(Keywords::MOD));
        keywords.insert("mut".to_string(), TokenType::KEYWORD(Keywords::MUT));
        keywords.insert("none".to_string(), TokenType::KEYWORD(Keywords::NONE));
        keywords.insert("not".to_string(), TokenType::KEYWORD(Keywords::NOT));
        keywords.insert("or".to_string(), TokenType::KEYWORD(Keywords::OR));
        keywords.insert("pub".to_string(), TokenType::KEYWORD(Keywords::PUB));
        keywords.insert("pass".to_string(), TokenType::KEYWORD(Keywords::PASS));
        keywords.insert("raise".to_string(), TokenType::KEYWORD(Keywords::RAISE));
        keywords.insert("return".to_string(), TokenType::KEYWORD(Keywords::RETURN));
        keywords.insert("self".to_string(), TokenType::KEYWORD(Keywords::SELF));
        keywords.insert("static".to_string(), TokenType::KEYWORD(Keywords::STATIC));
        keywords.insert("struct".to_string(), TokenType::KEYWORD(Keywords::STRUCT));
        keywords.insert("super".to_string(), TokenType::KEYWORD(Keywords::SUPER));
        keywords.insert("true".to_string(), TokenType::KEYWORD(Keywords::TRUE));
        keywords.insert("try".to_string(), TokenType::KEYWORD(Keywords::TRY));
        keywords.insert("type".to_string(), TokenType::KEYWORD(Keywords::TYPE));
        keywords.insert("typeof".to_string(), TokenType::KEYWORD(Keywords::TYPEOF));
        keywords.insert("use".to_string(), TokenType::KEYWORD(Keywords::USE));
        keywords.insert("with".to_string(), TokenType::KEYWORD(Keywords::WITH));
        keywords.insert("while".to_string(), TokenType::KEYWORD(Keywords::WHILE));
        keywords.insert("yield".to_string(), TokenType::KEYWORD(Keywords::YIELD));
        keywords
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

            // Calculer le niveau d'indentation actuel
            while let Some(c) = self.current_char {
                match c {
                    ' ' => indent_level += 1,
                    '\t' => {
                        // En Python, une tabulation est équivalente à 8 espaces
                        indent_level += 8 - (indent_level % 8);

                    }
                    _ => break,
                }

                self.next_char();
            }
            // if indent_level % 8 != 0 {
            //     return Err(LexerError::invalid_indentation(self.line, self.column));
            // }


            let mut tokens = Vec::new();

            // Si le niveau d'indentation augmente, produire un token `INDENT`
            if indent_level > self.current_indent {
                self.current_indent = indent_level;
                tokens.push(Token::new("INDENT".to_string(), TokenType::INDENT));
            }
            // Si le niveau d'indentation diminue, produire un ou plusieurs tokens `DEDENT`
            else if indent_level < self.current_indent {
                while indent_level < self.current_indent {
                    self.current_indent -= 8; // On suppose un DEDENT par bloc de 8 espaces/tabulation
                    tokens.push(Token::new("DEDENT".to_string(), TokenType::DEDENT));
                }
            } else {
                // Si le niveau d'indentation est le même, produire un token `NEWLINE`
                tokens.push(Token::new("NEWLINE".to_string(), TokenType::NEWLINE));
            }

            /////////// On  vas  essayer une bufferisation  des  token
            if !tokens.is_empty() {
                let first_token = tokens.remove(0);
                self.buffered_tokens.extend(tokens); // Bufferiser les autres tokens
                return Ok(Some(first_token));
            }
        }
        Ok(None)
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

    // /////////////////////////////
    // pub fn get_token(&mut self) -> Result<Option<Token>,LexerError>{
    //     if let Some(token) = self.buffered_tokens.pop(){
    //         return Ok(token);
    //     }
    //     self.skip_whitespace();
    //     self.skip_comment();
    //
    //     if let Some(indent_token) = self.handle_indentation()? {
    //         return Ok(indent_token)
    //     }
    //
    //
    // }
    // //////////////////////////////////////////

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

