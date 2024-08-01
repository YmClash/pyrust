use crate::lex::{Lexer, TokenType};

mod lex;

fn main() {
    let source_code = "let x = 5";


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


/*
pub enum TokenType {
     AND,
    AS,
    ASSERT,
    BREAK,
    CLASS,
    CONTINUE,
    DEF,
    DEL,
    DO,  // new keyword
    ELIF,
    ELSE,
    EXCEPT,
    FINALLY,
    FOR,
    FROM,
    GLOBAL, // maybe instead of global we can use PUB
    IF,
    IMPORT, // maybe instead of import we can use USE
    IN,
    IS,
    LAMBDA,
    LET,
    LOOP,
    MUT,
    //NONLOCAL, // maybe instead of nonlocal we can use PRIV
    NOT,
    OR,
    OPEN,
    PASS,
    PRINT,
    PUB,
    RAISE,
    RETURN,
    TRY,
    WHILE,
    WITH,
    YIELD,
    ENDWHILE,
    // Operators.
    EQ,          //
    PLUS,        // +
    MINUS,       // -
       ARROW,       // ->
    ASTERISK,   // *
    SLASH,      // /
    MOD,
    ARROW,       // ->
    COLON,   // :
    SEMICOLON, // ;
    POINT,     // .
    COMMA,    // ,
    LPAREN,     // (
    RPAREN,      // )
    EQEQ,       // ==
    NOTEQ,      // !=
    LT,          // <
    LTEQ,       // <=
    GT,         // >
    GTEQ,        // >=

*/