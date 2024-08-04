//mod lex;

use std::str::Chars;
use std::iter::Peekable;
use std::process::exit;
use std::collections::HashMap;


//use nom::character::complete::i64;


///  represente les differents type de token
#[derive(Debug,PartialEq,Eq,Clone)]
pub enum TokenType {
    EOF,
    NEWLINE,
    NUMBER,
    IDENT,
    STRING,
    // Keywords.
    AND,
    AS,
    ASSERT,
    ASYNC,
    AWAIT,
    BREAK,
    CLASS,
    CONST,
    CONTINUE,
    DEF, // Keyword for  pour la definition des de methode , il agirait omme une fonction ,cela peux etre un maniere de separe fn et def ou decorateur
    DEL,
    DO,  //  new keyword  maybe using it later
    ELIF,
    ELSE,
    ENUM,
    EXCEPT,
    FINALLY,
    FN,
    FOR,
    FROM,
    IMPORT,
    IF,
    IN,
    IS,
    LAMBDA,
    LET,
    LOOP,
    MATCH,
    MOD,
    MUT,
    NOT,
    OR,
    OPEN,
    PASS, // utile pour Compatibilité multi-style :En permettant à la fois l'utilisation d'accolades (style Rust) et l'indentation significative (style Python)
    PRINT,
    PUB,
    RAISE,
    RANGE, // new keyword
    RETURN,
    SELF, // new keyword
    STATIC, // new keyword
    STRUCT,
    TRY,
    TYPE,
    TYPEOF, // new keyword
    USE, // maybe instead of import we can use USE
    WHILE,
    WITH,
    YIELD,

    // Operators.
    AMPERSAND,              // &  // et
    EQ,                     //  =  // assignement
    PLUS,                   // +  // addition
    PLUSEQ,                 // += // addition assignement
    MINUS,                  // -  // soustraction
    MINUSEQ,                // -= // soustraction assignement
    ARROW,                  // -> // arrow assignement ??
    ASTERISK,               // *    // multiplication
    ASTERISKEQ,               // *= /  multiplication assignement
    SLASH,                  // /  // division
    SLASHEQ,                // /=  // division assignement
    MODULO,                 // %  // modulo
    POINT,                  // .     // point
    COLON,                  // :  // 2 point
    DCOLON,                 // ::  // double colon
    SEMICOLON,              // ;  // point virgule
    COMMA,                  // ,  // virgule
    LPAREN,                 // (        // left parenthesis
    RPAREN,                 // )
    LSQUAREBRACET,        // [
    RSQUAREBRACET,        // ]
    LCURBRACET,            // {
    RCURBRACET,            // }
    ATTENTIONDIESE,                  // #  // diese     !#     detection de #mode
    EQEQ,                   // ==   // equivalence
    NOTEQ,                   // !=   // pas egal
    LT,                     // <   // inferieur
    LTEQ,                   // <=   // inferieur ou égal
    SUIVANT,                // =>  // suivant
    GT,                     // >  // plus grand
    GTEQ,                   // >=  // plus grand ou égal

    //token special
    INDENT, // mode indentation
    DEDENT, // mode dedentation

    //&=, |=, ^=, <<=, >>= pour les opérations bit à bit avec assignation : a faire plus tard


}


/// Mon Lexer se base sur  quelque  fonction de base  pour extraire les token qui sont :

///    - new() : pour initialiser le lexer en  passant le fichier code source source
///                et initialiser le premier caractère current_char
///    - next_char() : pour obtenir le prochain caractère
///    - peek() : pour obtenir le prochain caractère sans le consommer
///    - abort() : pour afficher un message d'erreur
///    - skip_whitespace() : pour ignorer les espaces
///    - skip_comment() : pour ignorer les commentaires
///    - get_token() : Fonction principale pour obtenir le token suivant




/// represente un token avec son texte et son type
#[derive(Debug)]
pub struct Token {
    pub text: String,
    pub kind: TokenType,
}


/// implementation de la structure Token pour cree un nouveau token
impl Token {

    pub fn new(text: String, kind: TokenType) -> Token {
        Token { text, kind }
    }
}

/// la structure du lexer qui va nous permettre de recuperer les tokens
pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    current_char: Option<char>,
    keywords: HashMap<String,TokenType>,
}

/// implementation de la structure Lexer
impl<'a> Lexer<'a> {
    ///methode pour initialiser le lexer a partir du code source
    pub fn new(code_source: &'a str) -> Lexer<'a> {
        let mut lexer = Lexer {
            source: code_source.chars().peekable(),
            current_char: None,
            keywords: Lexer::init_keywords(),
        };
        lexer.next_char(); // Initialiser le premier caractère
        lexer
    }

    ///methode pour initialiser le HASHMAP de  mots clés

    fn init_keywords() -> HashMap<String,TokenType>{
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), TokenType::AND);
        keywords.insert("as".to_string(), TokenType::AS);
        keywords.insert("assert".to_string(), TokenType::ASSERT);
        keywords.insert("async".to_string(), TokenType::ASYNC);
        keywords.insert("await".to_string(), TokenType::AWAIT);
        keywords.insert("break".to_string(), TokenType::BREAK);
        keywords.insert("class".to_string(), TokenType::CLASS);
        keywords.insert("const".to_string(), TokenType::CONST);
        keywords.insert("continue".to_string(), TokenType::CONTINUE);
        keywords.insert("def".to_string(), TokenType::DEF);
        keywords.insert("del".to_string(), TokenType::DEL);
        keywords.insert("do".to_string(), TokenType::DO);
        keywords.insert("elif".to_string(), TokenType::ELIF);
        keywords.insert("else".to_string(), TokenType::ELSE);
        keywords.insert("enum".to_string(), TokenType::ENUM);
        keywords.insert("except".to_string(), TokenType::EXCEPT);
        keywords.insert("finally".to_string(), TokenType::FINALLY);
        keywords.insert("fn".to_string(), TokenType::FN);
        keywords.insert("for".to_string(), TokenType::FOR);
        keywords.insert("from".to_string(), TokenType::FROM);
        keywords.insert("import".to_string(), TokenType::IMPORT);
        keywords.insert("if".to_string(), TokenType::IF);
        keywords.insert("in".to_string(), TokenType::IN);
        keywords.insert("is".to_string(), TokenType::IS);
        keywords.insert("lambda".to_string(), TokenType::LAMBDA);
        keywords.insert("let".to_string(), TokenType::LET);
        keywords.insert("loop".to_string(), TokenType::LOOP);
        keywords.insert("match".to_string(), TokenType::MATCH);
        keywords.insert("mod".to_string(), TokenType::MOD);
        keywords.insert("mut".to_string(), TokenType::MUT);
        keywords.insert("not".to_string(), TokenType::NOT);
        keywords.insert("or".to_string(), TokenType::OR);
        keywords.insert("open".to_string(), TokenType::OPEN);
        keywords.insert("pass".to_string(), TokenType::PASS);
        keywords.insert("print".to_string(), TokenType::PRINT);
        keywords.insert("pub".to_string(), TokenType::PUB);
        keywords.insert("raise".to_string(), TokenType::RAISE);
        keywords.insert("range".to_string(), TokenType::RANGE);
        keywords.insert("return".to_string(), TokenType::RETURN);
        keywords.insert("self".to_string(), TokenType::SELF);
        keywords.insert("static".to_string(), TokenType::STATIC);
        keywords.insert("string".to_string(),TokenType::STRING);
        keywords.insert("struct".to_string(), TokenType::STRUCT);
        keywords.insert("try".to_string(), TokenType::TRY);
        keywords.insert("type".to_string(), TokenType::TYPE);
        keywords.insert("typeof".to_string(), TokenType::TYPEOF);
        keywords.insert("use".to_string(), TokenType::USE);
        keywords.insert("while".to_string(), TokenType::WHILE);
        keywords.insert("with".to_string(), TokenType::WITH);
        keywords.insert("yield".to_string(), TokenType::YIELD);
        return keywords;
    }


    ///methode pour obtenir le prochain caractère
    pub fn next_char(&mut self) {
        self.current_char = self.source.next();
    }


    ///methode pour obtenir ou regarder  prochain caractère sans le consommer
    pub fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    ///methode pour afficher un message d'erreur et quitte le lexer
    pub fn abort(&self, message: &str) {
        eprintln!("Lexing error: {}", message);
        exit(1);
    }

    /// cette methode est notre fonction principale pour obtenir les tokens
    /// methode pour obtenir le token suivant
    pub fn get_token(&mut self) -> Token {
        self.skip_whitespace();
        self.skip_comment();

        let token = match self.current_char {
            Some('&') => Token::new("&".to_string(), TokenType::AMPERSAND),
            Some('+') => {
                if self.peek() == Some(&'=') {
                    self.next_char();
                    Token::new("+=".to_string(), TokenType::PLUSEQ)
                } else {
                    Token::new("+".to_string(), TokenType::PLUS)
                }
            }
            Some('-') => {
                if self.peek() == Some(&'>') {
                    self.next_char();
                    Token::new("->".to_string(), TokenType::ARROW)
                } else if self.peek() == Some(&'=') {
                    self.next_char();
                    Token::new("-=".to_string(), TokenType::MINUSEQ)}
                else {
                    Token::new("-".to_string(), TokenType::MINUS)
                }
            }
            Some('*') => {
                if self.peek() == Some(&'='){
                    self.next_char();
                    Token::new("*=".to_string(), TokenType::ASTERISKEQ)
                } else {
                    Token::new("*".to_string(), TokenType::ASTERISK)
                }
            }
            Some('/') => {
                if self.peek() == Some(&'=') {
                    self.next_char();
                    Token::new("/=".to_string(), TokenType::SLASHEQ)
                } else {
                    Token::new("/".to_string(), TokenType::SLASH)
                }

            }
            Some('%') => Token::new("%".to_string(), TokenType::MODULO),
            Some('.') => Token::new(".".to_string(), TokenType::POINT),
            Some(':') => {
                if self.peek() == Some(&':'){
                    self.next_char();
                    Token::new("::".to_string(), TokenType::DCOLON)
                } else {
                    Token::new(":".to_string(), TokenType::COLON)
                }
            }
            Some(';') => Token::new(";".to_string(), TokenType::SEMICOLON),
            Some(',') => Token::new(",".to_string(), TokenType::COMMA),
            Some('(') => Token::new("(".to_string(), TokenType::LPAREN),
            Some(')') => Token::new(")".to_string(), TokenType::RPAREN),
            Some('[') => Token::new("[".to_string(), TokenType::LSQUAREBRACET),
            Some(']') => Token::new("]".to_string(), TokenType::RSQUAREBRACET),
            Some('{') => Token::new("{".to_string(), TokenType::LCURBRACET),
            Some('}') => Token::new("}".to_string(), TokenType::RCURBRACET),

            Some('=') => {
                if self.peek() == Some(&'=') {
                    self.next_char();
                    Token::new("==".to_string(), TokenType::EQEQ)
                } else if self.peek() == Some(&'>') {
                    self.next_char();
                    Token::new("=>".to_string(), TokenType::SUIVANT)
                }
                else {
                    Token::new("=".to_string(), TokenType::EQ)
                }
            }
            Some('>') => {
                if self.peek() == Some(&'=') {
                    self.next_char();
                    Token::new(">=".to_string(), TokenType::GTEQ)
                } else {
                    Token::new(">".to_string(), TokenType::GT)
                }
            }
            Some('<') => {
                if self.peek() == Some(&'=') {
                    self.next_char();
                    Token::new("<=".to_string(), TokenType::LTEQ)
                } else {
                    Token::new("<".to_string(), TokenType::LT)
                }
            }
            Some('!') => {
                if self.peek() == Some(&'='){
                    self.next_char();
                    Token::new("!".to_string(), TokenType::NOTEQ)
                }else if self.peek() == Some(&'#'){
                    Token::new("!".to_string(), TokenType::ATTENTIONDIESE)}
                else {
                    Token::new("!".to_string(), TokenType::NOT)
                }
            },



            Some('"') => {
                self.next_char();
                let mut string_content = String::new();
                while let Some(&c) = self.peek() {
                    if c == '"' {
                        break;
                    }
                    if c == '\r' || c == '\n' || c == '\t' || c == '\\' || c == '%' {
                        self.abort("Illegal character in string.");
                    }
                    string_content.push(c);
                    self.next_char();
                }
                self.next_char(); // Passer le dernier guillemet
                Token::new(string_content, TokenType::STRING)
            }
            Some(c) if c.is_ascii_digit() => {
                let mut number_content = String::new();
                number_content.push(c);
                while let Some(&c) = self.peek() {
                    if c.is_ascii_digit() {
                        number_content.push(c);
                        self.next_char();
                    } else {
                        break;
                    }
                }
                if self.peek() == Some(&'.') {
                    number_content.push('.');
                    self.next_char();
                    if !self.peek().unwrap_or(&' ').is_ascii_digit() {
                        self.abort("Illegal character in number.");
                    }
                    while let Some(&c ) = self.peek() {
                        if c.is_ascii_digit() {
                            number_content.push(c);
                            self.next_char();
                        } else {
                            break;
                        }
                    }
                }
                Token::new(number_content, TokenType::NUMBER)
            }
            Some(c) if c.is_ascii_alphabetic() => {
                let mut ident_content = String::new();
                ident_content.push(c);
                while let Some(&c) = self.peek() {
                    if c.is_ascii_alphanumeric() {
                        ident_content.push(c);
                        self.next_char();
                    } else {
                        break;
                    }
                }
                let token_type = match ident_content.as_str() {
                    "and" => TokenType::AND,
                    "as" => TokenType::AS,
                    "assert" => TokenType::ASSERT,
                    "async" => TokenType::ASYNC,
                    "await" => TokenType::AWAIT,
                    "break" => TokenType::BREAK,
                    "class" => TokenType::CLASS,
                    "const" => TokenType::CONST,
                    "continue" => TokenType::CONTINUE,
                    "def" => TokenType::DEF,
                    "del" => TokenType::DEL,
                    "do" => TokenType::DO,
                    "elif" => TokenType::ELIF,
                    "else" => TokenType::ELSE,
                    "enum" => TokenType::ENUM,
                    "except" => TokenType::EXCEPT,
                    "finally" => TokenType::FINALLY,
                    "fn" => TokenType::FN,
                    "for" => TokenType::FOR,
                    "from" => TokenType::FROM,
                    "import" => TokenType::IMPORT,
                    "if" => TokenType::IF,
                    "in" => TokenType::IN,
                    "is" => TokenType::IS,
                    "lambda" => TokenType::LAMBDA,
                    "let" => TokenType::LET,
                    "loop" => TokenType::LOOP,
                    "match" => TokenType::MATCH,
                    "mod" => TokenType::MOD,
                    "mut" => TokenType::MUT,
                    "not" => TokenType::NOT,
                    "or" => TokenType::OR,
                    "open" => TokenType::OPEN,
                    "pass" => TokenType::PASS,
                    "print" => TokenType::PRINT,
                    //"priv" => TokenType::PRIV,
                    "pub" => TokenType::PUB,
                    "raise" => TokenType::RAISE,
                    "range" => TokenType::RANGE,
                    "return" => TokenType::RETURN,
                    "self" => TokenType::SELF,
                    "static" => TokenType::STATIC,
                    "struct" => TokenType::STRUCT,
                    "try" => TokenType::TRY,
                    "type" => TokenType::TYPE,
                    "typeof" => TokenType::TYPEOF,
                    "use" => TokenType::USE,
                    "while" => TokenType::WHILE,
                    "with" => TokenType::WITH,
                    "yield" => TokenType::YIELD,
                    _ => TokenType::IDENT,
                };
                Token::new(ident_content, token_type)
            }


            ////
            Some(c) if c.is_ascii_digit() => self.get_identifier_or_keyword(),
            Some(c) if c.is_ascii_alphabetic() => self.get_number(),
            Some('"') => self.get_string(),
            /////

            Some('\n') => Token::new("\n".to_string(), TokenType::NEWLINE),
            None => Token::new("".to_string(), TokenType::EOF),
            Some(c) => {
                self.abort(&format!("Unknown token: {}", c));
                Token::new("".to_string(), TokenType::EOF) // Ne sera jamais atteint
            }
        };
        self.next_char();
        token
    }

    fn get_identifier_or_keyword(&mut self) -> Token{
        let mut iden_content = String::new();
        while let Some(&c) = self.peek(){
            if c.is_ascii_alphanumeric(){
                iden_content.push(c);
                self.next_char();
            } else {
                break;
            }
        }
        let token_type = self.keywords.get(&iden_content).cloned().unwrap_or(TokenType::IDENT);
        Token::new(iden_content,token_type)
    }

    ///methode pour obtenir un nombre
    fn get_number(&mut self) -> Token{
        let mut number_content = String::new();
        while let Some(&c) = self.peek(){
            if c.is_ascii_digit() || c == '.' {
                number_content.push(c);
                self.next_char();
            } else {
                break;
            }
        }
        Token::new(number_content,TokenType::NUMBER)
    }



    ///methode pour obtenir une chaine de caractère
    fn get_string(&mut self) -> Token{
        self.next_char();
        let mut string_content = String::new();
        while let Some(&c) = self.peek(){
            if c == '"' {
                break;
            }
            if c == '\r' || c == '\n' || c == '\t' || c == '\\' || c == '%' {
                self.abort("Illegal character in string.");
            }
            string_content.push(c);
            self.next_char();
        }
        self.next_char(); // Passer le dernier guillemet
        Token::new(string_content, TokenType::STRING)
    }





    ///methode pour ignorer les espaces
    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() && c != '\n' {
                self.next_char();
            } else {
                break;
            }
        }
    }

    /// fonction/methode  pour ignorer les commentaires :
    /// les commentaire commence avec # et se termine avec un retour à la ligne

    pub fn skip_comment(&mut self) {
        if self.current_char == Some('#') {
            while self.current_char != Some('\n') {
                self.next_char();
            }
        }
    }
}





//
//
//
// ///////// 3 eme version du lexer  avec gestion des erreur ///////
// */
//
//
//
// #[derive(Debug)]
// pub struct Token {
//     pub text: String,
//     pub kind: TokenType,
// }
//
//
// impl Token {
//     pub fn new(text: String, kind: TokenType) -> Token {
//         Token { text, kind }
//     }
// }
//
// pub struct Lexer<'a> {
//     source: Peekable<Chars<'a>>,
//     current_char: Option<char>,
// }
//
// // implementation d'un gestion des erreur plus flexible
//
// #[derive(Debug)]
// pub enum LexError {
//     InvalidCharacter(char),
//     InvalidNumber(String),
//     InvalidString(String),
//     UnexpectedEndOfFile,
//
// }
//
//
//
// impl<'a> Lexer<'a> {
//     pub fn new(code_source: &'a str) -> Lexer<'a> {
//         let mut lexer = Lexer {
//             source: code_source.chars().peekable(),
//             current_char: None,
//         };
//         lexer.next_char(); // Initialize the first character
//         lexer
//     }
//
//     // Method to get the next character
//     pub fn next_char(&mut self) {
//         self.current_char = self.source.next();
//     }
//
//     // Method to peek at the next character without consuming it
//     pub fn peek(&mut self) -> Option<&char> {
//         self.source.peek()
//     }
//
//     // Method to handle errors
//     pub fn abort(&self, message: &str, kind: LexError) -> LexError {
//         eprintln!("Lexing error: {} - {:?}", message, kind);
//         kind
//     }
//
//     // Main method to get the next token
//     pub fn get_token(&mut self) -> Result<Token, LexError> {
//         self.skip_whitespace();
//         self.skip_comment();
//
//         let token = match self.current_char {
//             Some('+') => Token::new("+".to_string(), TokenType::PLUS),
//             Some('-') => {
//                 if self.peek() == Some(&'>') {
//                     self.next_char();
//                     Token::new("->".to_string(), TokenType::ARROW)
//                 } else {
//                     Token::new("-".to_string(), TokenType::MINUS)
//                 }
//             }
//             Some('*') => Token::new("*".to_string(), TokenType::ASTERISK),
//             Some('/') => Token::new("/".to_string(), TokenType::SLASH),
//             Some('%') => Token::new("%".to_string(), TokenType::MOD),
//             Some('.') => Token::new(".".to_string(), TokenType::POINT),
//             Some(':') => {
//                 if self.peek() == Some(&':') {
//                     self.next_char();
//                     Token::new("::".to_string(), TokenType::DCOLON)
//                 } else {
//                     Token::new(":".to_string(), TokenType::COLON)
//                 }
//             }
//             Some(';') => Token::new(";".to_string(), TokenType::SEMICOLON),
//             Some(',') => Token::new(",".to_string(), TokenType::COMMA),
//             Some('(') => Token::new("(".to_string(), TokenType::LPAREN),
//             Some(')') => Token::new(")".to_string(), TokenType::RPAREN),
//             Some('[') => Token::new("[".to_string(), TokenType::LSQUAREBRACET),
//             Some(']') => Token::new("]".to_string(), TokenType::RSQUAREBRACET),
//             Some('{') => Token::new("{".to_string(), TokenType::LCURBRACET),
//             Some('}') => Token::new("}".to_string(), TokenType::RCURBRACET),
//             Some('=') => {
//                 if self.peek() == Some(&'=') {
//                     self.next_char();
//                     Token::new("==".to_string(), TokenType::EQEQ)
//                 } else if self.peek() == Some(&'>') {
//                     self.next_char();
//                     Token::new("=>".to_string(), TokenType::SUIVANT)
//                 } else {
//                     Token::new("=".to_string(), TokenType::EQ)
//                 }
//             }
//             Some('>') => {
//                 if self.peek() == Some(&'=') {
//                     self.next_char();
//                     Token::new(">=".to_string(), TokenType::GTEQ)
//                 } else {
//                     Token::new(">".to_string(), TokenType::GT)
//                 }
//             }
//             Some('<') => {
//                 if self.peek() == Some(&'=') {
//                     self.next_char();
//                     Token::new("<=".to_string(), TokenType::LTEQ)
//                 } else {
//                     Token::new("<".to_string(), TokenType::LT)
//                 }
//             }
//             Some('!') => {
//                 if self.peek() == Some(&'=') {
//                     self.next_char();
//                     Token::new("!=".to_string(), TokenType::NOTEQ)
//                 } else {
//                     return Err(self.abort("Expected !=, got !", LexError::InvalidCharacter('!')));
//                 }
//             }
//             Some('\"') => {
//                 self.next_char(); // Skip the opening quote
//                 let mut string_content = String::new();
//                 while let Some(&c) = self.peek() {
//                     if c == '\"' {
//                         break;
//                     }
//                     if c == '\r' || c == '\n' || c == '\t' || c == '\\' || c == '%' {
//                         return Err(self.abort("Illegal character in string.", LexError::InvalidString(string_content.clone())));
//                     }
//                     string_content.push(c);
//                     self.next_char();
//                 }
//                 self.next_char(); // Skip the closing quote
//                 Token::new(string_content, TokenType::STRING)
//             }
//             Some(c) if c.is_ascii_digit() => {
//                 let mut number_content = String::new();
//                 number_content.push(c);
//                 while let Some(&c) = self.peek() {
//                     if c.is_ascii_digit() {
//                         number_content.push(c);
//                         self.next_char();
//                     } else {
//                         break;
//                     }
//                 }
//                 if self.peek() == Some(&'.') {
//                     number_content.push('.');
//                     self.next_char();
//                     if !self.peek().unwrap_or(&' ').is_ascii_digit() {
//                         return Err(self.abort("Illegal character in number.", LexError::InvalidNumber(number_content)));
//                     }
//                     while let Some(&c) = self.peek() {
//                         if c.is_ascii_digit() {
//                             number_content.push(c);
//                             self.next_char();
//                         } else {
//                             break;
//                         }
//                     }
//                 }
//                 Token::new(number_content, TokenType::NUMBER)
//             }
//             Some(c) if c.is_ascii_alphabetic() => {
//                 let mut ident_content = String::new();
//                 ident_content.push(c);
//                 while let Some(&c) = self.peek() {
//                     if c.is_ascii_alphanumeric() {
//                         ident_content.push(c);
//                         self.next_char();
//                     } else {
//                         break;
//                     }
//                 }
//                 let token_type = match ident_content.as_str() {
//                     "and" => TokenType::AND,
//                     "as" => TokenType::AS,
//                     "assert" => TokenType::ASSERT,
//                     "break" => TokenType::BREAK,
//                     "class" => TokenType::CLASS,
//                     "continue" => TokenType::CONTINUE,
//                     "def" => TokenType::DEF,
//                     "del" => TokenType::DEL,
//                     "do" => TokenType::DO,
//                     "elif" => TokenType::ELIF,
//                     "else" => TokenType::ELSE,
//                     "except" => TokenType::EXCEPT,
//                     "finally" => TokenType::FINALLY,
//                     "fn" => TokenType::FN,
//                     "for" => TokenType::FOR,
//                     "from" => TokenType::FROM,
//                     "global" => TokenType::GLOBAL,
//                     "if" => TokenType::IF,
//                     "import" => TokenType::IMPORT,
//                     "in" => TokenType::IN,
//                     "is" => TokenType::IS,
//                     "lambda" => TokenType::LAMBDA,
//                     "let" => TokenType::LET,
//                     "loop" => TokenType::LOOP,
//                     "mut" => TokenType::MUT,
//                     "not" => TokenType::NOT,
//                     "or" => TokenType::OR,
//                     "open" => TokenType::OPEN,
//                     "pass" => TokenType::PASS,
//                     "print" => TokenType::PRINT,
//                     "pub" => TokenType::PUB,
//                     "raise" => TokenType::RAISE,
//                     "return" => TokenType::RETURN,
//                     "struct" => TokenType::STRUCT,
//                     "try" => TokenType::TRY,
//                     "while" => TokenType::WHILE,
//                     "with" => TokenType::WITH,
//                     "yield" => TokenType::YIELD,
//                     _ => TokenType::IDENT,
//                 };
//                 Token::new(ident_content, token_type)
//             }
//             Some('\n') => Token::new("\n".to_string(), TokenType::NEWLINE),
//             Some(c) => return Err(self.abort(&format!("Unknown token: {}", c), LexError::InvalidCharacter(c))),
//             None => Token::new("".to_string(), TokenType::EOF),
//         };
//         self.next_char();
//         Ok(token)
//     }
//
//     // Method to skip whitespace
//     pub fn skip_whitespace(&mut self) {
//         while let Some(c) = self.current_char {
//             if c.is_whitespace() && c != '\n' {
//                 self.next_char();
//             } else {
//                 break;
//             }
//         }
//     }
//
//     // Method to skip comments
//     pub fn skip_comment(&mut self) {
//         if self.current_char == Some('#') {
//             while self.current_char != Some('\n') {
//                 self.next_char();
//             }
//         }
//     }
// }
//



//
// ////////////// PREMIER LEXER //////////
// fn lex(input: &str) -> Vec<Token> {
//     let mut tokens = Vec::new();
//     let chars:Vec<char> = input.chars().collect();
//     let mut i = 0;
//     while i < chars.len(){
//         match chars[i] {
//             '0'..='9' => {
//                 let mut num = chars[i].to_digit(10).unwrap() as i64;
//                 while i+1 < chars.len() && chars[i+1].is_digit(10){
//                     i += 1;
//                     num = num * 10 + chars[i].to_digit(10).unwrap() as i64;
//                 }
//                 tokens.push(Token::Number(num));
//             },
//             '+' => tokens.push(Token::Plus),
//             '-' => tokens.push(Token::Minus),
//             '*' => tokens.push(Token::Multiply),
//             '/' => tokens.push(Token::Divide),
//             '(' => tokens.push(Token::LParen),
//             ')' => tokens.push(Token::RParen),
//             // "Def" => tokens.push(Token::Def),
//             // "Goto" => tokens.push(Token::Goto),
//
//             ' ' | '\n' | '\t' => (),
//
//             _ => panic!("Invalid character: {}",chars[i]),
//         }
//         i += 1;
//
//     }
//     tokens
//
//
// }





















///////// AST
// TODO: Implement the AST
//
// #[derive(Debug)]
// enum ASTNode {
//     Number(i64),
//     Add(Box<ASTNode>,Box<ASTNode>),
//     Subtract(Box<ASTNode>,Box<ASTNode>),
//     Multiply(Box<ASTNode>,Box<ASTNode>),
//     Divide(Box<ASTNode>,Box<ASTNode>),
// }
// fn parse(tokens: &[Token]) -> ASTNode {
//     fn parse_expr(tokens: &[Token], pos: &mut usize) -> ASTNode {
//         let mut left = parse_term(tokens, pos);
//         while *pos < tokens.len() {
//             match tokens[*pos] {
//                 Token::Plus => {
//                     *pos += 1;
//                     left = ASTNode::Add(Box::new(left), Box::new(parse_term(tokens, pos)));
//                 },
//                 Token::Minus => {
//                     *pos += 1;
//                     left = ASTNode::Subtract(Box::new(left), Box::new(parse_term(tokens, pos)));
//                 },
//                 _ => break,
//             }
//         }
//         left
//     }
//
//     fn parse_term(tokens: &[Token], pos: &mut usize) -> ASTNode {
//         let mut left = parse_factor(tokens, pos);
//         while *pos < tokens.len() {
//             match tokens[*pos] {
//                 Token::Multiply => {
//                     *pos += 1;
//                     left = ASTNode::Multiply(Box::new(left), Box::new(parse_factor(tokens, pos)));
//                 },
//                 Token::Divide => {
//                     *pos += 1;
//                     left = ASTNode::Divide(Box::new(left), Box::new(parse_factor(tokens, pos)));
//                 },
//                 _ => break,
//             }
//         }
//         left
//     }
//
//     fn parse_factor(tokens: &[Token], pos: &mut usize) -> ASTNode {
//         match tokens[*pos] {
//             Token::Number(num) => {
//                 *pos += 1;
//                 ASTNode::Number(num)
//             },
//             Token::LParen => {
//                 *pos += 1;
//                 let expr = parse_expr(tokens, pos);
//                 if let Token::RParen = tokens[*pos] {
//                     *pos += 1;
//                     expr
//                 } else {
//                     panic!("Expected closing parenthesis");
//                 }
//             },
//             _ => panic!("Unexpected token: {:?}", tokens[*pos]),
//         }
//     }
//
//     let mut pos = 0;
//     let ast = parse_expr(tokens, &mut pos);
//     if pos != tokens.len() {
//         panic!("Unexpected tokens at the end");
//     }
//     ast
// }