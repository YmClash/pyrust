
/// PYRUST TOKEN TYPE

#[derive(Debug, PartialEq,Eq, Clone)]

///  represente les differents type de token


#[derive(Debug, PartialEq,Eq, Clone)]
pub enum TokenType{
    ENDMARKER,
    EOF,
    NAME,
    NUMBER,
    STRING,
    INTERGER,
    FLOAT,
    IMAGINARY,
    HEXNUMBER,

    NEWLINE,
    INDENT,
    DEDENT,
    BOOLEAN,



    //OPERATORS
    LPAR,       //          '('       PARANTHESE GAUCHE   / LEFT PARENTHESIS
    RPAR, //           ')'       PARANTHESE DROITE  / RIGHT PARENTHESIS
    LSQB, //           '['       CROCHET GAUCHE    / LEFT SQUARE BRACKET
    RSQB, //           ']'       CROCHET DROIT    / RIGHT SQUARE BRACKET
    COLON, //          ':'      DEUX POINT   / COLON
    COMMA, //          ','        VIRGULE   / COMMA
    SEMI, //           ';'       POINT VIRGULE  / SEMICOLON
    PLUS, //           '+'       PLUS   / PLUS SIGN
    MINUS, //          '-'      MOINS   / MINUS SIGN
    STAR, //           '*'       ETOILE   / STAR
    SLASH, //          '/'      SLASH   / SLASH
    VBAR, //           '|'       BARRE VERTICALE  / VERTICAL BAR
    AMPER, //          '&'      ET COMMERCIAL  / AMPERSAND
    LESS, //           '<'       INFERIEUR   / LESS-THAN SIGN
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

    //OP, //      OP        OPERATEUR   / OPERATOR
    TYPEIGNORE,
    COMMENT,     //      COMMENTAIRE   / COMMENT
    NL, //      NL        NOUVELLE LIGNE  / NEW LINE







}

#[derive(Debug, PartialEq,Eq, Clone)]
pub enum Keywords{
    AND,
    AS,
    ASYNC,
    AWAIT,
    BREAK,
    CONST,
    CLASS,
    CONTINUE,
    DEF,
    DEL,
    ELIF,
    ELSE,
    ENUM,
    EXCEPT,
    FALSE,
    FN,
    FOR,
    FROM,
    IF,
    IMPL,
    IMPORT,
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
    PUB,
    PASS,
    RAISE,
    RETURN,
    SELF,
    STATIC,
    STRUCT,
    SUPER,
    TRUE,
    TRY,
    TYPE,
    TYPEOF,
    USE,
    WITH,
    WHILE,
    YIELD,
}







rustkeywords = ["as", "break", "const", "continue", "crate","dyn", "else", "enum", "extern", "false", "fn",
"for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
"return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
"use", "where", "while", "async", "await", "dyn",]


pythonkeywords = sorted(['False', 'None', 'True', 'and', 'as', 'assert', 'async', 'await', 'break', 'class',
'continue', 'def', 'del', 'elif', 'else', 'except', 'finally', 'for', 'from', 'global',
'if', 'import', 'in', 'is', 'lambda', 'nonlocal', 'not', 'or', 'pass', 'raise', 'return',
'try', 'while', 'with', 'yield'])