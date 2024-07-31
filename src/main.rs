use std::str::Chars;
use std::iter::Peekable;
use std::process::exit;

//use nom::character::complete::i64;



fn main() {
    let source_code = "LET x = (5 * 5)";


    let mut lexer = Lexer::new(source_code);
    loop {
        let token = lexer.get_token();
        println!("{:?}", token.kind);
        println!("Token text : {}", token.text);
        if token.kind == TokenType::EOF {
            break;
        }
    }


    // let tokens = Lexer(source_code);
    // println!("{:?}",tokens);


    println!("Pyrust Compiler ");
    println!("By YmC")
}


#[derive(Debug,PartialEq,Eq)]

enum TokenType {
    EOF,
    NEWLINE,
    NUMBER,
    IDENT,
    STRING,
    // Keywords.
    LABEL,
    GOTO,
    PRINT,
    INPUT,
    LET,
    IF,
    THEN,
    ENDIF,
    WHILE,
    REPEAT,
    ENDWHILE,
    // Operators.
    EQ,
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    LPAREN,
    RPAREN,
    EQEQ,
    NOTEQ,
    LT,
    LTEQ,
    GT,
    GTEQ,

// other keywords for after
    // EOF,
    // NEWLINE,
    //
    // //operators
    // Number(i64),
    // Plus,
    // Minus,
    // Multiply,
    // Divide,
    // LParen,
    // RParen,
    // //keywords
    // Def,
    // Goto,
    // Input,
    // Print,
    // Let,
    // Mut,
    // For,
    // If,
    // Elif,
    // Else,
    // While,
    // Loop,
    // Break,

}

#[derive(Debug)]
struct Token {
    text: String,
    kind: TokenType,
}


impl Token {
    fn new(text: String, kind: TokenType) -> Token {
        Token { text, kind }
    }
}

struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    fn new(code_source: &'a str) -> Lexer<'a> {
        let mut lexer = Lexer {
            source: code_source.chars().peekable(),
            current_char: None,
        };
        lexer.next_char(); // Initialiser le premier caractère
        lexer
    }

    //methode pour obtenir le prochain caractère
    fn next_char(&mut self) {
        self.current_char = self.source.next();
    }



    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    //methode pour afficher un message d'erreur
    fn abort(&self, message: &str) {
        eprintln!("Lexing error: {}", message);
        exit(1);
    }

    // cette methode est notre fonction principale pour obtenir les tokens
    //methode pour obtenir le token suivant

    fn get_token(&mut self) -> Token {
        self.skip_whitespace();
        self.skip_comment();
        let token = match self.current_char {
            Some('+') => Token::new("+".to_string(), TokenType::PLUS),
            Some('-') => Token::new("-".to_string(), TokenType::MINUS),
            Some('*') => Token::new("*".to_string(), TokenType::ASTERISK),
            Some('/') => Token::new("/".to_string(), TokenType::SLASH),
            Some('(') => Token::new("(".to_string(), TokenType::LPAREN),
            Some(')') => Token::new(")".to_string(), TokenType::RPAREN),
            Some('=') => {
                if self.peek() == Some(&'=') {
                    self.next_char();
                    Token::new("==".to_string(), TokenType::EQEQ)
                } else {
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
                if self.peek() == Some(&'=') {
                    self.next_char();
                    Token::new("!=".to_string(), TokenType::NOTEQ)
                } else {
                    self.abort("Expected !=, got !");
                    Token::new("".to_string(), TokenType::EOF) // Ne sera jamais atteint
                }
            }
            Some('\"') => {
                self.next_char();
                let mut string_content = String::new();
                while let Some(&c) = self.peek() {
                    if c == '\"' {
                        break;
                    }
                    if c == '\r' || c == '\n' || c == '\t' || c == '\\' || c == '%' {
                        self.abort("Illegal character in string.");
                    }
                    string_content.push(c);
                    self.next_char();
                }
                self.next_char(); // Consommer le guillemet fermant
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
                    while let Some(&c) = self.peek() {
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
                    "LABEL" => TokenType::LABEL,
                    "GOTO" => TokenType::GOTO,
                    "PRINT" => TokenType::PRINT,
                    "INPUT" => TokenType::INPUT,
                    "LET" => TokenType::LET,
                    "IF" => TokenType::IF,
                    "THEN" => TokenType::THEN,
                    "ENDIF" => TokenType::ENDIF,
                    "WHILE" => TokenType::WHILE,
                    "REPEAT" => TokenType::REPEAT,
                    "ENDWHILE" => TokenType::ENDWHILE,
                    _ => TokenType::IDENT,
                };
                Token::new(ident_content, token_type)
            }
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

    //methode pour ignorer les espaces
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() && c != '\n' {
                self.next_char();
            } else {
                break;
            }
        }
    }
    // fonction/methode  pour ignorer les commentaires :
    // les commentaire commence avec # et se termine avec un retour à la ligne
    fn skip_comment(&mut self) {
        if self.current_char == Some('#') {
            while self.current_char != Some('\n') {
                self.next_char();
            }
        }
    }
}




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