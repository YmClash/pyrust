//
//
// //use crate::error::{LexerError, LexerErrorKind};
//
// use num_bigint::BigInt;
// use std::fmt::{self, Write};
// use std::fmt::{Display, Formatter};
//
//
//
// /// PYRUST TOKEN TYPE
// /// Représente les différents types de tokens
//
//
//
//
// #[derive(Debug, PartialEq, Clone)]
// pub enum TokenType {
//     NAME{name: String},
//     INTEGER{value: BigInt},
//     FLOAT{value: f64},
//     COMPLEX{real: f64, imag: f64},
//     STRING{value: String, kind: StringKind},
//     BYTES{value: Vec<u8>},
//     EOF,
//     NEWLINE,
//     INDENT,
//     DEDENT,
//     STARTMODULE,
//     STARTEXPRESSION,
//
//     //OPERATORS,
//     PLUS, // '+' PLUS / PLUS SIGN
//     MINUS, // '-' MOINS / MINUS SIGN
//     STAR, // '*' ETOILE / STAR
//     SLASH, // '/' SLASH / SLASH
//     VBAR, // '|' BARRE VERTICALE / VERTICAL BAR
//     AMPER, // '&' ET COMMERCIAL / AMPERSAND
//     LESS, // '<' INFERIEUR / LESS-THAN SIGN
//     GREATER, // '>' SUPERIEUR / GREATER-THAN SIGN
//     EQUAL, // '=' EGAL / EQUALS SIGN
//     PERCENT, // '%' POURCENTAGE / PERCENT
//     EQEQUAL, // '==' EGAL EGAL / EQUALS EQUALS
//     NOTEQUAL, // '!=' DIFFERENT / NOT EQUAL
//     LESSEQUAL, // '<=' INFERIEUR EGAL / LESS-THAN EQUAL
//     FATARROW, // '=>' IMPLIQUE / IMPLIES
//     GREATEREQUAL, // '>=' SUPERIEUR EGAL / GREATER-THAN EQUAL
//     TILDE, // '~' TILDE / TILDE
//     CIRCUMFLEX, // '^' CIRCONFLEXE / CIRCUMFLEX
//     LEFTSHIFT, // '<<' DECALAGE GAUCHE / LEFT SHIFT
//     RIGHTSHIFT, // '>>' DECALAGE DROITE / RIGHT SHIFT
//     DOUBLESTAR, // '**' DOUBLE ÉTOILE / DOUBLE STAR
//     PLUSEQUAL, // '+=' PLUS EGAL / PLUS EQUAL
//     MINEQUAL, // '-=' MOINS EGAL / MINUS EQUAL
//     STAREQUAL, // '*=' ETOILE EGAL / STAR EQUAL
//     SLASHEQUAL, // '/=' SLASH EGAL / SLASH EQUAL
//     PERCENTEQUAL, // '%=' POURCENTAGE EGAL / PERCENT EQUAL
//     AMPEREQUAL, // '&=' ET COMMERCIAL EGAL / AMPERSAND EQUAL
//     VBAREQUAL, // '|=' BARRE VERTICALE EGAL / VERTICAL BAR EQUAL
//     CIRCUMFLEXEQUAL, // '^=' CIRCONFLEXE EGAL / CIRCUMFLEX EQUAL
//     LEFTSHIFTEQUAL, // '<<=' DECALAGE GAUCHE EGAL / LEFT SHIFT EQUAL
//     RIGHTSHIFTEQUAL, // '>>=' DECALAGE DROITE EGAL / RIGHT SHIFT EQUAL
//     DOUBLESTAREQUAL, // '**=' DOUBLE ETOILE EGAL / DOUBLE STAR EQUAL
//     DOUBLESLASH, // '//' DOUBLE SLASH / DOUBLE SLASH
//     DOUBLESLASHEQUAL, // '//=' DOUBLE SLASH EGAL / DOUBLE SLASH EQUAL
//     AT, // '@' AROBASE / AT
//     ATEQUAL, // '@=' AROBASE EGAL / AT EQUAL
//     RARROW, // '->' FLECHE DROITE / RIGHT ARROW
// //    ELLIPSIS, // '...' POINTS DE SUSPENSION / ELLIPSIS
//     COLONEQUAL, // ':=' DEUX POINT EGAL / COLON EQUAL
//     STARSLASH, // '*/' ETOILE SLASH / STAR SLASH
//     SLASHSTAR, // '/*' SLASH ETOILE / SLASH STAR
//     DIESE, // '#' DIESE / HASH
//     EXCLAMATION, // '!' POINT D'EXCLAMATION / EXCLAMATION POINT
//     INTERROGATION, // '?' POINT D'INTERROGATION / QUESTION MARK
//
//     //DELIMITERS,
//     LPAR, // '(' PARANTHESE GAUCHE / LEFT PARENTHESIS
//     RPAR, // ')' PARANTHESE DROITE / RIGHT PARENTHESIS
//     LSQB, // '[' CROCHET GAUCHE / LEFT SQUARE BRACKET
//     RSQB, // ']' CROCHET DROIT / RIGHT SQUARE BRACKET
//     COLON, // ':' DEUX POINT / COLON
//     COMMA, // ',' VIRGULE / COMMA
//     SEMICOLON, // ';' POINT VIRGULE / SEMICOLON
//     DOT, // '.' POINT / DOT
//     LCURBRACE, // '{' ACCOLADE GAUCHE / LEFT CURLY BRACKET
//     RCURBRACE, // '}' ACCOLADE DROITE / RIGHT CURLY BRACKET
//     ELLIPSIS, // '...' POINTS DE SUSPENSION / ELLIPSIS
//     DCOLON, // '::' DEUX POINT DEUX POINT / DOUBLE COLON
//
//     //KEYWORDS,
//     AND, AS ,ASYNC, AWAIT, BREAK, CONST, CLASS, CONTINUE, DEF, DEL, ELIF, ELSE,
//     ENUM, EXCEPT, FALSE, FN, FOR, FROM, IF, IMPL, IMPORT, IN, IS, LAMBDA, LET,
//     LOOP, MATCH, MOD, MUT, NONE, NOT, OR, PUB, PASS, RAISE, RETURN, SELF,
//     STATIC, STRUCT, SUPER, TRUE, TRY, TYPE, TYPEOF, USE, WITH, WHILE, YIELD,
//     // KEYWORD(Keywords),
//     //OPERATOR(Operator),
//     //DELIMITERS(Delimiter),
//     //LITERALS(Literal),
//
// }
//
//
//
//
//
// impl TokenType {
//     pub fn is_keyword(&self) -> bool {
//         matches!(self, TokenType::AND | TokenType::AS | TokenType::ASYNC | TokenType::AWAIT |
//             TokenType::BREAK| TokenType::CONST | TokenType::CLASS | TokenType::CONTINUE |
//             TokenType::DEF | TokenType::DEL | TokenType::ELIF | TokenType::ELSE | TokenType::ENUM |
//             TokenType::EXCEPT | TokenType::FALSE | TokenType::FN | TokenType::FOR | TokenType::FROM |
//             TokenType::IF | TokenType::IMPL | TokenType::IMPORT | TokenType::IN | TokenType::IS |
//             TokenType::LAMBDA | TokenType::LET | TokenType::LOOP | TokenType::MATCH | TokenType::MOD |
//             TokenType::MUT | TokenType::NONE | TokenType::NOT | TokenType::OR | TokenType::PUB |
//             TokenType::PASS | TokenType::RAISE | TokenType::RETURN | TokenType::SELF | TokenType::STATIC |
//             TokenType::STRUCT | TokenType::SUPER | TokenType::TRUE | TokenType::TRY | TokenType::TYPE |
//             TokenType::TYPEOF | TokenType::USE | TokenType::WITH | TokenType::WHILE | TokenType::YIELD)
//     }
//
//     pub fn is_operator(&self) -> bool {
//         matches!(self, TokenType::PLUS | TokenType::MINUS | TokenType::STAR | TokenType::SLASH |
//             TokenType::VBAR | TokenType::AMPER | TokenType::LESS | TokenType::GREATER | TokenType::EQUAL |
//             TokenType::PERCENT | TokenType::EQEQUAL | TokenType::NOTEQUAL | TokenType::LESSEQUAL |
//             TokenType::FATARROW | TokenType::GREATEREQUAL | TokenType::TILDE | TokenType::CIRCUMFLEX |
//             TokenType::LEFTSHIFT | TokenType::RIGHTSHIFT | TokenType::DOUBLESTAR | TokenType::PLUSEQUAL |
//             TokenType::MINEQUAL | TokenType::STAREQUAL | TokenType::SLASHEQUAL | TokenType::PERCENTEQUAL |
//             TokenType::AMPEREQUAL | TokenType::VBAREQUAL | TokenType::CIRCUMFLEXEQUAL | TokenType::LEFTSHIFTEQUAL |
//             TokenType::RIGHTSHIFTEQUAL | TokenType::DOUBLESTAREQUAL | TokenType::DOUBLESLASH | TokenType::DOUBLESLASHEQUAL |
//             TokenType::AT | TokenType::ATEQUAL | TokenType::RARROW | TokenType::COLONEQUAL | TokenType::STARSLASH |
//             TokenType::SLASHSTAR | TokenType::DIESE | TokenType::EXCLAMATION | TokenType::INTERROGATION)
//     }
//
//     pub fn is_delimiter(&self) -> bool {
//         matches!(self, TokenType::LPAR | TokenType::RPAR | TokenType::LSQB | TokenType::RSQB |
//             TokenType::COLON | TokenType::COMMA | TokenType::SEMICOLON | TokenType::DOT | TokenType::LCURBRACE |
//             TokenType::RCURBRACE | TokenType::ELLIPSIS | TokenType::DCOLON)
//     }
// }
//
//
//
//
//
//
//
//
//
//
// #[derive(Debug, PartialEq,Eq, Clone)]
// pub enum StringKind{
//     NORMAL,
//     FORMATTED,
//     UNICODE,
// }
// impl Display for TokenType {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         use TokenType::*;
//         match self {
//             NAME { name } => write!(f, "NAME({})", name),
//             INTEGER { value } => write!(f, "INTEGER({})", value),
//             FLOAT { value } => write!(f, "FLOAT({})", value),
//             COMPLEX { real, imag } => write!(f, "COMPLEX({}, {})", real, imag),
//             STRING { value, kind } => {
//                 match kind {
//                     StringKind::FORMATTED => write!(f, "STRING({:?})", value),
//                     StringKind::UNICODE => write!(f, "STRING({:?})", value),
//                     StringKind::NORMAL => write!(f, "STRING({:?})", value),
//                 }
//             }
//             BYTES { value } => {
//                 write!(f, "b\"")?;
//                 for byte in value {
//                     match byte {
//                         9 => f.write_str(r"\\t")?,
//                         10 => f.write_str(r"\\n")?,
//                         13 => f.write_str(r"\\r")?,
//                         32..=126 => f.write_char(*byte as char)?,
//                         _ => write!(f, "\\x{:02x}", byte)?,
//                     }
//                 }
//                 f.write_str("\"")
//             }
//             EOF => f.write_str("EOF"),
//             NEWLINE => f.write_str("NEWLINE"),
//             INDENT => f.write_str("INDENT"),
//             DEDENT => f.write_str("DEDENT"),
//             STARTMODULE => f.write_str("STARTMODULE"),
//             STARTEXPRESSION => f.write_str("STARTEXPRESSION"),
//             PLUS => f.write_str("PLUS"),
//             MINUS => f.write_str("MINUS"),
//             STAR => f.write_str("STAR"),
//             SLASH => f.write_str("SLASH"),
//             VBAR => f.write_str("VBAR"),
//             AMPER => f.write_str("AMPER"),
//             LESS => f.write_str("LESS"),
//             GREATER => f.write_str("GREATER"),
//             EQUAL => f.write_str("EQUAL"),
//             PERCENT => f.write_str("PERCENT"),
//             EQEQUAL => f.write_str("EQEQUAL"),
//             NOTEQUAL => f.write_str("NOTEQUAL"),
//             LESSEQUAL => f.write_str("LESSEQUAL"),
//             FATARROW => f.write_str("FATARROW"),
//             GREATEREQUAL => f.write_str("GREATEREQUAL"),
//             TILDE => f.write_str("TILDE"),
//             CIRCUMFLEX => f.write_str("CIRCUMFLEX"),
//             LEFTSHIFT => f.write_str("LEFTSHIFT"),
//             RIGHTSHIFT => f.write_str("RIGHTSHIFT"),
//             DOUBLESTAR => f.write_str("DOUBLESTAR"),
//             PLUSEQUAL => f.write_str("PLUSEQUAL"),
//             MINEQUAL => f.write_str("MINEQUAL"),
//             STAREQUAL => f.write_str("STAREQUAL"),
//             SLASHEQUAL => f.write_str("SLASHEQUAL"),
//             PERCENTEQUAL => f.write_str("PERCENTEQUAL"),
//             AMPEREQUAL => f.write_str("AMPEREQUAL"),
//             VBAREQUAL => f.write_str("VBAREQUAL"),
//             CIRCUMFLEXEQUAL => f.write_str("CIRCUMFLEXEQUAL"),
//             LEFTSHIFTEQUAL => f.write_str("LEFTSHIFTEQUAL"),
//             RIGHTSHIFTEQUAL => f.write_str("RIGHTSHIFTEQUAL"),
//             DOUBLESTAREQUAL => f.write_str("DOUBLESTAREQUAL"),
//             DOUBLESLASH => f.write_str("DOUBLESLASH"),
//             DOUBLESLASHEQUAL => f.write_str("DOUBLESLASHEQUAL"),
//             AT => f.write_str("AT"),
//             ATEQUAL => f.write_str("ATEQUAL"),
//             RARROW => f.write_str("RARROW"),
//             COLONEQUAL => f.write_str("COLONEQUAL"),
//             STARSLASH => f.write_str("STARSLASH"),
//             SLASHSTAR => f.write_str("SLASHSTAR"),
//             DIESE => f.write_str("DIESE"),
//             EXCLAMATION => f.write_str("EXCLAMATION"),
//             INTERROGATION => f.write_str("INTERROGATION"),
//             LPAR => f.write_str("LPAR"),
//             RPAR => f.write_str("RPAR"),
//             LSQB => f.write_str("LSQB"),
//             RSQB => f.write_str("RSQB"),
//             COLON => f.write_str("COLON"),
//             COMMA => f.write_str("COMMA"),
//             SEMICOLON => f.write_str("SEMICOLON"),
//             DOT => f.write_str("DOT"),
//             LCURBRACE => f.write_str("LCURBRACE"),
//             RCURBRACE => f.write_str("RCURBRACE"),
//             ELLIPSIS => f.write_str("ELLIPSIS"),
//             DCOLON => f.write_str("DCOLON"),
//             AND => f.write_str("AND"),
//             AS => f.write_str("AS"),
//             ASYNC => f.write_str("ASYNC"),
//             AWAIT => f.write_str("AWAIT"),
//             BREAK => f.write_str("BREAK"),
//             CONST => f.write_str("CONST"),
//             CLASS => f.write_str("CLASS"),
//             CONTINUE => f.write_str("CONTINUE"),
//             DEF => f.write_str("DEF"),
//             DEL => f.write_str("DEL"),
//             ELIF => f.write_str("ELIF"),
//             ELSE => f.write_str("ELSE"),
//             ENUM => f.write_str("ENUM"),
//             EXCEPT => f.write_str("EXCEPT"),
//             FALSE => f.write_str("FALSE"),
//             FN => f.write_str("FN"),
//             FOR => f.write_str("FOR"),
//             FROM => f.write_str("FROM"),
//             IF => f.write_str("IF"),
//             IMPL => f.write_str("IMPL"),
//             IMPORT => f.write_str("IMPORT"),
//             IN => f.write_str("IN"),
//             IS => f.write_str("IS"),
//             LAMBDA => f.write_str("LAMBDA"),
//             LET => f.write_str("LET"),
//             LOOP => f.write_str("LOOP"),
//             MATCH => f.write_str("MATCH"),
//             MOD => f.write_str("MOD"),
//             MUT => f.write_str("MUT"),
//             NONE => f.write_str("NONE"),
//             NOT => f.write_str("NOT"),
//             OR => f.write_str("OR"),
//             PUB => f.write_str("PUB"),
//             PASS => f.write_str("PASS"),
//             RAISE => f.write_str("RAISE"),
//             RETURN => f.write_str("RETURN"),
//             SELF => f.write_str("SELF"),
//             STATIC => f.write_str("STATIC"),
//             STRUCT => f.write_str("STRUCT"),
//             SUPER => f.write_str("SUPER"),
//             TRUE => f.write_str("TRUE"),
//             TRY => f.write_str("TRY"),
//             TYPE => f.write_str("TYPE"),
//             TYPEOF => f.write_str("TYPEOF"),
//             USE => f.write_str("USE"),
//             WITH => f.write_str("WITH"),
//             WHILE => f.write_str("WHILE"),
//             YIELD => f.write_str("YIELD"),
//         }
//     }
// }
//
//
// ////  Essai  implementation  de  token_type is_keyword is_operator is_literal is_delimiter
//
//
//
//
//
//
//
// //
// // #[derive(Debug, PartialEq, Clone)]
// // pub enum TokenType {
// //     // END MARKERS
// //     ENDER(Endmarker),
// //     // Special Token
// //     NEWLINE,
// //     INDENT,
// //     DEDENT,
// //     BOOLEAN,
// //
// //     KEYWORD(Keywords),
// //     OPERATOR(Operator),
// //     DELIMITERS(Delimiter),
// //     LITERALS(Literal),
// //
// //     COMMENT,     // COMMENTAIRE / COMMENT
// //     NL, // NOUVELLE LIGNE / NEW LINE
// // }
// //
// // /// Représente les différents types de mots-clés du langage
// //
// // #[derive(Debug, PartialEq, Eq, Clone)]
// // pub enum Keywords {
// //     AND, AS, ASYNC, AWAIT, BREAK, CONST, CLASS, CONTINUE, DEF, DEL, ELIF, ELSE,
// //     ENUM, EXCEPT, FALSE, FN, FOR, FROM, IF, IMPL, IMPORT, IN, IS, LAMBDA, LET,
// //     LOOP, MATCH, MOD, MUT, NONE, NOT, OR, PUB, PASS, RAISE, RETURN, SELF,
// //     STATIC, STRUCT, SUPER, TRUE, TRY, TYPE, TYPEOF, USE, WITH, WHILE, YIELD,
// // }
// //
// // #[derive(Debug, PartialEq, Eq, Clone)]
// // pub enum Operator {
// //     // OPERATORS
// //     PLUS, // '+' PLUS / PLUS SIGN
// //     MINUS, // '-' MOINS / MINUS SIGN
// //     STAR, // '*' ETOILE / STAR
// //     SLASH, // '/' SLASH / SLASH
// //     VBAR, // '|' BARRE VERTICALE / VERTICAL BAR
// //     AMPER, // '&' ET COMMERCIAL / AMPERSAND
// //     LESS, // '<' INFERIEUR / LESS-THAN SIGN
// //     GREATER, // '>' SUPERIEUR / GREATER-THAN SIGN
// //     EQUAL, // '=' EGAL / EQUALS SIGN
// //     PERCENT, // '%' POURCENTAGE / PERCENT
// //     EQEQUAL, // '==' EGAL EGAL / EQUALS EQUALS
// //     NOTEQUAL, // '!=' DIFFERENT / NOT EQUAL
// //     LESSEQUAL, // '<=' INFERIEUR EGAL / LESS-THAN EQUAL
// //     FATARROW, // '=>' IMPLIQUE / IMPLIES
// //     GREATEREQUAL, // '>=' SUPERIEUR EGAL / GREATER-THAN EQUAL
// //     TILDE, // '~' TILDE / TILDE
// //     CIRCUMFLEX, // '^' CIRCONFLEXE / CIRCUMFLEX
// //     LEFTSHIFT, // '<<' DECALAGE GAUCHE / LEFT SHIFT
// //     RIGHTSHIFT, // '>>' DECALAGE DROITE / RIGHT SHIFT
// //     DOUBLESTAR, // '**' DOUBLE ÉTOILE / DOUBLE STAR
// //     PLUSEQUAL, // '+=' PLUS EGAL / PLUS EQUAL
// //     MINEQUAL, // '-=' MOINS EGAL / MINUS EQUAL
// //     STAREQUAL, // '*=' ETOILE EGAL / STAR EQUAL
// //     SLASHEQUAL, // '/=' SLASH EGAL / SLASH EQUAL
// //     PERCENTEQUAL, // '%=' POURCENTAGE EGAL / PERCENT EQUAL
// //     AMPEREQUAL, // '&=' ET COMMERCIAL EGAL / AMPERSAND EQUAL
// //     VBAREQUAL, // '|=' BARRE VERTICALE EGAL / VERTICAL BAR EQUAL
// //     CIRCUMFLEXEQUAL, // '^=' CIRCONFLEXE EGAL / CIRCUMFLEX EQUAL
// //     LEFTSHIFTEQUAL, // '<<=' DECALAGE GAUCHE EGAL / LEFT SHIFT EQUAL
// //     RIGHTSHIFTEQUAL, // '>>=' DECALAGE DROITE EGAL / RIGHT SHIFT EQUAL
// //     DOUBLESTAREQUAL, // '**=' DOUBLE ETOILE EGAL / DOUBLE STAR EQUAL
// //     DOUBLESLASH, // '//' DOUBLE SLASH / DOUBLE SLASH
// //     DOUBLESLASHEQUAL, // '//=' DOUBLE SLASH EGAL / DOUBLE SLASH EQUAL
// //     AT, // '@' AROBASE / AT
// //     ATEQUAL, // '@=' AROBASE EGAL / AT EQUAL
// //     RARROW, // '->' FLECHE DROITE / RIGHT ARROW
// //     ELLIPSIS, // '...' POINTS DE SUSPENSION / ELLIPSIS
// //     COLONEQUAL, // ':=' DEUX POINT EGAL / COLON EQUAL
// //     STARSLASH, // '*/' ETOILE SLASH / STAR SLASH
// //     SLASHSTAR, // '/*' SLASH ETOILE / SLASH STAR
// //     DIESE, // '#' DIESE / HASH
// //     EXCLAMATION, // '!' POINT D'EXCLAMATION / EXCLAMATION POINT
// //     INTERROGATION, // '?' POINT D'INTERROGATION / QUESTION MARK
// // }
// //
// // #[derive(Debug, PartialEq, Eq, Clone)]
// // pub enum Delimiter {
// //     LPAR, // '(' PARANTHESE GAUCHE / LEFT PARENTHESIS
// //     RPAR, // ')' PARANTHESE DROITE / RIGHT PARENTHESIS
// //     LSQB, // '[' CROCHET GAUCHE / LEFT SQUARE BRACKET
// //     RSQB, // ']' CROCHET DROIT / RIGHT SQUARE BRACKET
// //     COLON, // ':' DEUX POINT / COLON
// //     COMMA, // ',' VIRGULE / COMMA
// //     SEMICOLON, // ';' POINT VIRGULE / SEMICOLON
// //     DOT, // '.' POINT / DOT
// //     LCURBRACE, // '{' ACCOLADE GAUCHE / LEFT CURLY BRACKET
// //     RCURBRACE, // '}' ACCOLADE DROITE / RIGHT CURLY BRACKET
// //     ELLIPSIS, // '...' POINTS DE SUSPENSION / ELLIPSIS
// //     DCOLON, // '::' DEUX POINT DEUX POINT / DOUBLE COLON
// // }
// //
// // // Utiliser un type personnalisé pour représenter les nombres à virgule flottante qui implémente Eq
// // // #[derive(Debug, PartialEq, Eq, Clone)]
// // // pub struct Float(String);
// //
// // #[derive(Debug, PartialEq, Clone)]
// // pub enum Literal {
// //     IDENTIFIER(String),
// //     NUMBER(i64),
// //     STRING(String),
// //     INTEGER(i64),
// //     FLOAT(f64),
// //     HEXNUMBER(String),
// //     BOOLEAN(bool),
// //     CHAR(char),
// // }
// // #[derive(Debug, PartialEq, Eq, Clone)]
// // pub enum Endmarker {
// //     ENDMARKER,
// //     EOF,
// // }
// //
// // impl TokenType {
// //     /// Vérifie si le token est un mot-clé
// //     pub fn is_keyword(&self) -> bool {
// //         matches!(self, TokenType::KEYWORD(_))
// //     }
// //
// //     /// Vérifie si le token est un opérateur
// //     pub fn is_operator(&self) -> bool {
// //         matches!(self, TokenType::OPERATOR(_))
// //     }
// //
// //     /// Vérifie si le token est un littéral
// //     pub fn is_literal(&self) -> bool {
// //         matches!(self, TokenType::LITERALS(_))
// //     }
// //
// //     /// Vérifie si le token est un délimiteur
// //     pub fn is_delimiter(&self) -> bool {
// //         matches!(self, TokenType::DELIMITERS(_))
// //     }
// //
// //     /// Vérifie si le token est un marqueur de fin
// //     pub fn is_end_marker(&self) -> bool {
// //         matches!(self, TokenType::ENDER(_))
// //     }
// // }
// //
// //
// //
// // // Implementation de Display pour TokenType
// // impl fmt::Display for TokenType {
// //     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
// //         match self {
// //             TokenType::KEYWORD(kw) => write!(f, "{:?}", kw),
// //             TokenType::OPERATOR(op) => write!(f, "{:?}", op),
// //             TokenType::DELIMITERS(del) => write!(f, "{:?}", del),
// //             TokenType::LITERALS(lit) => write!(f, "{:?}", lit),
// //             TokenType::ENDER(end) => write!(f, "{:?}", end),
// //             TokenType::NEWLINE => write!(f, "NEWLINE"),
// //             TokenType::INDENT => write!(f, "INDENT"),
// //             TokenType::DEDENT => write!(f, "DEDENT"),
// //             TokenType::BOOLEAN => write!(f, "BOOLEAN"),
// //             TokenType::COMMENT => write!(f, "COMMENT"),
// //             TokenType::NL => write!(f, "NL"),
// //         }
// //     }
// // }
// //
// // // Implementation de From<String> pour TokenType
// // impl FromStr for TokenType {
// //     type Err = ();
// //
// //     fn from_str(s: &str) -> Result<Self, Self::Err> {
// //         match s {
// //             // Opérateurs
// //             "+" => Ok(TokenType::OPERATOR(Operator::PLUS)),
// //             "-" => Ok(TokenType::OPERATOR(Operator::MINUS)),
// //             "*" => Ok(TokenType::OPERATOR(Operator::STAR)),
// //             "/" => Ok(TokenType::OPERATOR(Operator::SLASH)),
// //             "|" => Ok(TokenType::OPERATOR(Operator::VBAR)),
// //             "&" => Ok(TokenType::OPERATOR(Operator::AMPER)),
// //             "<" => Ok(TokenType::OPERATOR(Operator::LESS)),
// //             ">" => Ok(TokenType::OPERATOR(Operator::GREATER)),
// //             "=" => Ok(TokenType::OPERATOR(Operator::EQUAL)),
// //             "%" => Ok(TokenType::OPERATOR(Operator::PERCENT)),
// //             "==" => Ok(TokenType::OPERATOR(Operator::EQEQUAL)),
// //             "!=" => Ok(TokenType::OPERATOR(Operator::NOTEQUAL)),
// //             "<=" => Ok(TokenType::OPERATOR(Operator::LESSEQUAL)),
// //             "=>" => Ok(TokenType::OPERATOR(Operator::FATARROW)),
// //             ">=" => Ok(TokenType::OPERATOR(Operator::GREATEREQUAL)),
// //             "~" => Ok(TokenType::OPERATOR(Operator::TILDE)),
// //             "^" => Ok(TokenType::OPERATOR(Operator::CIRCUMFLEX)),
// //             "<<" => Ok(TokenType::OPERATOR(Operator::LEFTSHIFT)),
// //             ">>" => Ok(TokenType::OPERATOR(Operator::RIGHTSHIFT)),
// //             "**" => Ok(TokenType::OPERATOR(Operator::DOUBLESTAR)),
// //             "+=" => Ok(TokenType::OPERATOR(Operator::PLUSEQUAL)),
// //             "-=" => Ok(TokenType::OPERATOR(Operator::MINEQUAL)),
// //             "*=" => Ok(TokenType::OPERATOR(Operator::STAREQUAL)),
// //             "/=" => Ok(TokenType::OPERATOR(Operator::SLASHEQUAL)),
// //             "%=" => Ok(TokenType::OPERATOR(Operator::PERCENTEQUAL)),
// //             "&=" => Ok(TokenType::OPERATOR(Operator::AMPEREQUAL)),
// //             "|=" => Ok(TokenType::OPERATOR(Operator::VBAREQUAL)),
// //             "^=" => Ok(TokenType::OPERATOR(Operator::CIRCUMFLEXEQUAL)),
// //             "<<=" => Ok(TokenType::OPERATOR(Operator::LEFTSHIFTEQUAL)),
// //             ">>=" => Ok(TokenType::OPERATOR(Operator::RIGHTSHIFTEQUAL)),
// //             "**=" => Ok(TokenType::OPERATOR(Operator::DOUBLESTAREQUAL)),
// //             "//" => Ok(TokenType::OPERATOR(Operator::DOUBLESLASH)),
// //             "//=" => Ok(TokenType::OPERATOR(Operator::DOUBLESLASHEQUAL)),
// //             "@" => Ok(TokenType::OPERATOR(Operator::AT)),
// //             "@=" => Ok(TokenType::OPERATOR(Operator::ATEQUAL)),
// //             "->" => Ok(TokenType::OPERATOR(Operator::RARROW)),
// //             ":=" => Ok(TokenType::OPERATOR(Operator::COLONEQUAL)),
// //             // "::" => Ok(TokenType::OPERATOR(Operator::DCOLON)),
// //             "*/" => Ok(TokenType::OPERATOR(Operator::STARSLASH)),
// //             "/*" => Ok(TokenType::OPERATOR(Operator::SLASHSTAR)),
// //             "#" => Ok(TokenType::OPERATOR(Operator::DIESE)),
// //
// //             // Délimiteurs
// //             "(" => Ok(TokenType::DELIMITERS(Delimiter::LPAR)),
// //             ")" => Ok(TokenType::DELIMITERS(Delimiter::RPAR)),
// //             "[" => Ok(TokenType::DELIMITERS(Delimiter::LSQB)),
// //             "]" => Ok(TokenType::DELIMITERS(Delimiter::RSQB)),
// //             ":" => Ok(TokenType::DELIMITERS(Delimiter::COLON)),
// //             "," => Ok(TokenType::DELIMITERS(Delimiter::COMMA)),
// //             ";" => Ok(TokenType::DELIMITERS(Delimiter::SEMICOLON)),
// //             "." => Ok(TokenType::DELIMITERS(Delimiter::DOT)),
// //             "::" => Ok(TokenType::DELIMITERS(Delimiter::DCOLON)),
// //             "{" => Ok(TokenType::DELIMITERS(Delimiter::LCURBRACE)),
// //             "}" => Ok(TokenType::DELIMITERS(Delimiter::RCURBRACE)),
// //             "..." => Ok(TokenType::DELIMITERS(Delimiter::ELLIPSIS)),
// //
// //             // Mots-clés
// //             "and" => Ok(TokenType::KEYWORD(Keywords::AND)),
// //             "as" => Ok(TokenType::KEYWORD(Keywords::AS)),
// //             "async" => Ok(TokenType::KEYWORD(Keywords::ASYNC)),
// //             "await" => Ok(TokenType::KEYWORD(Keywords::AWAIT)),
// //             "break" => Ok(TokenType::KEYWORD(Keywords::BREAK)),
// //             "const" => Ok(TokenType::KEYWORD(Keywords::CONST)),
// //             "class" => Ok(TokenType::KEYWORD(Keywords::CLASS)),
// //             "continue" => Ok(TokenType::KEYWORD(Keywords::CONTINUE)),
// //             "def" => Ok(TokenType::KEYWORD(Keywords::DEF)),
// //             "del" => Ok(TokenType::KEYWORD(Keywords::DEL)),
// //             "elif" => Ok(TokenType::KEYWORD(Keywords::ELIF)),
// //             "else" => Ok(TokenType::KEYWORD(Keywords::ELSE)),
// //             "enum" => Ok(TokenType::KEYWORD(Keywords::ENUM)),
// //             "except" => Ok(TokenType::KEYWORD(Keywords::EXCEPT)),
// //             "false" | "False" | "FALSE" => Ok(TokenType::KEYWORD(Keywords::FALSE)),
// //             "fn" => Ok(TokenType::KEYWORD(Keywords::FN)),
// //             "for" => Ok(TokenType::KEYWORD(Keywords::FOR)),
// //             "from" => Ok(TokenType::KEYWORD(Keywords::FROM)),
// //             "if" => Ok(TokenType::KEYWORD(Keywords::IF)),
// //             "impl" => Ok(TokenType::KEYWORD(Keywords::IMPL)),
// //             "import" => Ok(TokenType::KEYWORD(Keywords::IMPORT)),
// //             "in" => Ok(TokenType::KEYWORD(Keywords::IN)),
// //             "is" => Ok(TokenType::KEYWORD(Keywords::IS)),
// //             "lambda" => Ok(TokenType::KEYWORD(Keywords::LAMBDA)),
// //             "let" => Ok(TokenType::KEYWORD(Keywords::LET)),
// //             "loop" => Ok(TokenType::KEYWORD(Keywords::LOOP)),
// //             "match" => Ok(TokenType::KEYWORD(Keywords::MATCH)),
// //             "mod" => Ok(TokenType::KEYWORD(Keywords::MOD)),
// //             "mut" => Ok(TokenType::KEYWORD(Keywords::MUT)),
// //             "none" | "None" | "NONE" => Ok(TokenType::KEYWORD(Keywords::NONE)),
// //             "not" => Ok(TokenType::KEYWORD(Keywords::NOT)),
// //             "or" => Ok(TokenType::KEYWORD(Keywords::OR)),
// //             "pub" => Ok(TokenType::KEYWORD(Keywords::PUB)),
// //             "pass" => Ok(TokenType::KEYWORD(Keywords::PASS)),
// //             "raise" => Ok(TokenType::KEYWORD(Keywords::RAISE)),
// //             "return" => Ok(TokenType::KEYWORD(Keywords::RETURN)),
// //             "self" => Ok(TokenType::KEYWORD(Keywords::SELF)),
// //             "static" => Ok(TokenType::KEYWORD(Keywords::STATIC)),
// //             "struct" => Ok(TokenType::KEYWORD(Keywords::STRUCT)),
// //             "super" => Ok(TokenType::KEYWORD(Keywords::SUPER)),
// //             "true" | "True" | "TRUE" => Ok(TokenType::KEYWORD(Keywords::TRUE)),
// //             "try" => Ok(TokenType::KEYWORD(Keywords::TRY)),
// //             "type" => Ok(TokenType::KEYWORD(Keywords::TYPE)),
// //             "typeof" => Ok(TokenType::KEYWORD(Keywords::TYPEOF)),
// //             "use" => Ok(TokenType::KEYWORD(Keywords::USE)),
// //             "with" => Ok(TokenType::KEYWORD(Keywords::WITH)),
// //             "while" => Ok(TokenType::KEYWORD(Keywords::WHILE)),
// //             "yield" => Ok(TokenType::KEYWORD(Keywords::YIELD)),
// //
// //             // Littéraux
// //             s if s.starts_with('"') && s.ends_with('"') => Ok(TokenType::LITERALS(Literal::STRING(s[1..s.len()-1].to_string()))),
// //             s if s.parse::<i64>().is_ok() => Ok(TokenType::LITERALS(Literal::INTEGER(s.parse().unwrap()))),
// //             s if s.starts_with("0x") => Ok(TokenType::LITERALS(Literal::HEXNUMBER(s.to_string()))),
// //             s if s.parse::<f64>().is_ok() => Ok(TokenType::LITERALS(Literal::FLOAT(s.parse().unwrap()))),
// //             s if s.starts_with('\'') && s.ends_with('\'') && s.len() == 3 =>
// //                 Ok(TokenType::LITERALS(Literal::CHAR(s.chars().nth(1).unwrap()))),
// //
// //             _ => Ok(TokenType::LITERALS(Literal::IDENTIFIER(s.to_string()))),
// //         }
// //     }
// //
// // }
// //
// // impl LexerError {
// //     pub fn invalid_integer(number: String, line: usize, column: usize) -> Self {
// //         LexerError::new(
// //             LexerErrorKind::InvalidToken(number), // Utilisation correcte de InvalidToken
// //             format!("Invalid integer: {}", number),
// //             line,
// //             column,
// //         )
// //     }
// //
// //     // pub fn invalid_character(c: char, line: usize, column: usize) -> Self {
// //     //     LexerError::new(
// //     //         LexerErrorKind::InvalidCharacter(), // Conversion de char en String
// //     //         format!("Invalid character: {}", c),
// //     //         line,
// //     //         column,
// //     //     )
// //     // }
// //
// //     pub fn invalid_token(t: &str, line: usize, column: usize) -> Self {
// //         LexerError::new(
// //             LexerErrorKind::InvalidToken(t.to_string()), // Conversion de &str en String
// //             format!("Invalid token: {}", t),
// //             line,
// //             column,
// //         )
// //     }
// //
// //     pub fn unterminated_string(line: usize, column: usize) -> Self {
// //         LexerError::new(
// //             LexerErrorKind::UnterminatedString, // Aucun argument requis
// //             "Unterminated string".to_string(),
// //             line,
// //             column,
// //         )
// //     }
// //
// //     pub fn unterminated_comment(line: usize, column: usize) -> Self {
// //         LexerError::new(
// //             LexerErrorKind::UnterminatedComment, // Aucun argument requis
// //             "Unterminated comment".to_string(),
// //             line,
// //             column,
// //         )
// //     }
// //
// //     pub fn invalid_float(value: String, line: usize, column: usize) -> Self {
// //         LexerError::new(
// //             LexerErrorKind::InvalidToken(value.clone()), // Utilisation correcte de InvalidToken
// //             format!("Invalid float: {}", value),
// //             line,
// //             column,
// //         )
// //     }
// // }
// //
// //
// // impl TokenType {
// //     pub fn as_str(&self) -> &'static str {
// //         match self {
// //             TokenType::ENDER(Endmarker::ENDMARKER) => "ENDMARKER",
// //             TokenType::ENDER(Endmarker::EOF) => "EOF",
// //             TokenType::NEWLINE => "NEWLINE",
// //             TokenType::INDENT => "INDENT",
// //             TokenType::DEDENT => "DEDENT",
// //             TokenType::BOOLEAN => "BOOLEAN",
// //             TokenType::KEYWORD(kw) => kw.as_str(),
// //             TokenType::OPERATOR(op) => op.as_str(),
// //             TokenType::DELIMITERS(del) => del.as_str(),
// //             TokenType::LITERALS(_) => "LITERAL",
// //             TokenType::COMMENT => "COMMENT",
// //             TokenType::NL => "NL",
// //         }
// //     }
// // }
// // // Exemple pour Keywords
// // impl Keywords {
// //     pub fn as_str(&self) -> &'static str {
// //         match self {
// //             Keywords::AND => "and",
// //             Keywords::AS => "as",
// //             Keywords::ASYNC => "async",
// //             Keywords::AWAIT => "await",
// //             Keywords::BREAK => "break",
// //             Keywords::CLASS => "class",
// //             Keywords::CONTINUE => "continue",
// //             Keywords::DEF => "def",
// //             Keywords::DEL => "del",
// //             Keywords::ELIF => "elif",
// //             Keywords::ELSE => "else",
// //             Keywords::ENUM => "enum",
// //             Keywords::EXCEPT => "except",
// //             Keywords::FALSE => "false",
// //             Keywords::FN => "fn",
// //             Keywords::FOR => "for",
// //             Keywords::FROM => "from",
// //             Keywords::IF => "if",
// //             Keywords::IMPL => "impl",
// //             Keywords::IMPORT => "import",
// //             Keywords::IN => "in",
// //             Keywords::IS => "is",
// //             Keywords::LAMBDA => "lambda",
// //             Keywords::LET => "let",
// //             Keywords::LOOP => "loop",
// //             Keywords::MATCH => "match",
// //             Keywords::MOD => "mod",
// //             Keywords::MUT => "mut",
// //             Keywords::NONE => "none",
// //             Keywords::NOT => "not",
// //             Keywords::OR => "or",
// //             Keywords::PUB => "pub",
// //             Keywords::PASS => "pass",
// //             Keywords::RAISE => "raise",
// //             Keywords::RETURN => "return",
// //             Keywords::SELF => "self",
// //             Keywords::STATIC => "static",
// //             Keywords::STRUCT => "struct",
// //             Keywords::SUPER => "super",
// //             Keywords::TRUE => "true",
// //             Keywords::TRY => "try",
// //             Keywords::TYPE => "type",
// //             Keywords::TYPEOF => "typeof",
// //             Keywords::USE => "use",
// //             Keywords::WITH => "with",
// //             Keywords::WHILE => "while",
// //             Keywords::YIELD => "yield",
// //             // Ajoutez tous les autres cas ici...
// //             _ => unimplemented!(),
// //         }
// //     }
// // }
// //
// // // Exemple pour Operator
// // impl Operator {
// //     pub fn as_str(&self) -> &'static str {
// //         match self {
// //             Operator::PLUS => "+",
// //             Operator::MINUS => "-",
// //             Operator::STAR => "*",
// //             Operator::SLASH => "/",
// //             Operator::EQEQUAL => "==",
// //             Operator::NOTEQUAL => "!=",
// //             Operator::LESSEQUAL => "<=",
// //             Operator::FATARROW => "=>",
// //             Operator::GREATEREQUAL => ">=",
// //             Operator::TILDE => "~",
// //             Operator::CIRCUMFLEX => "^",
// //             Operator::LEFTSHIFT => "<<",
// //             Operator::RIGHTSHIFT => ">>",
// //             Operator::DOUBLESTAR => "**",
// //             Operator::PLUSEQUAL => "+=",
// //             Operator::MINEQUAL => "-=",
// //             Operator::STAREQUAL => "*=",
// //             Operator::SLASHEQUAL => "/=",
// //             Operator::PERCENTEQUAL => "%=",
// //             Operator::AMPEREQUAL => "&=",
// //             Operator::VBAREQUAL => "|=",
// //             Operator::CIRCUMFLEXEQUAL => "^=",
// //             Operator::LEFTSHIFTEQUAL => "<<=",
// //             Operator::RIGHTSHIFTEQUAL => ">>=",
// //             Operator::DOUBLESTAREQUAL => "**=",
// //             Operator::DOUBLESLASH => "//",
// //             Operator::DOUBLESLASHEQUAL => "//=",
// //             Operator::AT => "@",
// //             Operator::ATEQUAL => "@=",
// //             Operator::RARROW => "->",
// //             Operator::ELLIPSIS => "...",
// //             Operator::COLONEQUAL => ":=",
// //             Operator::STARSLASH => "*/",
// //             Operator::SLASHSTAR => "/*",
// //             Operator::DIESE => "#",
// //             Operator::EXCLAMATION => "!",
// //             Operator::INTERROGATION => "?",
// //             // Ajoutez tous les autres cas ici...
// //             _ => unimplemented!(),
// //         }
// //     }
// // }
// //
// // // Exemple pour Delimiter
// // impl Delimiter {
// //     pub fn as_str(&self) -> &'static str {
// //         match self {
// //             Delimiter::LPAR => "(",
// //             Delimiter::RPAR => ")",
// //             Delimiter::LCURBRACE => "{",
// //             Delimiter::RCURBRACE => "}",
// //             Delimiter::LSQB => "[",
// //             Delimiter::RSQB => "]",
// //             Delimiter::COLON => ":",
// //             Delimiter::COMMA => ",",
// //             Delimiter::SEMICOLON => ";",
// //             Delimiter::DOT => ".",
// //             Delimiter::DCOLON => "::",
// //             Delimiter::ELLIPSIS => "...",
// //             // Ajoutez tous les autres cas ici...
// //             //_ => unimplemented!(),
// //         }
// //     }
// // }
// //
