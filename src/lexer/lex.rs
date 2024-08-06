//mod lex;

use std::str::Chars;
use std::iter::Peekable;
use std::process::exit;
use std::collections::HashMap;


///  represente les differents type de token
#[derive(Debug, PartialEq,Eq, Clone)]
pub enum TokenType{
    ENDMARKER,
    EOF,
    NAME,
    NUMBER,
    STRING,
    NEWLINE,
    INDENT,
    DEDENT,

    //OPERATORS
    LPAR, //    '('       PARANTHESE GAUCHE   / LEFT PARENTHESIS
    RPAR, //    ')'       PARANTHESE DROITE  / RIGHT PARENTHESIS
    LSQB, //    '['       CROCHET GAUCHE    / LEFT SQUARE BRACKET
    RSQB, //    ']'       CROCHET DROIT    / RIGHT SQUARE BRACKET
    COLON, //   ':'      DEUX POINT   / COLON
    COMMA, //   ','        VIRGULE   / COMMA
    SEMI, //    ';'       POINT VIRGULE  / SEMICOLON
    PLUS, //    '+'       PLUS   / PLUS SIGN
    MINUS, //   '-'      MOINS   / MINUS SIGN
    STAR, //    '*'       ETOILE   / STAR
    SLASH, //   '/'      SLASH   / SLASH
    VBAR, //    '|'       BARRE VERTICALE  / VERTICAL BAR
    AMPER, //   '&'      ET COMMERCIAL  / AMPERSAND
    LESS, //    '<'       INFERIEUR   / LESS-THAN SIGN
    GREATER, // '>'       SUPERIEUR   / GREATER-THAN SIGN
    EQUAL, //   '='       EGAL   / EQUALS SIGN
    DOT, //     '.'       POINT   / DOT
    PERCENT, // '%'       POURCENTAGE   / PERCENT
    LBRACE, //  '{'       ACCOLADE GAUCHE  / LEFT CURLY BRACKET
    RBRACE, //  '}'       ACCOLADE DROITE  / RIGHT CURLY BRACKET
    EQEQUAL, // '=='      EGAL EGAL   / EQUALS EQUALS
    NOTEQUAL, // '!='     DIFFERENT   / NOT EQUAL
    LESSEQUAL, // '<='     INFERIEUR EGAL  / LESS-THAN EQUAL
    IMPLIES, // '=>'     IMPLIQUE   / IMPLIES ****
    GREATEREQUAL, // '>='     SUPERIEUR EGAL  / GREATER-THAN EQUAL
    TILDE, //   '~'       TILDE   / TILDE
    CIRCUMFLEX, // '^'       CIRCONFLEXE   / CIRCUMFLEX
    LEFTSHIFT, // '<<'     DECALAGE GAUCHE  / LEFT SHIFT
    RIGHTSHIFT, // '>>'     DECALAGE DROITE  / RIGHT SHIFT
    DOUBLESTAR, // '**'     DOUBLE ETOILE  / DOUBLE STAR
    PLUSEQUAL, // '+='     PLUS EGAL   / PLUS EQUAL
    MINEQUAL, // '-='     MOINS EGAL   / MINUS EQUAL
    STAREQUAL, // '*='     ETOILE EGAL  / STAR EQUAL
    SLASHEQUAL, // '/='     SLASH EGAL   / SLASH EQUAL
    PERCENTEQUAL, // '%='     POURCENTAGE EGAL  / PERCENT EQUAL
    AMPEREQUAL, // '&='     ET COMMERCIAL EGAL  / AMPERSAND EQUAL
    VBAREQUAL, // '|='     BARRE VERTICALE EGAL  / VERTICAL BAR EQUAL
    CIRCUMFLEXEQUAL, // '^='     CIRCONFLEXE EGAL  / CIRCUMFLEX EQUAL
    LEFTSHIFTEQUAL, // '<<='    DECALAGE GAUCHE EGAL  / LEFT SHIFT EQUAL
    RIGHTSHIFTEQUAL, // '>>='    DECALAGE DROITE EGAL  / RIGHT SHIFT EQUAL
    DOUBLESTAREQUAL, // '**='    DOUBLE ETOILE EGAL  / DOUBLE STAR EQUAL
    DOUBLESLASH, // '//'     DOUBLE SLASH  / DOUBLE SLASH
    DOUBLESLASHEQUAL, // '//='    DOUBLE SLASH EGAL  / DOUBLE SLASH EQUAL
    AT, //      '@'       AROBASE   / AT
    ATEQUAL, // '@='     AROBASE EGAL  / AT EQUAL
    RARROW, // '->'     FLECHE DROITE  / RIGHT ARROW
    ELLIPSIS, // '...'    POINTS DE SUSPENSION  / ELLIPSIS      **COMMENT
    COLONEQUAL, // ':='     DEUX POINT EGAL  / COLON EQUAL
    DCOLON, // '::'     DEUX POINT DEUX POINT  / DOUBLE COLON
    STARSLASH, // '*/'     ETOILE SLASH  / STAR SLASH           **COMMENT
    SLASHSTAR, // '/*'     SLASH ETOILE  / SLASH STAR           **COMMENT
    DIESE, //   '#'       DIESE   / HASH ///                    **COMMENT

    OP, //      OP        OPERATEUR   / OPERATOR
    TYPEIGNORE,
    COMMENT,     //      COMMENTAIRE   / COMMENT
    NL, //      NL        NOUVELLE LIGNE  / NEW LINE








}
