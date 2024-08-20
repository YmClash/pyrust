
use std::iter::Peekable;
use std::str::Chars;
use std::collections::HashMap;
use crate::tok::{TokenType, Keywords, Delimiters, Operators, StringKind};



/// structure Token
/// elle contient le text du token, le type du token, la ligne et la colonne
#[allow(dead_code)]
#[derive(Debug)]
pub struct Token{
    pub text: String,
    pub token_type: TokenType,
    line: usize,
    column: usize,
}

/// implementation de la structure Token
#[allow(dead_code)]
impl Token {
    fn new(text: String,token_type: TokenType,line: usize,column: usize) -> Self{
        Token{text, token_type, line, column}
    }
}

/// structure Lexer
#[allow(dead_code)]
pub struct Lexer<'a>{
    source:Peekable<Chars<'a>>,
    current_char: char,
    keywords: HashMap<String, Keywords>,
    operators: HashMap<String, Operators>,
    delimiters: HashMap<String, Delimiters>,
    current_line: usize,
    current_column: usize,
    current_token_text: String,
}

/// implementation  du lexer avec tout les  methodes pour classer les tokens
#[allow(dead_code)]
impl<'a> Lexer<'a> {
    /// creation d'une nouvelle instance de lexer
    pub fn new(code_source: &'a str) -> Self{
        let lexer = Lexer{
            source: code_source.chars().peekable(),
            current_char: '\0',
            keywords: Self::keywords(),
            operators: Self::operators(),
            delimiters: Self::delimiters(),
            current_line: 1,
            current_column: 0,
            current_token_text: String::new(),
        };
        lexer


    }
    /// creation d'une hashmap pour les mots cles
    /// c'est plus facile de les stocker les mots cles dans une hashmap pour les retrouver plus facilement
    fn keywords() ->HashMap<String,Keywords>{
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), Keywords::AND);
        keywords.insert("as".to_string(), Keywords::AS);
        keywords.insert("async".to_string(), Keywords::ASYNC);
        keywords.insert("await".to_string(), Keywords::AWAIT);
        keywords.insert("break".to_string(), Keywords::BREAK);
        keywords.insert("const".to_string(), Keywords::CONST);
        keywords.insert("continue".to_string(), Keywords::CONTINUE);
        keywords.insert("def".to_string(), Keywords::DEF);
        keywords.insert("del".to_string(), Keywords::DEL);
        keywords.insert("elif".to_string(), Keywords::ELIF);
        keywords.insert("else".to_string(), Keywords::ELSE);
        keywords.insert("enum".to_string(), Keywords::ENUM);
        keywords.insert("except".to_string(), Keywords::EXCEPT);
        keywords.insert("false".to_string(), Keywords::FALSE);
        keywords.insert("fn".to_string(), Keywords::FN);
        keywords.insert("for".to_string(), Keywords::FOR);
        keywords.insert("from".to_string(), Keywords::FROM);
        keywords.insert("if".to_string(), Keywords::IF);
        keywords.insert("impl".to_string(), Keywords::IMPL);
        keywords.insert("import".to_string(), Keywords::IMPORT);
        keywords.insert("in".to_string(), Keywords::IN);
        keywords.insert("is".to_string(), Keywords::IS);
        keywords.insert("lambda".to_string(), Keywords::LAMBDA);
        keywords.insert("let".to_string(), Keywords::LET);
        keywords.insert("loop".to_string(), Keywords::LOOP);
        keywords.insert("match".to_string(), Keywords::MATCH);
        keywords.insert("mod".to_string(), Keywords::MOD);
        keywords.insert("mut".to_string(), Keywords::MUT);
        keywords.insert("none".to_string(), Keywords::NONE);
        keywords.insert("not".to_string(), Keywords::NOT);
        keywords.insert("or".to_string(), Keywords::OR);
        keywords.insert("pub".to_string(), Keywords::PUB);
        keywords.insert("pass".to_string(), Keywords::PASS);
        keywords.insert("raise".to_string(), Keywords::RAISE);
        keywords.insert("return".to_string(), Keywords::RETURN);
        keywords.insert("self".to_string(), Keywords::SELF);
        keywords.insert("static".to_string(), Keywords::STATIC);
        keywords.insert("struct".to_string(), Keywords::STRUCT);
        keywords.insert("super".to_string(), Keywords::SUPER);
        keywords.insert("true".to_string(), Keywords::TRUE);
        keywords.insert("try".to_string(), Keywords::TRY);
        keywords.insert("type".to_string(), Keywords::TYPE);
        keywords.insert("typeof".to_string(), Keywords::TYPEOF);
        keywords.insert("use".to_string(), Keywords::USE);
        keywords.insert("with".to_string(), Keywords::WITH);
        keywords.insert("while".to_string(), Keywords::WHILE);
        keywords.insert("yield".to_string(), Keywords::YIELD);
        return keywords;

    }
    /// creation d'une hashmap pour les operateurs
    fn operators() ->HashMap<String,Operators> {
        let mut operators = HashMap::new();
        operators.insert("+".to_string(), Operators::PLUS);
        operators.insert("-".to_string(), Operators::MINUS);
        operators.insert("*".to_string(), Operators::STAR);
        operators.insert("/".to_string(), Operators::SLASH);
        operators.insert("%".to_string(), Operators::PERCENT);
        operators.insert("==".to_string(), Operators::EQEQUAL);
        operators.insert("!=".to_string(), Operators::NOTEQUAL);
        operators.insert("<".to_string(), Operators::LESS);
        operators.insert(">".to_string(), Operators::GREATER);
        operators.insert("<=".to_string(), Operators::LESSEQUAL);
        operators.insert(">=".to_string(), Operators::GREATEREQUAL);
        operators.insert("=".to_string(), Operators::EQUAL);
        operators.insert("++".to_string(), Operators::PLUSEQUAL);
        operators.insert("--".to_string(), Operators::MINEQUAL);
        operators.insert("**".to_string(), Operators::DOUBLESTAR);
        operators.insert("//".to_string(), Operators::DOUBLESLASH);
        operators.insert("&&".to_string(), Operators::AND);
        operators.insert("||".to_string(), Operators::OR);
        operators.insert("!".to_string(), Operators::EXCLAMATION);
        operators.insert("&".to_string(), Operators::AMPER);
        operators.insert("|".to_string(), Operators::VBAR);
        operators.insert("^".to_string(), Operators::CIRCUMFLEX);
        operators.insert("<<".to_string(), Operators::LEFTSHIFT);
        operators.insert(">>".to_string(), Operators::RIGHTSHIFT);
        operators.insert("~".to_string(), Operators::TILDE);
        operators.insert("+=".to_string(), Operators::PLUSEQUAL);
        operators.insert("-=".to_string(), Operators::MINEQUAL);
        operators.insert("*=".to_string(), Operators::STAREQUAL);
        operators.insert("/=".to_string(), Operators::SLASHEQUAL);
        operators.insert("%=".to_string(), Operators::PERCENTEQUAL);
        operators.insert("&=".to_string(), Operators::AMPEREQUAL);
        operators.insert("|=".to_string(), Operators::VBAREQUAL);
        operators.insert("^=".to_string(), Operators::CIRCUMFLEXEQUAL);
        operators.insert("<<=".to_string(), Operators::LEFTSHIFTEQUAL);
        operators.insert(">>=".to_string(), Operators::RIGHTSHIFTEQUAL);
        operators.insert("**=".to_string(), Operators::DOUBLESTAREQUAL);
        operators.insert("//=".to_string(), Operators::DOUBLESLASHEQUAL);
        operators.insert("@".to_string(), Operators::AT);
        operators.insert("@=".to_string(), Operators::ATEQUAL);
        operators.insert("->".to_string(), Operators::RARROW);
        operators.insert(":=".to_string(), Operators::COLONEQUAL);
        operators.insert("*/".to_string(), Operators::STARSLASH);
        operators.insert("/*".to_string(), Operators::SLASHSTAR);
        operators.insert("#".to_string(), Operators::DIESE);
        operators.insert("?".to_string(), Operators::INTERROGATION);
        return operators;
    }

    /// creation d'une hashmap pour les delimiters
    fn delimiters() ->HashMap<String,Delimiters>{
        let mut delimiters = HashMap::new();
        delimiters.insert("(".to_string(), Delimiters::LPAR);
        delimiters.insert(")".to_string(), Delimiters::RPAR);
        delimiters.insert("{".to_string(), Delimiters::LCURBRACE);
        delimiters.insert("}".to_string(), Delimiters::RCURBRACE);
        delimiters.insert("]".to_string(), Delimiters::RSBRACKET);
        delimiters.insert("[".to_string(), Delimiters::LSBRACKET);
        delimiters.insert(";".to_string(), Delimiters::SEMICOLON);
        delimiters.insert(":".to_string(), Delimiters::COLON);
        delimiters.insert(",".to_string(), Delimiters::COMMA);
        delimiters.insert(".".to_string(), Delimiters::DOT);
        delimiters.insert("...".to_string(), Delimiters::ELLIPSIS);
        delimiters.insert("::".to_string(), Delimiters::DOUBLECOLON);

        return delimiters;
    }

        /// methode pour avancer d'un caractere
    #[allow(dead_code)]
    fn next_char(&mut self) -> Option<char> {
        let ch = self.source.next()?;
        self.current_char = ch;
        if ch == '\n' {
            self.current_line += 1;
            self.current_column = 1;
        } else {
            self.current_column += 1;
        }
        Some(ch)
    }

    /// methode pour regarder le prochain caractere sans avancer
    #[allow(dead_code)]
    fn peek_char(&mut self) -> Option<char>{
        self.source.peek().copied()
    }
    /// methode pour regarder le 2eme prochain caractere sans avancer
    #[allow(dead_code)]
    fn peek_next_char(&mut self) -> Option<char> {
        self.source.clone().nth(1)
    }

    /// C'est L'une de 2 methode principal avec fonction tokenize() pour obtenir le token
    /// son role c'est de sauter les espaces et examiner le prochain caractère
    /// Détermine le type de token en fonction de ce caractère.
    /// Appelle la méthode appropriée (comme lex_number(), lex_identifier_or_keyword(), etc.) pour analyser le token complet.
    /// Renvoie une Option<TokenType> représentant un seul token.

    /// methode pour obtenir le token
    pub fn get_token(&mut self) -> Option<TokenType> {
        self.skip_whitespace();

        match self.peek_char() {
            Some('0'..='9') => Some(self.lex_number()),
            Some('a'..='z') | Some('A'..='Z') | Some('_') => Some(self.lex_identifier_or_keyword()),
            Some('"') | Some('\'') => Some(self.lex_string()),
            Some('#') => Some(self.lex_comment()),
            Some('/') => {
                if self.peek_next_char() == Some('/') || self.peek_next_char() == Some('*') {
                    Some(self.lex_comment())
                } else {
                    self.lex_operator()
                }
            }
            Some(ch) if self.delimiters.contains_key(&ch.to_string()) => Some(self.lex_delimiter()),
            Some(ch) if !ch.is_alphanumeric() => self.lex_operator(),
            None => Some(TokenType::EOF),
            _ => Some(self.lex_unknown()),
        }
    }

    ///methode pour les differents types de token de Type Number integer et float
    fn lex_number(&mut self) -> TokenType {
        self.current_token_text.clear();
        let mut number = String::new();
        let mut is_float = false;

        while let Some(&ch) = self.source.peek() {
            if ch.is_digit(10) || (ch == '.' && !is_float) {
                if ch == '.' {
                    is_float = true;
                }
                let ch = self.advance();
                self.current_token_text.push(ch);
                number.push(ch);
            } else {
                break;
            }
        }

        if is_float {
            TokenType::FLOAT { value: number.parse().unwrap() }
        } else {
            TokenType::INTEGER { value: number.parse().unwrap() }
        }
    }

    //fn lex_identifier(){}
    /// methode pour les differents types de token de Type Identifier  ou Keyword
    fn lex_identifier_or_keyword(&mut self) -> TokenType {
        self.current_token_text.clear();
        while let Some(&ch) = self.source.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                let ch = self.advance();
                self.current_token_text.push(ch);  // Ajoute le caractère à la chaîne de texte du token
            } else {
                break;
            }
        }

        if let Some(keyword) = self.keywords.get(&self.current_token_text) {
            TokenType::KEYWORD(keyword.clone())
        } else {
            TokenType::IDENTIFIER { name: self.current_token_text.clone() }
        }
    }

    /// methode pour les differents types de token de Type String
    fn lex_string(&mut self) -> TokenType {
        self.current_token_text.clear();

        let quote = self.advance();  // Consomme le premier guillemet
        let mut value = String::new();
        let mut is_escaped = false;

        while let Some(&ch) = self.source.peek() {
            self.advance();  // Consomme le caractère actuel

            if is_escaped {
                // Gérer les séquences d'échappement
                match ch {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    '\'' => value.push('\''),
                    _ => {
                        // Pour les autres caractères, on les ajoute tels quels
                        value.push('\\');
                        value.push(ch);
                    }
                }
                is_escaped = false;
            } else if ch == '\\' {
                is_escaped = true;  // Activer le flag d'échappement
            } else if ch == quote {
                // Terminer la chaîne à la guillemet fermante
                break;
            } else {
                value.push(ch);
            }
        }

        self.current_token_text = value.clone();
        TokenType::STRING { value, kind: StringKind::NORMAL }
    }

    /// methode pour les differents types de token de Type Operator
    fn lex_operator(&mut self) -> Option<TokenType> {
        self.current_token_text.clear();

        // Regardez les deux prochains caractères pour vérifier les opérateurs composés
        let first_char = self.advance().to_string();
        let mut op = first_char.clone();

        if let Some(&next_char) = self.source.peek() {
            op.push(next_char);  // Regardez l'opérateur composé potentiel
            if self.operators.contains_key(&op) {
                self.advance();  // Consomme le deuxième caractère de l'opérateur composé
                return Some(TokenType::OPERATOR(self.operators[&op].clone()));
            }
        }

        // Si ce n'est pas un opérateur composé, vérifiez l'opérateur simple
        if let Some(operator) = self.operators.get(&first_char) {
            return Some(TokenType::OPERATOR(operator.clone()));
        }

        // Si l'opérateur n'est pas reconnu, retournez un token UNKNOWN
        println!("Unknown token: {}", first_char); // Affichez l'opérateur inconnu
        Some(TokenType::UNKNOWN)
    }

    // fn lex_operator(&mut self) -> Option<TokenType> {
    //     self.current_token_text.clear();
    //
    //     let mut op = String::new();
    //     while let Some(&ch) = self.source.peek() {
    //         if !ch.is_alphanumeric() && !ch.is_whitespace() {
    //             op.push(self.advance());
    //         } else {
    //             break;
    //         }
    //     }
    //
    //     if let Some(operator) = self.operators.get(&op) {
    //         Some(TokenType::OPERATOR(operator.clone()))
    //     } else {
    //         println!("Unknown token: {}", op);
    //         Some(TokenType::UNKNOWN) // Retourner UNKNOWN au lieu de None pour les opérateurs inconnus
    //     }
    // }

    /// methode pour les differents types de token de Type Delimiter
    fn lex_delimiter(&mut self) -> TokenType {
        self.current_token_text.clear();
        let ch = self.advance();
        if let Some(delimiter) = self.delimiters.get(&ch.to_string()) {
            return TokenType::DELIMITER(delimiter.clone());
        } else {
            return TokenType::UNKNOWN;
        }
        // TokenType::DELIMITER(self.delimiters[&ch.to_string()].clone())
    }

    /// methode pour les differents types de token de Type Comment # ou // ou /* */
    fn lex_comment(&mut self) -> TokenType {
        self.current_token_text.clear();
        let start_char = self.advance(); // Consomme le '#' ou le '/'
        let mut comment = String::new();

        match start_char {
            '#' => {
                // Commentaire en ligne commençant par '#'
                while let Some(ch) = self.next_char() {
                    if ch == '\n' {
                        break;
                    }
                    comment.push(ch);
                }
            },
            '/' => {
                if let Some(&next_char) = self.source.peek() {
                    match next_char {
                        '/' => {
                            // Commentaire en ligne commençant par '//'
                            self.advance(); // Consomme le deuxième '/'
                            while let Some(ch) = self.next_char() {
                                if ch == '\n' {
                                    break;
                                }
                                comment.push(ch);
                            }
                        },
                        '*' => {
                            // Commentaire multi-lignes
                            self.advance(); // Consomme le '*'
                            let mut depth = 1;
                            while let Some(ch) = self.next_char() {
                                if ch == '*' && self.peek_char() == Some('/') {
                                    self.advance(); // Consomme le '/'
                                    depth -= 1;
                                    if depth == 0 {
                                        break;
                                    }
                                } else if ch == '/' && self.peek_char() == Some('*') {
                                    self.advance(); // Consomme le '*'
                                    depth += 1;
                                }
                                comment.push(ch);
                            }
                        },
                        _ => {
                            // Ce n'est pas un commentaire, c'est probablement un opérateur de division
                            return TokenType::OPERATOR(Operators::SLASH);
                        }
                    }
                }
            },
            _ => {
                // Ce cas ne devrait jamais se produire si la fonction est appelée correctement
                return TokenType::UNKNOWN;
            }
        }

        self.current_token_text = comment.clone();
        return TokenType::COMMENT(comment);
    }
    ////////////

    /// methode pour avancer d'un caractere
    fn advance(&mut self) -> char {
        let ch = self.source.next().unwrap();
        if ch == '\n' {
            self.current_line += 1; // Incrémenter le numéro de ligne
            self.current_column = 1; // Réinitialiser le numéro de colonne
        } else {
            self.current_column += 1;// sinon incrementer le numero de colonne
        }
        ch
    }

    /// methode pour sauter les espaces
    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.source.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// C'est la deuxieme methode principal avec get_token() pour obtenir les tokens
    /// son role c'est de tokeniser le code source
    /// appel la methode get_token pour obtenir les tokens
    /// elle creeun objet Token pour chaque TokenType retourné par get_token()
    /// elle retourne  un vecteur de tokens Vec<Token>
    /// methode pour tokeniser le code source
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token_type) = self.get_token() {
            let token = Token::new(
                self.current_token_text.clone(),
                token_type.clone(),
                self.current_line,
                self.current_column
            );
            tokens.push(token);
            self.current_token_text.clear();
            if matches!(token_type,TokenType::EOF){
                break;
            }
        }
        return tokens;
    }
    /// methode pour les differents types de token de Type Unknown
    fn lex_unknown(&mut self) -> TokenType{
        let ch = self.advance();
        self.current_token_text = ch.to_string();
        TokenType::UNKNOWN
    }
}











////////////////////////essai/////////////////////////////////////////////
pub fn lox(input:&str) ->Vec<Tok>{
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next(){
        match c {
            '+' => tokens.push(Tok::PLUS),
            '-' => tokens.push(Tok::MINUS),
            '*' => tokens.push(Tok::MUL),
            '/' => tokens.push(Tok::DIV),
            '0'..='9' => {
                let mut number = String::from(c);
                while let Some(&next_char) = chars.peek() {
                    if next_char.is_ascii_digit(){
                        number.push(chars.next().unwrap());
                    } else {
                        break;
                    }

                }
                tokens.push(Tok::NUMBER(number.parse().unwrap()));
            }


            _  => {}
        }
    }
    tokens
}


#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Tok{
    PLUS,
    MINUS,
    MUL,
    DIV,
    NUMBER(i64),

}
