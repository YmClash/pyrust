use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

/// PYRUST TOKEN TYPE
/// Représente les différents types de tokens

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    // END MARKERS
    ENDER(Endmarker),
    // Special Token
    NEWLINE,
    INDENT,
    DEDENT,
    BOOLEAN,

    KEYWORD(Keywords),
    OPERATOR(Operator),
    DELIMITERS(Delimiter),
    LITERALS(Literal),

    COMMENT,     // COMMENTAIRE / COMMENT
    NL, // NOUVELLE LIGNE / NEW LINE
}

/// Représente les différents types de mots-clés du langage

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Keywords {
    AND, AS, ASYNC, AWAIT, BREAK, CONST, CLASS, CONTINUE, DEF, DEL, ELIF, ELSE,
    ENUM, EXCEPT, FALSE, FN, FOR, FROM, IF, IMPL, IMPORT, IN, IS, LAMBDA, LET,
    LOOP, MATCH, MOD, MUT, NONE, NOT, OR, PUB, PASS, RAISE, RETURN, SELF,
    STATIC, STRUCT, SUPER, TRUE, TRY, TYPE, TYPEOF, USE, WITH, WHILE, YIELD,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operator {
    // OPERATORS
    PLUS, // '+' PLUS / PLUS SIGN
    MINUS, // '-' MOINS / MINUS SIGN
    STAR, // '*' ETOILE / STAR
    SLASH, // '/' SLASH / SLASH
    VBAR, // '|' BARRE VERTICALE / VERTICAL BAR
    AMPER, // '&' ET COMMERCIAL / AMPERSAND
    LESS, // '<' INFERIEUR / LESS-THAN SIGN
    GREATER, // '>' SUPERIEUR / GREATER-THAN SIGN
    EQUAL, // '=' EGAL / EQUALS SIGN
    PERCENT, // '%' POURCENTAGE / PERCENT
    EQEQUAL, // '==' EGAL EGAL / EQUALS EQUALS
    NOTEQUAL, // '!=' DIFFERENT / NOT EQUAL
    LESSEQUAL, // '<=' INFERIEUR EGAL / LESS-THAN EQUAL
    FATARROW, // '=>' IMPLIQUE / IMPLIES
    GREATEREQUAL, // '>=' SUPERIEUR EGAL / GREATER-THAN EQUAL
    TILDE, // '~' TILDE / TILDE
    CIRCUMFLEX, // '^' CIRCONFLEXE / CIRCUMFLEX
    LEFTSHIFT, // '<<' DECALAGE GAUCHE / LEFT SHIFT
    RIGHTSHIFT, // '>>' DECALAGE DROITE / RIGHT SHIFT
    DOUBLESTAR, // '**' DOUBLE ÉTOILE / DOUBLE STAR
    PLUSEQUAL, // '+=' PLUS EGAL / PLUS EQUAL
    MINEQUAL, // '-=' MOINS EGAL / MINUS EQUAL
    STAREQUAL, // '*=' ETOILE EGAL / STAR EQUAL
    SLASHEQUAL, // '/=' SLASH EGAL / SLASH EQUAL
    PERCENTEQUAL, // '%=' POURCENTAGE EGAL / PERCENT EQUAL
    AMPEREQUAL, // '&=' ET COMMERCIAL EGAL / AMPERSAND EQUAL
    VBAREQUAL, // '|=' BARRE VERTICALE EGAL / VERTICAL BAR EQUAL
    CIRCUMFLEXEQUAL, // '^=' CIRCONFLEXE EGAL / CIRCUMFLEX EQUAL
    LEFTSHIFTEQUAL, // '<<=' DECALAGE GAUCHE EGAL / LEFT SHIFT EQUAL
    RIGHTSHIFTEQUAL, // '>>=' DECALAGE DROITE EGAL / RIGHT SHIFT EQUAL
    DOUBLESTAREQUAL, // '**=' DOUBLE ETOILE EGAL / DOUBLE STAR EQUAL
    DOUBLESLASH, // '//' DOUBLE SLASH / DOUBLE SLASH
    DOUBLESLASHEQUAL, // '//=' DOUBLE SLASH EGAL / DOUBLE SLASH EQUAL
    AT, // '@' AROBASE / AT
    ATEQUAL, // '@=' AROBASE EGAL / AT EQUAL
    RARROW, // '->' FLECHE DROITE / RIGHT ARROW
    ELLIPSIS, // '...' POINTS DE SUSPENSION / ELLIPSIS
    COLONEQUAL, // ':=' DEUX POINT EGAL / COLON EQUAL
    STARSLASH, // '*/' ETOILE SLASH / STAR SLASH
    SLASHSTAR, // '/*' SLASH ETOILE / SLASH STAR
    DIESE, // '#' DIESE / HASH
    EXCLAMATION, // '!' POINT D'EXCLAMATION / EXCLAMATION POINT
    INTERROGATION, // '?' POINT D'INTERROGATION / QUESTION MARK
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Delimiter {
    LPAR, // '(' PARANTHESE GAUCHE / LEFT PARENTHESIS
    RPAR, // ')' PARANTHESE DROITE / RIGHT PARENTHESIS
    LSQB, // '[' CROCHET GAUCHE / LEFT SQUARE BRACKET
    RSQB, // ']' CROCHET DROIT / RIGHT SQUARE BRACKET
    COLON, // ':' DEUX POINT / COLON
    COMMA, // ',' VIRGULE / COMMA
    SEMICOLON, // ';' POINT VIRGULE / SEMICOLON
    DOT, // '.' POINT / DOT
    LCURBRACE, // '{' ACCOLADE GAUCHE / LEFT CURLY BRACKET
    RCURBRACE, // '}' ACCOLADE DROITE / RIGHT CURLY BRACKET
    ELLIPSIS, // '...' POINTS DE SUSPENSION / ELLIPSIS
    DCOLON, // '::' DEUX POINT DEUX POINT / DOUBLE COLON
}

// Utiliser un type personnalisé pour représenter les nombres à virgule flottante qui implémente Eq
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Float(String);

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Literal {
    IDENTIFIER(String),
    NUMBER(i64),
    STRING(String),
    INTEGER(i64),
    FLOAT(Float),
    HEXNUMBER(String),
    BOOLEAN(bool),
    CHAR(char),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Endmarker {
    ENDMARKER,
    EOF,
}
/////////////////////IMPLEMENTATION  DE  FLOAT////////////////////////////////////////
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
///////////////// IMPLEMENTATION DES METHODES POUR LE TYPE DE TOKEN ////////////////////
impl TokenType {
    /// Vérifie si le token est un mot-clé
    pub fn is_keyword(&self) -> bool {
        matches!(self, TokenType::KEYWORD(_))
    }

    /// Vérifie si le token est un opérateur
    pub fn is_operator(&self) -> bool {
        matches!(self, TokenType::OPERATOR(_))
    }

    /// Vérifie si le token est un littéral
    pub fn is_literal(&self) -> bool {
        matches!(self, TokenType::LITERALS(_))
    }

    /// Vérifie si le token est un délimiteur
    pub fn is_delimiter(&self) -> bool {
        matches!(self, TokenType::DELIMITERS(_))
    }

    /// Vérifie si le token est un marqueur de fin
    pub fn is_end_marker(&self) -> bool {
        matches!(self, TokenType::ENDER(_))
    }
}



// Implementation de Display pour TokenType
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::KEYWORD(kw) => write!(f, "{:?}", kw),
            TokenType::OPERATOR(op) => write!(f, "{:?}", op),
            TokenType::DELIMITERS(del) => write!(f, "{:?}", del),
            TokenType::LITERALS(lit) => write!(f, "{:?}", lit),
            TokenType::ENDER(end) => write!(f, "{:?}", end),
            TokenType::NEWLINE => write!(f, "NEWLINE"),
            TokenType::INDENT => write!(f, "INDENT"),
            TokenType::DEDENT => write!(f, "DEDENT"),
            TokenType::BOOLEAN => write!(f, "BOOLEAN"),
            TokenType::COMMENT => write!(f, "COMMENT"),
            TokenType::NL => write!(f, "NL"),
        }
    }
}

// Implementation de From<String> pour TokenType
impl FromStr for TokenType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Opérateurs
            "+" => Ok(TokenType::OPERATOR(Operator::PLUS)),
            "-" => Ok(TokenType::OPERATOR(Operator::MINUS)),
            "*" => Ok(TokenType::OPERATOR(Operator::STAR)),
            "/" => Ok(TokenType::OPERATOR(Operator::SLASH)),
            "|" => Ok(TokenType::OPERATOR(Operator::VBAR)),
            "&" => Ok(TokenType::OPERATOR(Operator::AMPER)),
            "<" => Ok(TokenType::OPERATOR(Operator::LESS)),
            ">" => Ok(TokenType::OPERATOR(Operator::GREATER)),
            "=" => Ok(TokenType::OPERATOR(Operator::EQUAL)),
            "%" => Ok(TokenType::OPERATOR(Operator::PERCENT)),
            "==" => Ok(TokenType::OPERATOR(Operator::EQEQUAL)),
            "!=" => Ok(TokenType::OPERATOR(Operator::NOTEQUAL)),
            "<=" => Ok(TokenType::OPERATOR(Operator::LESSEQUAL)),
            "=>" => Ok(TokenType::OPERATOR(Operator::FATARROW)),
            ">=" => Ok(TokenType::OPERATOR(Operator::GREATEREQUAL)),
            "~" => Ok(TokenType::OPERATOR(Operator::TILDE)),
            "^" => Ok(TokenType::OPERATOR(Operator::CIRCUMFLEX)),
            "<<" => Ok(TokenType::OPERATOR(Operator::LEFTSHIFT)),
            ">>" => Ok(TokenType::OPERATOR(Operator::RIGHTSHIFT)),
            "**" => Ok(TokenType::OPERATOR(Operator::DOUBLESTAR)),
            "+=" => Ok(TokenType::OPERATOR(Operator::PLUSEQUAL)),
            "-=" => Ok(TokenType::OPERATOR(Operator::MINEQUAL)),
            "*=" => Ok(TokenType::OPERATOR(Operator::STAREQUAL)),
            "/=" => Ok(TokenType::OPERATOR(Operator::SLASHEQUAL)),
            "%=" => Ok(TokenType::OPERATOR(Operator::PERCENTEQUAL)),
            "&=" => Ok(TokenType::OPERATOR(Operator::AMPEREQUAL)),
            "|=" => Ok(TokenType::OPERATOR(Operator::VBAREQUAL)),
            "^=" => Ok(TokenType::OPERATOR(Operator::CIRCUMFLEXEQUAL)),
            "<<=" => Ok(TokenType::OPERATOR(Operator::LEFTSHIFTEQUAL)),
            ">>=" => Ok(TokenType::OPERATOR(Operator::RIGHTSHIFTEQUAL)),
            "**=" => Ok(TokenType::OPERATOR(Operator::DOUBLESTAREQUAL)),
            "//" => Ok(TokenType::OPERATOR(Operator::DOUBLESLASH)),
            "//=" => Ok(TokenType::OPERATOR(Operator::DOUBLESLASHEQUAL)),
            "@" => Ok(TokenType::OPERATOR(Operator::AT)),
            "@=" => Ok(TokenType::OPERATOR(Operator::ATEQUAL)),
            "->" => Ok(TokenType::OPERATOR(Operator::RARROW)),
            ":=" => Ok(TokenType::OPERATOR(Operator::COLONEQUAL)),
            // "::" => Ok(TokenType::OPERATOR(Operator::DCOLON)),
            "*/" => Ok(TokenType::OPERATOR(Operator::STARSLASH)),
            "/*" => Ok(TokenType::OPERATOR(Operator::SLASHSTAR)),
            "#" => Ok(TokenType::OPERATOR(Operator::DIESE)),

            // Délimiteurs
            "(" => Ok(TokenType::DELIMITERS(Delimiter::LPAR)),
            ")" => Ok(TokenType::DELIMITERS(Delimiter::RPAR)),
            "[" => Ok(TokenType::DELIMITERS(Delimiter::LSQB)),
            "]" => Ok(TokenType::DELIMITERS(Delimiter::RSQB)),
            ":" => Ok(TokenType::DELIMITERS(Delimiter::COLON)),
            "," => Ok(TokenType::DELIMITERS(Delimiter::COMMA)),
            ";" => Ok(TokenType::DELIMITERS(Delimiter::SEMICOLON)),
            "." => Ok(TokenType::DELIMITERS(Delimiter::DOT)),
            "::" => Ok(TokenType::DELIMITERS(Delimiter::DCOLON)),
            "{" => Ok(TokenType::DELIMITERS(Delimiter::LCURBRACE)),
            "}" => Ok(TokenType::DELIMITERS(Delimiter::RCURBRACE)),
            "..." => Ok(TokenType::DELIMITERS(Delimiter::ELLIPSIS)),

            // Mots-clés
            "and" => Ok(TokenType::KEYWORD(Keywords::AND)),
            "as" => Ok(TokenType::KEYWORD(Keywords::AS)),
            "async" => Ok(TokenType::KEYWORD(Keywords::ASYNC)),
            "await" => Ok(TokenType::KEYWORD(Keywords::AWAIT)),
            "break" => Ok(TokenType::KEYWORD(Keywords::BREAK)),
            "const" => Ok(TokenType::KEYWORD(Keywords::CONST)),
            "class" => Ok(TokenType::KEYWORD(Keywords::CLASS)),
            "continue" => Ok(TokenType::KEYWORD(Keywords::CONTINUE)),
            "def" => Ok(TokenType::KEYWORD(Keywords::DEF)),
            "del" => Ok(TokenType::KEYWORD(Keywords::DEL)),
            "elif" => Ok(TokenType::KEYWORD(Keywords::ELIF)),
            "else" => Ok(TokenType::KEYWORD(Keywords::ELSE)),
            "enum" => Ok(TokenType::KEYWORD(Keywords::ENUM)),
            "except" => Ok(TokenType::KEYWORD(Keywords::EXCEPT)),
            "false" | "False" | "FALSE" => Ok(TokenType::KEYWORD(Keywords::FALSE)),
            "fn" => Ok(TokenType::KEYWORD(Keywords::FN)),
            "for" => Ok(TokenType::KEYWORD(Keywords::FOR)),
            "from" => Ok(TokenType::KEYWORD(Keywords::FROM)),
            "if" => Ok(TokenType::KEYWORD(Keywords::IF)),
            "impl" => Ok(TokenType::KEYWORD(Keywords::IMPL)),
            "import" => Ok(TokenType::KEYWORD(Keywords::IMPORT)),
            "in" => Ok(TokenType::KEYWORD(Keywords::IN)),
            "is" => Ok(TokenType::KEYWORD(Keywords::IS)),
            "lambda" => Ok(TokenType::KEYWORD(Keywords::LAMBDA)),
            "let" => Ok(TokenType::KEYWORD(Keywords::LET)),
            "loop" => Ok(TokenType::KEYWORD(Keywords::LOOP)),
            "match" => Ok(TokenType::KEYWORD(Keywords::MATCH)),
            "mod" => Ok(TokenType::KEYWORD(Keywords::MOD)),
            "mut" => Ok(TokenType::KEYWORD(Keywords::MUT)),
            "none" | "None" | "NONE" => Ok(TokenType::KEYWORD(Keywords::NONE)),
            "not" => Ok(TokenType::KEYWORD(Keywords::NOT)),
            "or" => Ok(TokenType::KEYWORD(Keywords::OR)),
            "pub" => Ok(TokenType::KEYWORD(Keywords::PUB)),
            "pass" => Ok(TokenType::KEYWORD(Keywords::PASS)),
            "raise" => Ok(TokenType::KEYWORD(Keywords::RAISE)),
            "return" => Ok(TokenType::KEYWORD(Keywords::RETURN)),
            "self" => Ok(TokenType::KEYWORD(Keywords::SELF)),
            "static" => Ok(TokenType::KEYWORD(Keywords::STATIC)),
            "struct" => Ok(TokenType::KEYWORD(Keywords::STRUCT)),
            "super" => Ok(TokenType::KEYWORD(Keywords::SUPER)),
            "true" | "True" | "TRUE" => Ok(TokenType::KEYWORD(Keywords::TRUE)),
            "try" => Ok(TokenType::KEYWORD(Keywords::TRY)),
            "type" => Ok(TokenType::KEYWORD(Keywords::TYPE)),
            "typeof" => Ok(TokenType::KEYWORD(Keywords::TYPEOF)),
            "use" => Ok(TokenType::KEYWORD(Keywords::USE)),
            "with" => Ok(TokenType::KEYWORD(Keywords::WITH)),
            "while" => Ok(TokenType::KEYWORD(Keywords::WHILE)),
            "yield" => Ok(TokenType::KEYWORD(Keywords::YIELD)),

            // Littéraux
            s if s.starts_with('"') && s.ends_with('"') => Ok(TokenType::LITERALS(Literal::STRING(s[1..s.len()-1].to_string()))),
            s if s.parse::<i64>().is_ok() => Ok(TokenType::LITERALS(Literal::INTEGER(s.parse().unwrap()))),
            s if s.starts_with("0x") => Ok(TokenType::LITERALS(Literal::HEXNUMBER(s.to_string()))),
            s if s.parse::<f64>().is_ok() => Ok(TokenType::LITERALS(Literal::FLOAT(Float(s.to_string())))),
            s if s.starts_with('\'') && s.ends_with('\'') && s.len() == 3 =>
                Ok(TokenType::LITERALS(Literal::CHAR(s.chars().nth(1).unwrap()))),

            _ => Ok(TokenType::LITERALS(Literal::IDENTIFIER(s.to_string()))), // Par défaut, considérez-le comme un identifiant
        }
    }
}
