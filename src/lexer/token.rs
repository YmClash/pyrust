use std::fmt;
use std::fmt::Formatter;



/// PYRUST TOKEN TYPE

///  represente les differents type de token



#[derive(Debug, PartialEq,Eq, Clone)]
pub enum TokenType{
    //END MARKERS
    ENDER(Endmarker),
    //Special Token
    NEWLINE,
    INDENT,
    DEDENT,
    BOOLEAN,

    KEYWORD(Keywords),
    OPERATOR(Operator),
    DELIMITERS(Delimiter),
    LITERALS(Literal),

    COMMENT,     //      COMMENTAIRE   / COMMENT
    NL, //      NL        NOUVELLE LIGNE  / NEW LINE
}

/// Represente les differents type de   mot cle du langage

#[derive(Debug, PartialEq,Eq, Clone)]
pub enum Keywords{
        AND, AS, ASYNC, AWAIT, BREAK, CONST, CLASS, CONTINUE, DEF, DEL, ELIF, ELSE,
        ENUM, EXCEPT, FALSE, FN, FOR, FROM, IF, IMPL, IMPORT, IN, IS, LAMBDA, LET,
        LOOP, MATCH, MOD, MUT, NONE, NOT, OR, PUB, PASS, RAISE, RETURN, SELF,
        STATIC, STRUCT, SUPER, TRUE, TRY, TYPE, TYPEOF, USE, WITH, WHILE, YIELD,
}

#[derive(Debug, PartialEq,Eq, Clone)]
pub enum Operator {
    //OPERATORS
    PLUS, //           '+'          PLUS   / PLUS SIGN
    MINUS, //          '-'          MOINS   / MINUS SIGN
    STAR, //           '*'          ETOILE   / STAR
    SLASH, //          '/'          SLASH   / SLASH
    VBAR, //           '|'          BARRE VERTICALE  / VERTICAL BAR
    AMPER, //          '&'          ET COMMERCIAL  / AMPERSAND
    LESS, //           '<'          INFERIEUR   / LESS-THAN SIGN
    GREATER, //        '>'          SUPERIEUR   / GREATER-THAN SIGN
    EQUAL, //          '='          EGAL   / EQUALS SIGN
    PERCENT, //        '%'          POURCENTAGE   / PERCENT
    EQEQUAL, //        '=='         EGAL EGAL   / EQUALS EQUALS
    NOTEQUAL, //       '!='         DIFFERENT   / NOT EQUAL
    LESSEQUAL, //      '<='         INFERIEUR EGAL  / LESS-THAN EQUAL
    FATARROW, //       '=>'         IMPLIQUE   / IMPLIES ****
    GREATEREQUAL, //   '>='         SUPERIEUR EGAL  / GREATER-THAN EQUAL
    TILDE, //          '~'          TILDE   / TILDE
    CIRCUMFLEX, //     '^'          CIRCONFLEXE   / CIRCUMFLEX
    LEFTSHIFT, //      '<<'         DECALAGE GAUCHE  / LEFT SHIFT
    RIGHTSHIFT, //     '>>'         DECALAGE DROITE  / RIGHT SHIFT
    DOUBLESTAR, //     '**'         DOUBLE ÉTOILE  / DOUBLE STAR
    PLUSEQUAL, //      '+='         PLUS EGAL   / PLUS EQUAL
    MINEQUAL, //       '-='         MOINS EGAL   / MINUS EQUAL
    STAREQUAL, //      '*='         ETOILE EGAL  / STAR EQUAL
    SLASHEQUAL, //     '/='         SLASH EGAL   / SLASH EQUAL
    PERCENTEQUAL, //   '%='         POURCENTAGE EGAL  / PERCENT EQUAL
    AMPEREQUAL, //     '&='         ET COMMERCIAL EGAL  / AMPERSAND EQUAL
    VBAREQUAL, //       '|='        BARRE VERTICALE EGAL  / VERTICAL BAR EQUAL
    CIRCUMFLEXEQUAL, // '^='        CIRCONFLEXE EGAL  / CIRCUMFLEX EQUAL
    LEFTSHIFTEQUAL, //  '<<='       DECALAGE GAUCHE EGAL  / LEFT SHIFT EQUAL
    RIGHTSHIFTEQUAL, // '>>='       DECALAGE DROITE EGAL  / RIGHT SHIFT EQUAL
    DOUBLESTAREQUAL, // '**='       DOUBLE ETOILE EGAL  / DOUBLE STAR EQUAL
    DOUBLESLASH, //     '//'        DOUBLE SLASH  / DOUBLE SLASH
    DOUBLESLASHEQUAL, //'//='       DOUBLE SLASH EGAL  / DOUBLE SLASH EQUAL
    AT, //              '@'         AROBASE   / AT
    ATEQUAL, //         '@='        AROBASE EGAL  / AT EQUAL
    RARROW, //          '->'        FLECHE DROITE  / RIGHT ARROW
    ELLIPSIS, //        '...'       POINTS DE SUSPENSION  / ELLIPSIS      **COMMENT
    COLONEQUAL, //      ':='        DEUX POINT EGAL  / COLON EQUAL
    DCOLON, //          '::'        DEUX POINT DEUX POINT  / DOUBLE COLON
    STARSLASH, //       '*/'        ETOILE SLASH  / STAR SLASH           **COMMENT
    SLASHSTAR, //       '/*'        SLASH ETOILE  / SLASH STAR           **COMMENT
    DIESE, //           '#'         DIESE   / HASH ///                    **COMMENT


}

#[derive(Debug, PartialEq,Eq, Clone)]
pub enum Delimiter {
    LPAR, //        '('         PARANTHESE GAUCHE / LEFT PARENTHESIS
    RPAR, //        ')'         PARANTHESE DROITE / RIGHT PARENTHESIS
    LSQB, //        '['         CROCHET GAUCHE / LEFT SQUARE BRACKET
    RSQB, //        ']'         CROCHET DROIT / RIGHT SQUARE BRACKET
    COLON, //       ':'         DEUX POINT / COLON
    COMMA, //       ','         VIRGULE / COMMA
    SEMICOLON, //   ';'         POINT VIRGULE / SEMICOLON
    DOT, //         '.'         POINT / DOT
    LCURBRACE, //   '{'         ACCOLADE GAUCHE / LEFT CURLY BRACKET
    RCURBRACE, //   '}'         ACCOLADE DROITE / RIGHT CURLY BRACKET
    //AT, //          '@'         AROBASE / AT
    ELLIPSIS, //    '...'       POINTS DE SUSPENSION / ELLIPSIS
    DCOLON, //      '::'        DEUX POINT DEUX POINT / DOUBLE COLON
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Literal {
    IDENTIFIER(String),
    NUMBER(i64),
    STRING(String),
    INTEGER(i64),
    FLOAT(Float),
    //IMAGINARY(complex::Complex<f32>),
    HEXNUMBER(String),
    BOOLEAN(bool),
    CHAR(char),
}

#[derive(Debug, PartialEq,Eq, Clone)]
pub enum Endmarker {
    ENDMARKER,
    EOF,
}


// Utiliser un type personnalisé pour représenter les nombres à virgule flottante qui implémente Eq

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Float(String);

impl Float {
    // Méthode pour créer un nouveau Float
    pub fn new<T: ToString>(value: T) -> Result<Self, ParseFloatError> {
        let s = value.to_string();
        // Vérifier si la chaîne est un nombre flottant valide
        s.parse::<f64>()?;
        Ok(Float(s))
    }

    // Méthode pour obtenir la valeur en tant que f64
    pub fn as_f64(&self) -> f64 {
        self.0.parse().unwrap()
    }
}

// Implémentation de Display pour l'affichage
impl fmt::Display for Float {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Implémentation de PartialOrd et Ord si nécessaire
impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_f64().partial_cmp(&other.as_f64())
    }
}

impl Ord for Float {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

// Une enum personnalisée pour les erreurs de parsing
#[derive(Debug)]
pub enum ParseFloatError {
    InvalidFloat,
}

impl From<std::num::ParseFloatError> for ParseFloatError {
    fn from(_: std::num::ParseFloatError) -> Self {
        ParseFloatError::InvalidFloat
    }
}

/////////////////IMPLEMENTATION DES METHODES POUR LE TYPE DE TOKEN////////////////////
impl TokenType {
    /// verifie sie le token est un mot cle
    pub fn is_keyword(&self) -> bool {
        matches!(self,TokenType::KEYWORD(_))
    }
    /// verifie si le token est un operateur
    pub fn is_operator(&self) ->bool{
        matches!(self,TokenType::OPERATOR(_))
    }

    /// verifie si le token est un literal
    pub fn is_literal(&self) -> bool {
        matches!(self,TokenType::LITERALS(_))
    }
    /// verifie si le token est un delimitateur
    pub fn is_delimiter(&self) -> bool {
        matches!(self, TokenType::DELIMITERS(_))
    }
    //  vérifier si un token est un marqueur de fin
    pub fn is_end_marker(&self) -> bool {
        matches!(self, TokenType::ENDER(_))
    }

}

// implementation  die Display pour TokenType

impl fmt::Display for TokenType{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::KEYWORD(kw) => write!(f,"{:?}",kw),
            TokenType::OPERATOR(op) => write!(f,"{:?}",op),
            TokenType::DELIMITERS(del) => write!(f,"{:?}",del),
            TokenType::LITERALS(lit) => write!(f,"{:?}",lit),
            TokenType::ENDER(end) => write!(f,"{:?}",end),
            TokenType::NEWLINE => write!(f,"NEWLINE"),
            TokenType::INDENT => write!(f,"INDENT"),
            TokenType::DEDENT => write!(f,"DEDENT"),
            TokenType::BOOLEAN => write!(f,"BOOLEAN"),
            TokenType::COMMENT => write!(f,"COMMENT"),
            TokenType::NL => write!(f,"NL"),
        }
    }
}

//implementatnion  de  From<String> pour  TokenType


// Implementation de From<String> pour TokenType
impl From<String> for TokenType {
    fn from(s: String) -> Self {
        match s.as_str() {
            // Opérateurs
            "+" => TokenType::OPERATOR(Operator::PLUS),
            "-" => TokenType::OPERATOR(Operator::MINUS),
            "*" => TokenType::OPERATOR(Operator::STAR),
            "/" => TokenType::OPERATOR(Operator::SLASH),
            "|" => TokenType::OPERATOR(Operator::VBAR),
            "&" => TokenType::OPERATOR(Operator::AMPER),
            "<" => TokenType::OPERATOR(Operator::LESS),
            ">" => TokenType::OPERATOR(Operator::GREATER),
            "=" => TokenType::OPERATOR(Operator::EQUAL),
            "%" => TokenType::OPERATOR(Operator::PERCENT),
            "==" => TokenType::OPERATOR(Operator::EQEQUAL),
            "!=" => TokenType::OPERATOR(Operator::NOTEQUAL),
            "<=" => TokenType::OPERATOR(Operator::LESSEQUAL),
            "=>" => TokenType::OPERATOR(Operator::FATARROW),
            ">=" => TokenType::OPERATOR(Operator::GREATEREQUAL),
            "~" => TokenType::OPERATOR(Operator::TILDE),
            "^" => TokenType::OPERATOR(Operator::CIRCUMFLEX),
            "<<" => TokenType::OPERATOR(Operator::LEFTSHIFT),
            ">>" => TokenType::OPERATOR(Operator::RIGHTSHIFT),
            "**" => TokenType::OPERATOR(Operator::DOUBLESTAR),
            "+=" => TokenType::OPERATOR(Operator::PLUSEQUAL),
            "-=" => TokenType::OPERATOR(Operator::MINEQUAL),
            "*=" => TokenType::OPERATOR(Operator::STAREQUAL),
            "/=" => TokenType::OPERATOR(Operator::SLASHEQUAL),
            "%=" => TokenType::OPERATOR(Operator::PERCENTEQUAL),
            "&=" => TokenType::OPERATOR(Operator::AMPEREQUAL),
            "|=" => TokenType::OPERATOR(Operator::VBAREQUAL),
            "^=" => TokenType::OPERATOR(Operator::CIRCUMFLEXEQUAL),
            "<<=" => TokenType::OPERATOR(Operator::LEFTSHIFTEQUAL),
            ">>=" => TokenType::OPERATOR(Operator::RIGHTSHIFTEQUAL),
            "**=" => TokenType::OPERATOR(Operator::DOUBLESTAREQUAL),
            "//" => TokenType::OPERATOR(Operator::DOUBLESLASH),
            "//=" => TokenType::OPERATOR(Operator::DOUBLESLASHEQUAL),
            "@" => TokenType::OPERATOR(Operator::AT),
            "@=" => TokenType::OPERATOR(Operator::ATEQUAL),
            "->" => TokenType::OPERATOR(Operator::RARROW),
            ":=" => TokenType::OPERATOR(Operator::COLONEQUAL),
            "::" => TokenType::OPERATOR(Operator::DCOLON),
            "*/" => TokenType::OPERATOR(Operator::STARSLASH),
            "/*" => TokenType::OPERATOR(Operator::SLASHSTAR),
            "#" => TokenType::OPERATOR(Operator::DIESE),

            // Délimiteurs
            "(" => TokenType::DELIMITERS(Delimiter::LPAR),
            ")" => TokenType::DELIMITERS(Delimiter::RPAR),
            "[" => TokenType::DELIMITERS(Delimiter::LSQB),
            "]" => TokenType::DELIMITERS(Delimiter::RSQB),
            ":" => TokenType::DELIMITERS(Delimiter::COLON),
            "," => TokenType::DELIMITERS(Delimiter::COMMA),
            ";" => TokenType::DELIMITERS(Delimiter::SEMICOLON),
            "." => TokenType::DELIMITERS(Delimiter::DOT),
            "{" => TokenType::DELIMITERS(Delimiter::LCURBRACE),
            "}" => TokenType::DELIMITERS(Delimiter::RCURBRACE),
            "..." => TokenType::DELIMITERS(Delimiter::ELLIPSIS),

            // Mots-clés
            "and" => TokenType::KEYWORD(Keywords::AND),
            "as" => TokenType::KEYWORD(Keywords::AS),
            "async" => TokenType::KEYWORD(Keywords::ASYNC),
            "await" => TokenType::KEYWORD(Keywords::AWAIT),
            "break" => TokenType::KEYWORD(Keywords::BREAK),
            "const" => TokenType::KEYWORD(Keywords::CONST),
            "class" => TokenType::KEYWORD(Keywords::CLASS),
            "continue" => TokenType::KEYWORD(Keywords::CONTINUE),
            "def" => TokenType::KEYWORD(Keywords::DEF),
            "del" => TokenType::KEYWORD(Keywords::DEL),
            "elif" => TokenType::KEYWORD(Keywords::ELIF),
            "else" => TokenType::KEYWORD(Keywords::ELSE),
            "enum" => TokenType::KEYWORD(Keywords::ENUM),
            "except" => TokenType::KEYWORD(Keywords::EXCEPT),
            "false" | "False" | "FALSE" => TokenType::KEYWORD(Keywords::FALSE),
            "fn" => TokenType::KEYWORD(Keywords::FN),
            "for" => TokenType::KEYWORD(Keywords::FOR),
            "from" => TokenType::KEYWORD(Keywords::FROM),
            "if" => TokenType::KEYWORD(Keywords::IF),
            "impl" => TokenType::KEYWORD(Keywords::IMPL),
            "import" => TokenType::KEYWORD(Keywords::IMPORT),
            "in" => TokenType::KEYWORD(Keywords::IN),
            "is" => TokenType::KEYWORD(Keywords::IS),
            "lambda" => TokenType::KEYWORD(Keywords::LAMBDA),
            "let" => TokenType::KEYWORD(Keywords::LET),
            "loop" => TokenType::KEYWORD(Keywords::LOOP),
            "match" => TokenType::KEYWORD(Keywords::MATCH),
            "mod" => TokenType::KEYWORD(Keywords::MOD),
            "mut" => TokenType::KEYWORD(Keywords::MUT),
            "none" | "None" | "NONE" => TokenType::KEYWORD(Keywords::NONE),
            "not" => TokenType::KEYWORD(Keywords::NOT),
            "or" => TokenType::KEYWORD(Keywords::OR),
            "pub" => TokenType::KEYWORD(Keywords::PUB),
            "pass" => TokenType::KEYWORD(Keywords::PASS),
            "raise" => TokenType::KEYWORD(Keywords::RAISE),
            "return" => TokenType::KEYWORD(Keywords::RETURN),
            "self" => TokenType::KEYWORD(Keywords::SELF),
            "static" => TokenType::KEYWORD(Keywords::STATIC),
            "struct" => TokenType::KEYWORD(Keywords::STRUCT),
            "super" => TokenType::KEYWORD(Keywords::SUPER),
            "true" | "True" | "TRUE" => TokenType::KEYWORD(Keywords::TRUE),
            "try" => TokenType::KEYWORD(Keywords::TRY),
            "type" => TokenType::KEYWORD(Keywords::TYPE),
            "typeof" => TokenType::KEYWORD(Keywords::TYPEOF),
            "use" => TokenType::KEYWORD(Keywords::USE),
            "with" => TokenType::KEYWORD(Keywords::WITH),
            "while" => TokenType::KEYWORD(Keywords::WHILE),
            "yield" => TokenType::KEYWORD(Keywords::YIELD),

            // Littéraux
            s if s.starts_with('"') && s.ends_with('"') => TokenType::LITERALS(Literal::STRING(s[1..s.len()-1].to_string())),
            s if s.parse::<i64>().is_ok() => TokenType::LITERALS(Literal::INTEGER(s.parse().unwrap())),
            s if s.starts_with("0x") => TokenType::LITERALS(Literal::HEXNUMBER(s.to_string())),
            s if s.parse::<f64>().is_ok() => TokenType::LITERALS(Literal::FLOAT(Float(s.to_string()))),
            s if s.starts_with('\'') && s.ends_with('\'') && s.len() == 3 =>
                TokenType::LITERALS(Literal::CHAR(s.chars().nth(1).unwrap())),
            // "true" | "True" | "TRUE" => TokenType::LITERALS(Literal::BOOLEAN(true)),
            // "false" | "False" | "FALSE" => TokenType::LITERALS(Literal::BOOLEAN(false)),
            //

            _ => TokenType::LITERALS(Literal::IDENTIFIER(s)), // Par défaut, considérez-le comme un identifiant
        }
    }
}

//
//
//
//
// rustkeywords = ["as", "break", "const", "continue", "crate","dyn", "else", "enum", "extern", "false", "fn",
// "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
// "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
// "use", "where", "while", "async", "await", "dyn",]
//
//
// pythonkeywords = sorted(['False', 'None', 'True', 'and', 'as', 'assert', 'async', 'await', 'break', 'class',
// 'continue', 'def', 'del', 'elif', 'else', 'except', 'finally', 'for', 'from', 'global',
// 'if', 'import', 'in', 'is', 'lambda', 'nonlocal', 'not', 'or', 'pass', 'raise', 'return',
// 'try', 'while', 'with', 'yield'])