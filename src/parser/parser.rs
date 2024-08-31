//
// use crate::lex::{SyntaxMode, Token};
// use crate::tok::{Delimiters, Keywords, Operators, TokenType};
// use std::collections::HashMap;
// use num_bigint::BigInt;
//
//
//
// #[allow(dead_code)]
// pub struct Parser{
//     tokens: Vec<Token>,
//     current_token: usize,
//     syntax_mode: SyntaxMode,
// }
//
// // impl PartialEq<TokenType> for &TokenType {
// //     fn eq(&self, other: &TokenType) -> bool {
// //         todo!()
// //     }
// // }
// /*
// impl Parser{
//     pub fn new(tokens: Vec<Token>,syntax_mode: SyntaxMode) -> Self{
//         Parser{
//             tokens,
//             current_token: 0,
//             syntax_mode,
//         }
//     }
//
//     //methode principale
//     pub fn parse(&mut self) -> Result<ASTNode, String> {
//         let mut program = Vec::new();
//         while !self.is_at_end() {
//             match self.parse_declaration() {
//                 Ok(node) => program.push(node),
//                 Err(e) => return Err(e),
//             }
//         }
//         Ok(ASTNode::Program(program))
//     }
//
//     fn parse_declaration(&mut self) -> Result<ASTNode, String> {
//         if self.match_token(&[TokenType::KEYWORD(Keywords::DEF)]) {
//             self.parse_function_declaration()
//         } else if self.match_token(&[TokenType::KEYWORD(Keywords::LET)]) {
//             self.parse_variable_declaration()
//         } else {
//             self.parse_statement()
//         }
//     }
//     fn parse_fonction_declaration(&mut self) -> Result<ASTNode, String> {
//         let name = self.consuume_identifier("Expected function name")?;
//         self.consume(TokenType::DELIMITER(Delimiters::LPAR), "Expected '(' after function name")?;
//         let parameters = self.parse_parameters()?;
//         self.consume(TokenType::DELIMITER(Delimiters::RPAR), "Expected ')' after function parameters")?;
//
//         let body = if self.syntax_mode == SyntaxMode::Braces{
//             self.consume(TokenType::DELIMITER(Delimiters::LCURBRACE), "Expected '{' before function body")?;
//             let body = self.parse_block()?;
//             self.consume(TokenType::DELIMITER(Delimiters::RCURBRACE), "Expected '}' after function body")?;
//             body
//         } else {
//             self.consume(TokenType::DELIMITER(Delimiters::COLON), "Expected ':' before function return type")?;
//             self.parse_block()?
//         };
//
//         Ok(ASTNode::FunctionDeclaration {
//             name,
//             parameters,
//             body,
//         })
//
//     }
//     fn parse_parameters(&mut self) ->Result<Vec<String>,String> {
//         todo!("Implémentation de la logique pour parser les paramètres")
//     }
//     fn parse_variable_declaration(&mut self) -> Result<ASTNode,String>{
//         let name = self.consume_identifier("Expected variable name")?;
//         self.consume(TokenType::OPERATOR(Operators::EQUAL), "Expected '=' after variable name")?;
//         let value = self.parse_expression()?;
//         self.consume_statement_end("Expect ';' or newline after declaration ")?;
//
//         Ok(ASTNode::VariableDeclaration {
//             name: String::from(name),//////a revoir
//             value:Box::new(value),
//         })
//     }
//     //
//     fn parse_statement(&mut self) ->Result<ASTNode,String>{
//         if self.match_keyword(&[Keywords::IF]){
//             self.parse_if_statement()
//         }else if self.match_keyword(&[Keywords::WHILE]){
//             self.parse_while_statement()        //ansi de suite
//         }else {
//             self.parse_expression_statement()
//         }
//     }
//     fn parse_if_statement(&mut self) -> Result<ASTNode,String>{
//         //implémentation de la logique pour parser une instruction if
//         todo!()
//     }
//     fn parse_while_statement(&mut self) -> Result<ASTNode,String> {
//         //implémentation de la logique pour parser une instruction while
//         todo!()
//     }
//     fn parse_expression_statement(&mut self) -> Result<ASTNode,String> {
//         let expression = self.parse_expression()?;
//         self.consume_statement_end("Expect ';' or newline after expression")?;
//         Ok(expression)
//
//     }
//
//     fn parse_expression(&mut self) -> Result<ASTNode,String>{
//         //implémentation de la logique pour parser une expression
//         self.parse_binary_operation()
//     }
//     fn parse_binary_expression(&mut self) -> Result<ASTNode, String> {
//         let mut expr = self.parse_unary_expression()?;
//
//         while let Some(op) = self.match_operator(&[Operators::PLUS,
//             Operators::MINUS,
//             Operators::STAR,
//             Operators::SLASH/* autres opérateurs */]) {
//             let right = self.parse_unary_expression()?;
//             expr = ASTNode::BinaryOperation {
//                 left: Box::new(expr),
//                 operators: op,
//                 right: Box::new(right),
//             };
//         }
//
//         Ok(expr)
//     }
//
//     fn parse_unary_expression(&mut self) -> Result<ASTNode, String> {
//         if let Some(op) = self.match_operator(&[Operators::MINUS, Operators::NOTEQUAL]) {
//             let expr = self.parse_unary_expression()?;
//             Ok(ASTNode::UnaryOperation {
//                 operator: op,
//                 operand: Box::new(expr),
//             })
//         } else {
//             self.parse_primary_expression()
//         }
//     }
//
//     fn parse_primary_expression(&mut self) -> Result<ASTNode,String> {
//         if let Some(TokenType::INTEGER { value }) = self.peek().map(|t| &t.token_type) {
//             self.advance();
//             Ok(ASTNode::Literal(LiteralValue::Integer(*value)))
//         } else if let Some(TokenType::IDENTIFIER { name }) = self.peek().map(|t| &t.token_type) {
//             self.advance();
//             Ok(ASTNode::Identifier(name.clone()))
//         } else {
//             Err("Unexpected token in expression".to_string())
//         }
//     }
//
//     fn parse_block(&mut self) -> Result<ASTNode,String>{
//         let mut statements = Vec::new();
//         if self.syntax_mode == SyntaxMode::Indentation{
//             self.consume(TokenType::INDENT,"Expected indentation before block")?;
//             while !self.check(&TokenType::DEDENT) && !self.is_at_end(){
//                 statements.push(self.parse_declaration()?);
//             }
//             self.consume(TokenType::DEDENT,"Expected dedentation after block")?;
//         } else {
//             while !self.check(&TokenType::DELIMITER(Delimiters::RCURBRACE)) && !self.is_at_end(){
//                 statements.push(self.parse_declaration()?);
//             }
//         }
//         Ok(ASTNode::Block(statements))
//     }
//
//     //methode utilitaire et de support
//     fn match_keyword(&mut self, keywords: &[Keywords; 1]) ->bool{
//         for &keyword in keywords{
//             if self.check(&TokenType::KEYWORD(keyword)){
//                 self.advance();
//                 return true;
//             }
//         }
//         return false;
//     }
//
//     fn match_token(&mut self, types: &[TokenType]) -> bool {
//         for t in types {
//             if self.check(t) {
//                 self.advance();
//                 return true;
//             }
//         }
//         false
//     }
//
//
//     fn consume(&mut self,token_type: TokenType,message:&str) -> Result<(),String>{
//         if self.check(&token_type) {
//             self.advance();
//             Ok(())
//         } else {
//             Err(message.to_string())
//         }
//     }
//
//
//     fn consume_identifier(&mut self, message:&str) -> Result<(),String>{
//         if let Some(TokenType::IDENTIFIER { name }) = self.peek().map(|t| &t.token_type) {
//             let name = name.clone();
//             self.advance();
//             Ok(name)
//         }else {
//             Err(message.to_string())
//         }
//
//     }
//     fn consume_statement_end(&mut self, message: &str) -> Result<(), String> {
//         if self.syntax_mode == SyntaxMode::Braces{
//             self.consume(TokenType::DELIMITER(Delimiters::SEMICOLON), message)
//         } else {
//             self.consume(TokenType::NEWLINE, message)
//         }
//     }
//
//     fn check(&self,TokenType:&TokenType) ->bool{
//         *self.peek().map_or(false,|t| &t.token_type == *TokenType)
//     }
//     fn advance(&mut self) -> Option<&Token>{
//         if !self.is_at_end(){
//             self.current_token += 1;
//         }
//         self.previous()
//     }
//     fn is_at_end(&self)-> bool{
//         self.peek().map_or(true,|t| matches!(t.token_type,TokenType::EOF))
//     }
//     fn peek(&self) ->Option<&Token>{
//         self.tokens.get(self.current_token)
//     }
//     fn previous(&self) ->Option<&Token>{
//         self.tokens.get(self.current_token - 1)
//     }
//
//
//
// }
// */
//
//
//
//
//
//


















//
//
//
//     fn parse_declaration(){}
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
//     fn parse_return_statement(){}
//
//     fn parse_expression_statement(){}
//
//
//
//     fn parse_for_statement(){}
//
//     fn match_token(){}
//
//     fn check_token(){}
//
//     fn advance(){}
//
//     fn is_EndOfFile(){}
//
//     fn previous(){}
//
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
//
//
//
// // use crate::lex::{Token, TokenType};
// // use crate::ast::{Statement, Expression, FunctionDef, StructDef, LetStatement, IfStatement, WhileStatement, ForStatement, ReturnStatement, BinaryOp, UnaryOp, FunctionCall, Parameter, Argument, StructField, Type, Literal};
// // use crate::Lexer;
// //
// // struct Parser {
// //     lexer: Lexer,
// //     current_token: Token,
// // }
// //
// // impl Parser {
// //     fn new(lexer: Lexer) -> Self {
// //         let mut parser = Parser {
// //             lexer,
// //             current_token: Token::new("".to_string(), TokenType::EOF),
// //         };
// //         parser.next_token();
// //         parser
// //     }
// //
// //     fn next_token(&mut self) {
// //         self.current_token = self.lexer.get_token();
// //     }
// //
// //     fn parse_program(&mut self) -> Result<ASTNode, String> {
// //         // Implémentez la logique pour parser le programme complet
// //     }
// //
// //     fn parse_statement(&mut self) -> Result<ASTNode, String> {
// //         // Implémentez la logique pour parser une déclaration
// //     }
// //
// //     // Ajoutez d'autres méthodes pour parser différentes parties de la grammaire
// // }
// //
// //
// //
// //
// //
// //
// //
// //
//
//
//
//
//
// ///////////////////////////Second parser///////////////////////////
// /*
// pub struct Parser {
//     tokens: Vec<Token>,
//     current_token: usize,
// }
//
// impl Parser {
//     pub fn new(tokens: Vec<Token>) -> Parser {
//         Parser {
//             tokens,
//             current_token: 0,
//         }
//     }
//
//     fn current(&self) -> &Token {
//         &self.tokens[self.current_token]
//     }
//
//     fn next(&mut self) {
//         self.current_token += 1;
//     }
//
//     fn peek(&self) -> Option<&Token> {
//         self.tokens.get(self.current_token + 1)
//     }
//
//     fn parse_statement(&mut self) -> Result<Statement, String> {
//         match self.current().kind {
//             TokenType::FN => self.parse_function_def(),
//             TokenType::STRUCT => self.parse_struct_def(),
//             TokenType::LET => self.parse_let_statement(),
//             TokenType::IF => self.parse_if_statement(),
//             TokenType::WHILE => self.parse_while_statement(),
//             TokenType::FOR => self.parse_for_statement(),
//             TokenType::RETURN => self.parse_return_statement(),
//             _ => self.parse_expression(),
//         }
//     }
//
//     fn parse_function_def(&mut self) -> Result<Statement, String> {
//         self.next(); // Consume 'fn'
//         let name = self.current().text.clone();
//         self.next(); // Consume IDENT
//         self.next(); // Consume '('
//         let parameters = self.parse_parameter_list()?;
//         self.next(); // Consume ')'
//         self.next(); // Consume '->'
//         let return_type = self.parse_type()?;
//         let body = self.parse_block()?;
//         Ok(Statement::FunctionDef(FunctionDef {
//             name,
//             parameters,
//             return_type,
//             body,
//         }))
//     }
//
//     fn parse_struct_def(&mut self) -> Result<Statement, String> {
//         self.next(); // Consume 'struct'
//         let name = self.current().text.clone();
//         self.next(); // Consume IDENT
//         self.next(); // Consume '{'
//         let fields = self.parse_struct_field_list()?;
//         self.next(); // Consume '}'
//         Ok(Statement::StructDef(StructDef { name, fields }))
//     }
//
//     fn parse_let_statement(&mut self) -> Result<Statement, String> {
//         self.next(); // Consume 'let'
//         let mutable = if self.current().kind == TokenType::MUT {
//             self.next(); // Consume 'mut'
//             true
//         } else {
//             false
//         };
//         let name = self.current().text.clone();
//         self.next(); // Consume IDENT
//         self.next(); // Consume ':'
//         let type_ = self.parse_type()?;
//         self.next(); // Consume '='
//         let value = self.parse_expression()?;
//         Ok(Statement::LetStatement(LetStatement {
//             mutable,
//             name,
//             type_,
//             value,
//         }))
//     }
//
//     fn parse_if_statement(&mut self) -> Result<Statement, String> {
//         self.next(); // Consume 'if'
//         let condition = self.parse_expression()?;
//         let then_block = self.parse_block()?;
//         let mut elif_blocks = Vec::new();
//         while self.current().kind == TokenType::ELIF {
//             self.next(); // Consume 'elif'
//             let elif_condition = self.parse_expression()?;
//             let elif_block = self.parse_block()?;
//             elif_blocks.push((elif_condition, elif_block));
//         }
//         let else_block = if self.current().kind == TokenType::ELSE {
//             self.next(); // Consume 'else'
//             Some(self.parse_block()?)
//         } else {
//             None
//         };
//         Ok(Statement::IfStatement(IfStatement {
//             condition,
//             then_block,
//             elif_blocks,
//             else_block,
//         }))
//     }
//
//     fn parse_while_statement(&mut self) -> Result<Statement, String> {
//         self.next(); // Consume 'while'
//         let condition = self.parse_expression()?;
//         let body = self.parse_block()?;
//         Ok(Statement::WhileStatement(WhileStatement { condition, body }))
//     }
//
//     fn parse_for_statement(&mut self) -> Result<Statement, String> {
//         self.next(); // Consume 'for'
//         let variable = self.current().text.clone();
//         self.next(); // Consume IDENT
//         self.next(); // Consume 'in'
//         let iterable = self.parse_expression()?;
//         let body = self.parse_block()?;
//         Ok(Statement::ForStatement(ForStatement {
//             variable,
//             iterable,
//             body,
//         }))
//     }
//
//     fn parse_return_statement(&mut self) -> Result<Statement, String> {
//         self.next(); // Consume 'return'
//         let value = if self.current().kind != TokenType::SEMICOLON {
//             Some(self.parse_expression()?)
//         } else {
//             None
//         };
//         Ok(Statement::ReturnStatement(ReturnStatement { value }))
//     }
//
//     fn parse_expression(&mut self) -> Result<Expression, String> {
//         match self.current().kind {
//             TokenType::IDENT => self.parse_identifier(),
//             TokenType::NUMBER | TokenType::STRING | TokenType::TRUE | TokenType::FALSE => self.parse_literal(),
//             TokenType::LPAREN => self.parse_grouped_expression(),
//             TokenType::PLUS | TokenType::MINUS | TokenType::ASTERISK | TokenType::SLASH | TokenType::MOD | TokenType::EQEQ | TokenType::NOTEQ | TokenType::LT | TokenType::LTEQ | TokenType::GT | TokenType::GTEQ | TokenType::AND | TokenType::OR => self.parse_binary_op(),
//             TokenType::BANG | TokenType::MINUS => self.parse_unary_op(),
//             TokenType::IDENT => self.parse_function_call(),
//             _ => Err(format!("Unexpected token: {:?}", self.current())),
//         }
//     }
//
//     fn parse_identifier(&mut self) -> Result<Expression, String> {
//         let name = self.current().text.clone();
//         self.next();
//         Ok(Expression::Identifier(name))
//     }
//
//     fn parse_literal(&mut self) -> Result<Expression, String> {
//         let value = match self.current().kind {
//             TokenType::NUMBER => Literal::Number(self.current().text.parse().unwrap()),
//             TokenType::STRING => Literal::String(self.current().text.clone()),
//             TokenType::TRUE => Literal::Boolean(true),
//             TokenType::FALSE => Literal::Boolean(false),
//             _ => return Err(format!("Unexpected token: {:?}", self.current())),
//         };
//         self.next();
//         Ok(Expression::Literal(value))
//     }
//
//     fn parse_grouped_expression(&mut self) -> Result<Expression, String> {
//         self.next(); // Consume '('
//         let expr = self.parse_expression()?;
//         self.next(); // Consume ')'
//         Ok(expr)
//     }
//
//     fn parse_binary_op(&mut self) -> Result<Expression, String> {
//         let left = self.parse_expression()?;
//         let operator = match self.current().kind {
//             TokenType::PLUS => BinaryOp::Add,
//             TokenType::MINUS => BinaryOp::Subtract,
//             TokenType::ASTERISK => BinaryOp::Multiply,
//             TokenType::SLASH => BinaryOp::Divide,
//             TokenType::MOD => BinaryOp::Modulo,
//             TokenType::EQEQ => BinaryOp::Equal,
//             TokenType::NOTEQ => BinaryOp::NotEqual,
//             TokenType::LT => BinaryOp::LessThan,
//             TokenType::LTEQ => BinaryOp::LessThanOrEqual,
//             TokenType::GT => BinaryOp::GreaterThan,
//             TokenType::GTEQ => BinaryOp::GreaterThanOrEqual,
//             TokenType::AND => BinaryOp::And,
//             TokenType::OR => BinaryOp::Or,
//             _ => return Err(format!("Unexpected token: {:?}", self.current())),
//         };
//         self.next();
//         let right = self.parse_expression()?;
//         Ok(Expression::BinaryOp(BinaryOp {
//             left: Box::new(left),
//             operator,
//             right: Box::new(right),
//         }))
//     }
//
//     fn parse_unary_op(&mut self) -> Result<Expression, String> {
//         let operator = match self.current().kind {
//             TokenType::BANG => UnaryOp::Not,
//             TokenType::MINUS => UnaryOp::Negate,
//             _ => return Err(format!("Unexpected token: {:?}", self.current())),
//         };
//         self.next();
//         let expr = self.parse_expression()?;
//         Ok(Expression::UnaryOp(UnaryOp {
//             operator,
//             expr: Box::new(expr),
//         }))
//     }
//
//     fn parse_function_call(&mut self) -> Result<Expression, String> {
//         let name = self.current().text.clone();
//         self.next(); // Consume IDENT
//         self.next(); // Consume '('
//         let arguments = self.parse_argument_list()?;
//         self.next(); // Consume ')'
//         Ok(Expression::FunctionCall(FunctionCall { name, arguments }))
//     }
//
//     fn parse_parameter_list(&mut self) -> Result<Vec<Parameter>, String> {
//         let mut parameters = Vec::new();
//         if self.current().kind != TokenType::RPAREN {
//             parameters.push(self.parse_parameter()?);
//             while self.current().kind == TokenType::COMMA {
//                 self.next(); // Consume ','
//                 parameters.push(self.parse_parameter()?);
//             }
//         }
//         Ok(parameters)
//     }
//
//     fn parse_parameter(&mut self) -> Result<Parameter, String> {
//         let name = self.current().text.clone();
//         self.next(); // Consume IDENT
//         self.next(); // Consume ':'
//         let type_ = self.parse_type()?;
//         Ok(Parameter { name, type_ })
//     }
//
//     fn parse_argument_list(&mut self) -> Result<Vec<Expression>, String> {
//         let mut arguments = Vec::new();
//         if self.current().kind != TokenType::RPAREN {
//             arguments.push(self.parse_expression()?);
//             while self.current().kind == TokenType::COMMA {
//                 self.next(); // Consume ','
//                 arguments.push(self.parse_expression()?);
//             }
//         }
//         Ok(arguments)
//     }
//
//     fn parse_struct_field_list(&mut self) -> Result<Vec<StructField>, String> {
//         let mut fields = Vec::new();
//         if self.current().kind != TokenType::RCURBRACET {
//             fields.push(self.parse_struct_field()?);
//             while self.current().kind == TokenType::COMMA {
//                 self.next(); // Consume ','
//                 fields.push(self.parse_struct_field()?);
//             }
//         }
//         Ok(fields)
//     }
//
//     fn parse_struct_field(&mut self) -> Result<StructField, String> {
//         let name = self.current().text.clone();
//         self.next(); // Consume IDENT
//         self.next(); // Consume ':'
//         let type_ = self.parse_type()?;
//         Ok(StructField { name, type_ })
//     }
//
//     fn parse_type(&mut self) -> Result<Type, String> {
//         let type_ = match self.current().kind {
//             TokenType::I32 => Type::I32,
//             TokenType::F64 => Type::F64,
//             TokenType::BOOL => Type::Bool,
//             TokenType::STR => Type::Str,
//             TokenType::IDENT => Type::Custom(self.current().text.clone()),
//             _ => return Err(format!("Unexpected token: {:?}", self.current())),
//         };
//         self.next();
//         Ok(type_)
//     }
//
//     fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
//         self.next(); // Consume '{'
//         let mut statements = Vec::new();
//         while self.current().kind != TokenType::RCURBRACET {
//             statements.push(self.parse_statement()?);
//         }
//         self.next(); // Consume '}'
//         Ok(statements)
//     }
//
//     pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
//         let mut statements = Vec::new();
//         while self.current().kind != TokenType::EOF {
//             statements.push(self.parse_statement()?);
//         }
//         Ok(statements)
//     }
// }
//
// ////////////////////fin de mon  parse
// */