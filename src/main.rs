fn main() {
    println!("Pyrust Compiler ");
    println!("By YmC")
}


#[derive(Debug,PartialEq)]

enum Token {
    Number(i64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
}


fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars:Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len(){
        match chars[i] {
            '0'..='9' => {
                let mut num = chars[i].to_digit(10).unwrap() as i64;
                while i+1 < chars.len() && chars[i+1].is_digit(10){
                    i += 1;
                    num = num * 10 + chars[i].to_digit(10).unwrap() as i64;
                }
                tokens.push(Token::Number(num));
            },
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Multiply),
            '/' => tokens.push(Token::Divide),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            ' ' | '\n' | '\t' => (),
            _ => panic!("Invalid character: {}",chars[i]),
        }
        i += 1;

    }
    tokens


}
























