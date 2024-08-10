//mod lex;

use std::str::Chars;
use std::iter::Peekable;
//use std::process::exit;
use std::collections::HashMap;
use crate::lexer::token::{TokenType, Keywords, Delimiter,Endmarker,Literal,Operator,Float};
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
    //pub error: Option<LexerError>,  //   implementation pour plus tard
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
    buffered_tokens: Vec<Token>, // pour la gestion des token multiple en syntaxe_mode : python
    position: Position
}

/// Implementation de la structure Lexer

impl Error for LexerError {}

impl <'a>Lexer<'a> {
    pub fn new(code_source: &'a str) -> Lexer<'a> {
        let mut lexer = Lexer {
            source: code_source.chars().peekable(),
            current_char: None,
            syntax_mode: SyntaxMode::Indentation,    //mode par defaut
            keywords: Lexer::init_keywords(),
            current_indent: 0,
            buffered_tokens: Vec::new(),
            position: Position{line:1,column:1}
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
        SyntaxMode::Indentation => self.handle_python_indentation(),
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
                Ok(Some(Token::new("{".to_string(), TokenType::DELIMITERS(Delimiter::LCURBRACE))))
            },
            Some('}') => {
                self.next_char();
                Ok(Some(Token::new("}".to_string(), TokenType::DELIMITERS(Delimiter::RCURBRACE))))
            },
            Some(';') => {
                self.next_char();
                Ok(Some(Token::new(";".to_string(), TokenType::DELIMITERS(Delimiter::SEMICOLON))))
            },
            _ => Ok(None), // Aucun token d'indentation spécifique trouvé
        }
    }


    pub fn next_char(&mut self) {
        if let Some(c) = self.current_char{
            if c == '\n' {
                self.position.line +=1;
                self.position.column= 1;
            }else {
                self.position.column +=1;
            }
        }
        self.current_char = self.source.next()
    }
    pub fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }
    pub fn abort() {}

    // /////////////////////////////
    pub fn get_token(&mut self) -> Result<Option<Token>, LexerError> {
        if let Some(token) = self.buffered_tokens.pop() {
            return Ok(Some(token));
        }
        self.skip_whitespace();
        self.skip_comment();

        if let Some(indent_token) = self.handle_indentation()? {
            return Ok(Some(indent_token));
        }
            ////// redondance de code
        // if let Some(token) = self.get_identifier_or_keyword()? {
        //     return Ok(Some(token));
        // }
        //
        // if let Some(token) = self.get_number()? {
        //     return Ok(Some(token));
        // }
        //
        // if let Some(token) = self.get_string()? {
        //     return Ok(Some(token));
        // }

        match self.current_char {
            Some(c) if c.is_ascii_alphabetic() => self.get_identifier_or_keyword(),
            Some(c) if c.is_ascii_digit() => self.get_number(),
            Some('"') => self.get_string(),
            Some(c) => self.get_operator_or_delimiter(c),
            None => Ok(Some(Token::new("".to_string(), TokenType::ENDER(Endmarker::EOF)))),
        }
    }

    fn get_identifier_or_keyword(&mut self) -> Result<Option<Token>, LexerError> {
        let mut identifier = String::new();
        while let Some(c) = self.current_char {
            if c.is_ascii_alphanumeric() || c == '_' {
                identifier.push(c);
                self.next_char();
            } else {
                break;
            }
        }
        let token_type = self.keywords.get(&identifier)
            .cloned()
            .unwrap_or(TokenType::LITERALS(Literal::IDENTIFIER(identifier.clone())));
        Ok(Some(Token::new(identifier, token_type)))
    }

    /// implementation  de  get_number

    fn get_number(&mut self) -> Result<Option<Token>, LexerError> {
        let mut number = String::new();
        let mut is_float = false;
        let mut is_hex = false;

        // Vérifier si c'est un nombre hexadécimal
        if self.current_char == Some('0') && self.peek() == Some(&'x') {
            is_hex = true;
            number.push('0');
            self.next_char();
            number.push('x');
            self.next_char();
        }

        while let Some(c) = self.current_char {
            if c.is_ascii_digit() || (is_hex && c.is_ascii_hexdigit()) || (c == '.' && !is_float) {
                if c == '.' {
                    is_float = true;
                }
                number.push(c);
                self.next_char();
            } else if (c == 'e' || c == 'E') && !is_hex {
                // Notation scientifique
                number.push(c);
                self.next_char();
                if self.current_char == Some('+') || self.current_char == Some('-') {
                    number.push(self.current_char.unwrap());
                    self.next_char();
                }
            } else {
                break;
            }
        }

        let token_type = if is_hex {
            TokenType::LITERALS(Literal::HEXNUMBER(number.clone()))
        } else if is_float {
            TokenType::LITERALS(Literal::FLOAT(Float(number.clone())))
        } else {
            TokenType::LITERALS(Literal::INTEGER(number.parse().unwrap()))
        };

        Ok(Some(Token::new(number, token_type)))
    }


    /// implementation   de  get_String
    fn get_string(&mut self) -> Result<Option<Token>, LexerError> {
        self.next_char(); // Consommer le guillemet ouvrant
        let mut string = String::new();
        while let Some(c) = self.current_char {
            if c == '"' {
                self.next_char();
                return Ok(Some(Token::new(string.clone(), TokenType::LITERALS(Literal::STRING(string)))));
            }
            string.push(c);
            self.next_char();
        }
        Err(LexerError::unterminated_string(self.position.line, self.position.column))
    }

    fn get_operator_or_delimiter(&mut self,c:char) ->Result<Option<Token>,LexerError>{
        let mut op = c.to_string();
        let token_type = match (c, self.peek()) {
            ('=', Some(&'=')) => {
                self.next_char();
                op.push('=');
                TokenType::OPERATOR(Operator::EQEQUAL)
            },
            ('!', Some(&'=')) => {
                self.next_char();
                op.push('=');
                TokenType::OPERATOR(Operator::NOTEQUAL)
            },
            ('<', Some(&'=')) => {
                self.next_char();
                op.push('=');
                TokenType::OPERATOR(Operator::LESSEQUAL)
            },
            ('>', Some(&'=')) => {
                self.next_char();
                op.push('=');
                TokenType::OPERATOR(Operator::GREATEREQUAL)
            },
            // ... autres opérateurs composés ...

            _ => match c {
                '+' => TokenType::OPERATOR(Operator::PLUS),
                '-' => TokenType::OPERATOR(Operator::MINUS),
                '*' => TokenType::OPERATOR(Operator::STAR),
                '/' => TokenType::OPERATOR(Operator::SLASH),
                '%' => TokenType::OPERATOR(Operator::PERCENT),
                '|' => TokenType::OPERATOR(Operator::VBAR),
                '&' => TokenType::OPERATOR(Operator::AMPER),
                '=' => TokenType::OPERATOR(Operator::EQUAL),
                '!' => TokenType::OPERATOR(Operator::EXCLAMATION),
                '<' => TokenType::OPERATOR(Operator::LESS),
                '>' => TokenType::OPERATOR(Operator::GREATER),
                '^' => TokenType::OPERATOR(Operator::CIRCUMFLEX),
                '~' => TokenType::OPERATOR(Operator::TILDE),
                '@' => TokenType::OPERATOR(Operator::AT),
                '(' => TokenType::DELIMITERS(Delimiter::LPAR),
                ')' => TokenType::DELIMITERS(Delimiter::RPAR),
                '[' => TokenType::DELIMITERS(Delimiter::LSQB),
                ']' => TokenType::DELIMITERS(Delimiter::RSQB),
                ':' => TokenType::DELIMITERS(Delimiter::COLON),
                '{' => TokenType::DELIMITERS(Delimiter::LCURBRACE),
                '}' => TokenType::DELIMITERS(Delimiter::RCURBRACE),
                ',' => TokenType::DELIMITERS(Delimiter::COMMA),
                ';' => TokenType::DELIMITERS(Delimiter::SEMICOLON),
                '.' => TokenType::DELIMITERS(Delimiter::DOT),
                _ => return Err(LexerError::invalid_character(c, self.position.line, self.position.column)),
            }
        };
        self.next_char();
        Ok(Some(Token::new(c.to_string(),token_type))
        )
    }
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        loop {
            match self.get_token() {
                Ok(token) => {
                    if let Some(token) = token {
                        if token.kind == TokenType::ENDER(Endmarker::EOF) || token.kind == TokenType::ENDER(Endmarker::ENDMARKER) {
                            break;
                        }
                        tokens.push(token);
                    }
                }
                Err(e) => return Err(e),
            }
        }
        Ok(tokens)
    }
    /// fonction pour sauter les espaces
    pub fn skip_whitespace(&mut self){
        while let Some(c) = self.current_char{
            if !c.is_whitespace() || c == '\n'{
                break;
            }
            self.next_char();
        }

    }
    /// fonction pour sauter les commentaires
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

