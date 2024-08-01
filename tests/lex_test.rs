

#[cfg(test)]
mod tests {


    #[test]
    fn test_lexer_identifiers() {
        use pyrust::lex::{Lexer, TokenType};
        let input = "let mut momo = 5 + 5;";


        let expected_tokens = vec![
            TokenType::LET,
            TokenType::MUT,
            TokenType::IDENT,
            TokenType::EQ,
            TokenType::NUMBER,
            TokenType::PLUS,
            TokenType::NUMBER,
            TokenType::SEMICOLON,
            TokenType::EOF,
        ];
        let mut lexer = Lexer::new(input);
        for expected in expected_tokens {
            let token = lexer.get_token();
            assert_eq!(token.kind, expected);
        }

    }
    #[test]
    fn test_sign_token() {
        use pyrust::lex::{Lexer, TokenType};
        let code_source = "+ - * / % . : ; , ( ) [ ] { }";
        let expected_tokens = vec![
            TokenType::PLUS,
            TokenType::MINUS,
            TokenType::ASTERISK,
            TokenType::SLASH,
            TokenType::MOD,
            TokenType::POINT,
            TokenType::COLON,
            TokenType::SEMICOLON,
            TokenType::COMMA,
            TokenType::LPAREN,
            TokenType::RPAREN,
            TokenType::LSQUAREBRACET,
            TokenType::RSQUAREBRACET,
            TokenType::LCURBRACET,
            TokenType::RCURBRACET,
            TokenType::EOF,
        ];
        let mut lexer = Lexer::new(code_source);
        for expected in expected_tokens {
            let token = lexer.get_token();
            assert_eq!(token.kind, expected);
        }
    }

    #[test]
    fn test_complex_token() {
        use pyrust::lex::{Lexer, TokenType};
        let code_source = "== != <= >= -> :: =>";
        let expected_tokens = vec![
            TokenType::EQEQ,
            TokenType::NOTEQ,
            TokenType::LTEQ,
            TokenType::GTEQ,
            TokenType::ARROW,
            TokenType::DCOLON,
            TokenType::SUIVANT,
            TokenType::EOF,
        ];
        let mut lexer = Lexer::new(code_source);
        for expected in expected_tokens {
            let token = lexer.get_token();
            assert_eq!(token.kind, expected);
        }
    }

    #[test]
    fn test_keyword_token() {
        use pyrust::lex::{Lexer, TokenType};
        let code_source = "let mut fn struct import class pass open as break continue if else while for in do return";
        let expected_tokens = vec![
            TokenType::LET,
            TokenType::MUT,
            TokenType::FN,
            TokenType::STRUCT,
            TokenType::IMPORT,
            TokenType::CLASS,
            TokenType::PASS,
            TokenType::OPEN,
            TokenType::AS,
            TokenType::BREAK,
            TokenType::CONTINUE,
            TokenType::IF,
            TokenType::ELSE,
            TokenType::WHILE,
            TokenType::FOR,
            TokenType::IN,
            TokenType::DO,
            TokenType::RETURN,
            TokenType::EOF,
        ];
        let mut lexer = Lexer::new(code_source);
        for expected in expected_tokens {
            let token = lexer.get_token();
            assert_eq!(token.kind, expected);
        }
    }

    #[test]
    fn test_identifier_and_number() {
        let code_source = "variable123 45 7.54 \n string = \" Hello World\"";
        let mut lexer = pyrust::lex::Lexer::new(code_source);
        let token = lexer.get_token();
        assert_eq!(token.kind, pyrust::lex::TokenType::IDENT);
        assert_eq!(token.text, "variable123");

        let token = lexer.get_token();
        assert_eq!(token.kind, pyrust::lex::TokenType::NUMBER);
        assert_eq!(token.text, "45");

        let token = lexer.get_token();
        assert_eq!(token.kind, pyrust::lex::TokenType::NUMBER);
        assert_eq!(token.text, "7.54");

        let token = lexer.get_token();
        assert_eq!(token.kind, pyrust::lex::TokenType::NEWLINE);
        assert_eq!(token.text, "\n");

        let token = lexer.get_token();
        assert_eq!(token.kind, pyrust::lex::TokenType::IDENT);
        assert_eq!(token.text, "string");

        let token = lexer.get_token();
        assert_eq!(token.kind, pyrust::lex::TokenType::EQ);
        assert_eq!(token.text, "=");

        let token = lexer.get_token();
        assert_eq!(token.kind, pyrust::lex::TokenType::STRING);
        assert_eq!(token.text, "Hello World");

    }

    #[test]

    fn test_string(){
        use pyrust::lex::{Lexer, TokenType};
        let code_source = "\" Hello World\"";
        let mut lexer = Lexer::new(code_source);
        let token = lexer.get_token();
        assert_eq!(token.kind, TokenType::STRING);
        assert_eq!(token.text, "Hello World");

    }


    #[test]
    fn test_comment() {
        use pyrust::lex::{Lexer,TokenType};
        let code_source = "# This is a comment\nlet";
        let mut lexer = Lexer::new(code_source);
        let token = lexer.get_token();
        assert_eq!(token.kind, TokenType::NEWLINE);
        let token = lexer.get_token();
        assert_eq!(token.kind, TokenType::LET);
    }

    #[test]
    fn test_complex_input() {
        use pyrust::lex::{Lexer, TokenType};
        let code_source = "def code() \n fn main() {\n    let x = 5+9;\n    print(x);\n}";
        let expected_tokens = vec![
            TokenType::DEF,
            TokenType::IDENT,
            TokenType::LPAREN,
            TokenType::RPAREN,
            TokenType::NEWLINE,
            TokenType::FN,
            TokenType::IDENT,
            TokenType::LPAREN,
            TokenType::RPAREN,
            TokenType::LCURBRACET,
            TokenType::NEWLINE,
            TokenType::LET,
            TokenType::IDENT,
            TokenType::EQ,
            TokenType::NUMBER,
            TokenType::PLUS,
            TokenType::NUMBER,
            TokenType::SEMICOLON,
            TokenType::NEWLINE,
            TokenType::PRINT,
            TokenType::LPAREN,
            TokenType::IDENT,
            TokenType::RPAREN,
            TokenType::SEMICOLON,
            TokenType::NEWLINE,
            TokenType::RCURBRACET,
            TokenType::EOF,

        ];
        let mut lexer = Lexer::new(code_source);
        for expected in expected_tokens{
            let toke =lexer.get_token();
            assert_eq!(toke.kind, expected);
        }
    }




}


