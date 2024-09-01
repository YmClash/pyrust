pub mod lexer;
//mod parser;
mod semantic;
mod utils;
mod codegen;
pub mod parser;

//mod ast;
#[allow(dead_code)]
pub use lexer::lex::Lexer;
pub use lexer::tok;
pub use lexer::error;
pub use crate::lexer::lex::SyntaxMode;


//pub use parser::parser;
//pub use parser::ast;
//pub use parser::error;




//pub use parser::*;

//pub use ast::*;
