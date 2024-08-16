use std::char;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::{Chars, FromStr};

use num_bigint::BigInt;
use num_traits::identities::Zero;
use num_traits::Num;

use crate::error::{LexerError, LexerErrorType, Position};
use crate::token::{StringKind, TokenType};

#[derive(Debug, PartialEq, Clone)]
struct IndentationLevel {
    tabs: usize,
    spaces: usize,
}

impl IndentationLevel {
    fn compare_strict(&self, other: &IndentationLevel) -> Result<Ordering, LexerError> {
        match self.tabs.cmp(&other.tabs) {
            Ordering::Less => {
                if self.spaces <= other.spaces {
                    Ok(Ordering::Less)
                } else {
                    Err(LexerError {
                        error: LexerErrorType::TabError,
                    })
                }
            }
            Ordering::Greater => {
                if self.spaces >= other.spaces {
                    Ok(Ordering::Greater)
                } else {
                    Err(LexerError {
                        error: LexerErrorType::TabError,
                    })
                }
            }
            Ordering::Equal => Ok(self.spaces.cmp(&other.spaces)),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub text: String,
    pub kind: TokenType,
}

impl Token {
    pub fn new(text: String, kind: TokenType) -> Self {
        Token { text, kind }
    }
}

pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    start_of_line: bool,
    current_char: char,
    indentation_stack: Vec<IndentationLevel>,
    nesting: usize,
    chr0: Option<char>,
    char1: Option<char>,
    char2: Option<char>,
    keywords: HashMap<String, TokenType>,
    buffered_tokens: Vec<Token>, // pour la gestion des token multiple en syntaxe_mode : python
    position: Position,
}

impl<'a> Lexer<'a> {
    pub fn new(code_source: &'a str) -> Self {
        let lexer = Lexer {
            source: code_source.chars().peekable(),
            start_of_line: true,
            current_char: '\0',
            indentation_stack: vec![IndentationLevel { tabs: 0, spaces: 0 }],
            nesting: 0,
            chr0: None,
            char1: None,
            char2: None,
            keywords: HashMap::new(),
            buffered_tokens: Vec::new(),
            position: Position::new(1, 1),
        };
        lexer
    }

    pub fn next_char(&mut self) -> Option<char> {
        self.current_char = self.source.next()?;
        if self.current_char == '\n' {
            self.position.line += 1;
            self.position.column = 1;
        } else {
            self.position.column += 1;
        }
        Some(self.current_char)
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.source.peek()
    }

    fn get_token(&mut self) -> Result<Token, LexerError> {
        if let Some(token) = self.buffered_tokens.pop() {
            return Ok(token);
        }

        self.skip_whitespace();
        self.skip_comment();

        match self.chr0 {
            Some(c) if c.is_alphabetic() || c == '_' => self.lex_identifier(),
            Some(c) if c.is_digit(10) => self.lex_number(),
            Some('"') | Some('\'') => self.lex_string(),
            Some(c) => self.lex_operator_or_delimiter(c),
            None => Ok(Token::new("".to_string(), TokenType::EOF)),
        }
    }

    fn lex_identifier(&mut self) -> Result<Token, LexerError> {
        let mut name = String::new();
        let start_position = self.position;
        while self.is_char() {
            name.push(self.next_char().unwrap());
        }
        let end_position = self.position;

        if let Some(keyword_type) = self.keywords.get(&name) {
            Ok(Token::new(name, keyword_type.clone()))
        } else {
            Ok(Token::new(name.clone(), TokenType::NAME { name }))
        }
    }

    fn lex_number(&mut self) -> Result<Token, LexerError> {
        let mut value_text = String::new();
        let start_position = self.position;
        while self.is_number() {
            value_text.push(self.next_char().unwrap());
        }
        if self.current_char == '.' {
            value_text.push(self.next_char().unwrap());
            while self.is_number() {
                value_text.push(self.next_char().unwrap());
            }
            let value = f64::from_str(&value_text).map_err(|_| LexerError {
                error: LexerErrorType::InvalidNumber,
            })?;
            return Ok(Token::new(value_text, TokenType::FLOAT { value }));
        }
        let value = i32::from_str(&value_text).map_err(|_| LexerError {
            error: LexerErrorType::InvalidNumber,
        })?;
        Ok(Token::new(value_text, TokenType::INTEGER { value: BigInt::from(value) }))
    }

    fn lex_string(&mut self) -> Result<Token, LexerError> {
        let start_char = self.chr0.unwrap();
        let mut string = String::new();
        self.next_char(); // Consommer le guillemet ouvrant

        while let Some(c) = self.chr0 {
            if c == start_char {
                self.next_char(); // Consommer le guillemet fermant
                return Ok(Token::new(string.clone(), TokenType::STRING { value: string, kind: StringKind::NORMAL }));
            }
            if c == '\\' {
                self.next_char();
                // Gérer les caractères d'échappement
                if let Some(next_char) = self.chr0 {
                    match next_char {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        '\\' => string.push('\\'),
                        _ => string.push(next_char),
                    }
                }
                self.next_char();
            } else {
                string.push(c);
                self.next_char();
            }
        }
        Err(LexerError { error: LexerErrorType::UnterminatedString })
    }

    fn is_char(&self) -> bool {
        matches!(self.current_char, 'a'..='z' | 'A'..='Z' | '_')
    }

    fn is_number(&self) -> bool {
        matches!(self.current_char, '0'..='9')
    }
    fn is_operator(&self) -> bool {
        matches!(self.current_char, '+' | '-' | '*' | '/' | '%' | '=' | '!' | '<' | '>')
    }
    fn is_delimiter(&self) -> bool {
        matches!(self.current_char, '(' | ')' | '[' | ']' | '{' | '}' | ',' | ':' | ';' | '.')
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        while let Ok(token) = self.get_token() {
            if token.kind == TokenType::EOF {
                break;
            }
            tokens.push(token);
        }
        Ok(tokens)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if !c.is_whitespace() {
                break;
            }
            self.next_char();
        }
    }

    fn skip_comment(&mut self) {
        if self.chr0 == Some('#') {
            while let Some(c) = self.chr0 {
                if c == '\n' {
                    break;
                }
                self.next_char();
            }
        }
    }

    fn lex_operator_or_delimiter(&mut self, c: char) -> Result<Token, LexerError> {
        let token_type = match c {
            '+' => TokenType::PLUS,
            '-' => TokenType::MINUS,
            '*' => TokenType::STAR,
            '/' => TokenType::STAR,
            '%' => TokenType::MOD,
            '=' => TokenType::EQUAL,
            '!' => TokenType::NOT,
            '<' => TokenType::LESS,
            '>' => TokenType::GREATER,
            '(' => TokenType::LPAR,
            ')' => TokenType::RPAR,
            '[' => TokenType::LSQB,
            ']' => TokenType::RSQB,
            '{' => TokenType::LCURBRACE,
            '}' => TokenType::RCURBRACE,
            ',' => TokenType::COMMA,
            ':' => TokenType::COLON,
            ';' => TokenType::SEMICOLON,
            '.' => TokenType::DOT,
            _ => return Err(LexerError { error: LexerErrorType::InvalidCharacter }),
        };
        Ok(Token::new(c.to_string(), token_type))
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokenization() -> Result<(), LexerError> {
        let source = "x = 10 + 20.5";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;

        assert_eq!(tokens.len(), 5);

        assert!(matches!(&tokens[0].kind, TokenType::NAME { name } if name == "x"));
        assert_eq!(tokens[1].kind, TokenType::EQUAL);
        assert!(matches!(&tokens[2].kind, TokenType::INTEGER { value } if *value == BigInt::from(10)));
        assert_eq!(tokens[3].kind, TokenType::PLUS);
        assert!(matches!(tokens[4].kind, TokenType::FLOAT { value } if (value - 20.5).abs() < f64::EPSILON));

        Ok(())
    }

    #[test]
    fn test_string_and_comment() -> Result<(), LexerError> {
        let source = r#"print("Hello") # This is a comment"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;

        assert_eq!(tokens.len(), 4);

        assert!(matches!(&tokens[0].kind, TokenType::NAME { name } if name == "print"));
        assert_eq!(tokens[1].kind, TokenType::LPAR);
        assert!(matches!(&tokens[2].kind, TokenType::STRING { value, kind } if value == "Hello" && *kind == StringKind::NORMAL));
        assert_eq!(tokens[3].kind, TokenType::RPAR);

        Ok(())
    }

    #[test]
    fn test_complex_expression() -> Result<(), LexerError> {
        let source = "if (x > 5 and y < 10) or z == 0:";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize()?;

        assert_eq!(tokens.len(), 13);

        assert!(matches!(&tokens[0].kind, TokenType::NAME { name } if name == "if"));
        assert_eq!(tokens[1].kind, TokenType::LPAR);
        assert!(matches!(&tokens[2].kind, TokenType::NAME { name } if name == "x"));
        assert_eq!(tokens[3].kind, TokenType::GREATER);
        assert!(matches!(&tokens[4].kind, TokenType::INTEGER { value } if *value == BigInt::from(5)));
        // ... et ainsi de suite pour les autres tokens

        Ok(())
    }
}