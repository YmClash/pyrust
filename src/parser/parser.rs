#[allow(dead_code)]
use crate::lexer::lex::{SyntaxMode, Token};
use crate::parser::ast::{Assignment, ASTNode, Attribute, BinaryOperation, Block, BlockSyntax, ClassDeclaration, ConstDeclaration, Constructor, Declaration, EnumDeclaration, Expression, Field, Function, FunctionCall, FunctionDeclaration, FunctionSignature, Identifier, Literal, MemberAccess, Mutability, Operator, Parameters, ReturnStatement, Statement, StructDeclaration, TraitDeclaration, Type, TypeCast, UnaryOperation, UnaryOperator, VariableDeclaration, Visibility};
use crate::parser::parser_error::ParserErrorType::{ExpectColon, ExpectFunctionName, ExpectIdentifier, ExpectOperatorEqual, ExpectParameterName, ExpectValue, ExpectVariableName, ExpectedCloseParenthesis, ExpectedOpenParenthesis, ExpectedTypeAnnotation, InvalidFunctionDeclaration, InvalidTypeAnnotation, InvalidVariableDeclaration, UnexpectedEOF, UnexpectedEndOfInput, UnexpectedIndentation, UnexpectedToken, ExpectedParameterName, InvalidAssignmentTarget, ExpectedDeclaration};
use crate::parser::parser_error::{ParserError, ParserErrorType, Position};
use crate::tok::{Delimiters, Keywords, Operators, TokenType};

use num_bigint::BigInt;
use crate::parser::ast::Declaration::Variable;
//////////////////////Debut///////////////////////////

pub struct Parser {
    tokens: Vec<Token>, // liste des tokens genere par le lexer
    current: usize,     // index du token actuel
    syntax_mode: SyntaxMode,
    indent_level: Vec<usize>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, syntax_mode: SyntaxMode) -> Self {
        Parser {
            tokens,
            current: 0,
            syntax_mode,
            indent_level: vec![0],
        }
    }

    fn parse_program(&mut self) -> Result<ASTNode, ParserError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }
        Ok(ASTNode::Program(statements))
    }


    pub fn current_position(&self) -> Position {
        Position {
            index: self.current,
        }
    }

    /// fonction pour aider le parsing des blocs
    fn parse_block(&mut self, syntax: BlockSyntax) -> Result<ASTNode, ParserError> {
        match self.syntax_mode{
            SyntaxMode::Indentation => self.parse_indented_block(),
            SyntaxMode::Braces => self.parse_braced_block(),

        }
    }
    fn current_indent_level(&self) -> usize {
        if let Some(TokenType::INDENT) = self.current_token().map(|t| &t.token_type) {
            self.get_current_indent_level()
        } else {
            0
        }
    }
    pub fn get_current_indent_level(&self) -> usize {
        *self.indent_level.last().unwrap_or(&0)
    }

    fn parse_indented_block(&mut self) -> Result<ASTNode, ParserError> {
        println!("Parsing indented block");
        self.consume(TokenType::NEWLINE)?;
        self.consume(TokenType::INDENT)?;

        let mut statements = Vec::new();
        let initial_indent = self.current_indent_level();

        while !self.is_at_end() && self.current_indent_level() >= initial_indent {
            if self.match_token(&[TokenType::DEDENT]) {
                break;
            }

            let stmt = self.parse_statement()?;
            statements.push(stmt);

            // Consommer les newlines après chaque instruction
            while self.match_token(&[TokenType::NEWLINE]) {
                self.advance();
            }
        }

        self.consume(TokenType::DEDENT)?;
        Ok(ASTNode::Block(Block{
            statements,
            syntax_mode:BlockSyntax::Indentation,
        }))
    }

    fn parse_braced_block(&mut self) -> Result<ASTNode, ParserError> {
        println!("Parsing braced block");
        self.consume(TokenType::DELIMITER(Delimiters::LCURBRACE))?;

        let mut statements = Vec::new();

        while !self.match_token(&[TokenType::DELIMITER(Delimiters::RCURBRACE)]) {
            if self.is_at_end() {
                return Err(ParserError::new(UnexpectedEndOfInput, self.current_position()));
            }

            let stmt = self.parse_statement()?;
            statements.push(stmt);

            // Consommer le point-virgule si présent
            if self.match_token(&[TokenType::DELIMITER(Delimiters::SEMICOLON)]) {
                self.advance();
            }

            // Consommer les newlines
            while self.match_token(&[TokenType::NEWLINE]) {
                self.advance();
            }
        }

        self.consume(TokenType::DELIMITER(Delimiters::RCURBRACE))?;

        Ok(ASTNode::Block(Block{
            statements,
            syntax_mode:BlockSyntax::Braces,
        }))

    }

    fn begin_block(&mut self) {
        // self.indent_level.push(self.current_token().unwrap().indent);
        todo!()
    }

    fn end_block(&mut self) {
        // self.indent_level.pop();
        todo!()
    }



    fn parse_statement(&mut self) -> Result<ASTNode, ParserError> {
        if self.match_token(&[TokenType::KEYWORD(Keywords::RETURN)]) {
            self.parse_return_statement()
        } else if self.match_token(&[TokenType::KEYWORD(Keywords::IF)]){
            self.parse_if_statement()
        } else if self.match_token(&[TokenType::KEYWORD(Keywords::WHILE)]) {
            self.parse_while_statement()
        } else if self.match_token(&[TokenType::KEYWORD(Keywords::FOR)]) {
            self.parse_for_statement()
        } else {
            self.parse_expression_statement()
        }

    }

    /// fonction pour parser les expressions

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        self.parse_binary_expression(0)

    }

    fn parse_expression_statement(&mut self) -> Result<ASTNode, ParserError> {
        todo!()
    }

    fn parse_unary_expression(&mut self) -> Result<Expression, ParserError> {
        if self.match_token(&[
            TokenType::OPERATOR(Operators::MINUS),
            TokenType::OPERATOR(Operators::EXCLAMATION),
            TokenType::OPERATOR(Operators::AMPER),
        ]) {
            let operator = match self.previous_token().unwrap().token_type {
                TokenType::OPERATOR(Operators::MINUS) => UnaryOperator::Negative,
                TokenType::OPERATOR(Operators::EXCLAMATION) => UnaryOperator::Not,
                TokenType::OPERATOR(Operators::AMPER) => UnaryOperator::Reference,
                _ => unreachable!(),
            };
            let right = self.parse_unary_expression()?;
            return Ok(Expression::UnaryOperation(UnaryOperation {
                operator,
                operand: Box::new(right),
            }));
        }
        self.parse_postfix()

    }

    fn parse_primary_expression(&mut self) -> Result<Expression, ParserError> {
        println!("Début du parsing de l'expression primaire, current_token = {:?}", self.current_token());
        if let Some(token) = self.current_token() {
            let expr = match &token.token_type {
                TokenType::INTEGER { value } => {
                    let value = value.clone();
                    self.advance();
                    Expression::Literal(Literal::Integer { value })
                }
                TokenType::FLOAT { value } => {
                    let value = *value;
                    self.advance();
                    Expression::Literal(Literal::Float { value })
                }
                TokenType::STRING { value, .. } => {
                    let value = value.clone();
                    self.advance();
                    Expression::Literal(Literal::String(value))
                }
                TokenType::KEYWORD(Keywords::TRUE) => {
                    self.advance(); // Consomme le token
                    Expression::Literal(Literal::Boolean(true))
                }
                TokenType::KEYWORD(Keywords::FALSE) => {
                    self.advance(); // Consomme le token
                    Expression::Literal(Literal::Boolean(false))
                }
                // TokenType::KEYWORD(Keywords::TRUE) => Expression::Literal(Literal::Boolean(true)),
                // TokenType::KEYWORD(Keywords::FALSE) => Expression::Literal(Literal::Boolean(false)),

                TokenType::KEYWORD(Keywords::SELF) =>{
                    self.advance();
                    let name = "self".to_string();
                    Expression::Identifier(name)

                }


                TokenType::IDENTIFIER { name } => {
                    let name = name.clone();
                    self.advance();
                    Expression::Identifier(name)
                }
                TokenType::DELIMITER(Delimiters::LPAR) => {
                    self.advance(); // Consomme '('
                    let expr = self.parse_expression()?;
                    self.consume(TokenType::DELIMITER(Delimiters::RPAR))?; // Consomme ')'
                    expr
                }
                // TokenType::DELIMITER(Delimiters::LPAR) => {
                //     self.advance();
                //     let expr = self.parse_expression()?;
                //     if let Some(token) = self.current_token() {
                //         if matches!(token.token_type, TokenType::DELIMITER(Delimiters::RPAR)) {
                //             expr
                //         } else {
                //             return Err(ParserError::new(
                //                 ExpectedCloseParenthesis,
                //                 self.current_position(),
                //             ));
                //         }
                //     } else {
                //         return Err(ParserError::new(
                //             UnexpectedEndOfInput,
                //             self.current_position(),
                //         ));
                //     }
                // }
                _ => return Err(ParserError::new(UnexpectedToken, self.current_position())),
            };
            Ok(expr)
        } else {
            Err(ParserError::new(
                UnexpectedEndOfInput,
                self.current_position(),
            ))
        }

    }

    fn parse_binary_expression(&mut self, min_precedence: u8) -> Result<Expression, ParserError> {
        let mut left = self.parse_unary_expression()?;
        while let Some(op) = self.peek_operator() {
            let precedence = self.get_operator_precedence(&op);
            if precedence < min_precedence {
                break;
            }
            self.advance(); // Consomme l'opérateur
            let right = self.parse_binary_expression(precedence + 1)?;
            left = Expression::BinaryOperation(BinaryOperation {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            });
        }
        Ok(left)
    }
    fn parse_lambda_expression(&mut self) -> Result<Expression, ParserError> {
        todo!()
    }

    fn parse_assignment(&mut self) -> Result<Expression, ParserError> {
        let expression = self.parse_equality()?;

        if self.match_token(&[TokenType::OPERATOR(Operators::EQUAL)]) {
            let value = self.parse_assignment()?;
            match expression {
                Expression::Identifier(name) => Ok(Expression::Assignment(Assignment {
                    left: Box::new(Expression::Identifier(name)),
                    right: Box::new(value),
                })),
                Expression::MemberAccess(member_access) => Ok(Expression::Assignment(Assignment {
                    left: Box::new(Expression::MemberAccess(member_access)),
                    right: Box::new(value),
                })),
                _ => Err(ParserError::new(
                    InvalidAssignmentTarget,
                    self.current_position(),
                )),
            }
        } else {
            Ok(expression)
        }
    }

    fn parse_equality(&mut self) -> Result<Expression,ParserError>{
        let mut expression = self.parse_comparison()?;
        while self.match_token(&[
            TokenType::OPERATOR(Operators::EQEQUAL),
            TokenType::OPERATOR(Operators::NOTEQUAL)]){
            let operator = match self.previous_token().unwrap().token_type {
                TokenType::OPERATOR(Operators::EQEQUAL) => Operator::Equal,
                TokenType::OPERATOR(Operators::NOTEQUAL) => Operator::NotEqual,
                _ => unreachable!(),
            };
            let right = self.parse_comparison()?;
            expression = Expression::BinaryOperation(BinaryOperation {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expression)
    }
    fn parse_comparison(&mut self) -> Result<Expression,ParserError>{
        let mut expression = self.parse_term()?;

        while self.match_token(&[
            TokenType::OPERATOR(Operators::LESS),
            TokenType::OPERATOR(Operators::GREATER),
            TokenType::OPERATOR(Operators::LESSEQUAL),
            TokenType::OPERATOR(Operators::GREATEREQUAL),
        ]) {
            let operator = match self.previous_token().unwrap().token_type {
                TokenType::OPERATOR(Operators::LESS) => Operator::LessThan,
                TokenType::OPERATOR(Operators::GREATER) => Operator::GreaterThan,
                TokenType::OPERATOR(Operators::LESSEQUAL) => Operator::LesshanOrEqual,
                TokenType::OPERATOR(Operators::GREATEREQUAL) => Operator::GreaterThanOrEqual,
                _ => unreachable!(),
            };
            let right = self.parse_term()?;
            expression = Expression::BinaryOperation(BinaryOperation {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expression)

    }
    fn parse_term(&mut self) -> Result<Expression,ParserError>{
        let mut expression = self.parse_factor()?;
        while self.match_token(&[
            TokenType::OPERATOR(Operators::PLUS),
            TokenType::OPERATOR(Operators::MINUS),
        ]) {
            let operator = match self.previous_token().unwrap().token_type {
                TokenType::OPERATOR(Operators::PLUS) => Operator::Addition,
                TokenType::OPERATOR(Operators::MINUS) => Operator::Substraction,
                _ => unreachable!(),
            };
            let right = self.parse_factor()?;
            expression = Expression::BinaryOperation(BinaryOperation{
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            })

        }
        Ok(expression)
    }
    fn parse_factor(&mut self) -> Result<Expression,ParserError>{
        let mut expression = self.parse_unary_expression()?;
        while self.match_token(&[
            TokenType::OPERATOR(Operators::STAR),
            TokenType::OPERATOR(Operators::SLASH),
        ]) {
            let operator = match self.previous_token().unwrap().token_type {
                TokenType::OPERATOR(Operators::STAR) => Operator::Multiplication,
                TokenType::OPERATOR(Operators::SLASH) => Operator::Division,
                _ => unreachable!(),
            };
            let right = self.parse_unary_expression()?;
            expression = Expression::BinaryOperation(BinaryOperation{
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            })

        }
        Ok(expression)
    }

    fn parse_postfix(&mut self) -> Result<Expression,ParserError>{
        let mut expression = self.parse_primary_expression()?;
        loop {
            if self.match_token(&[TokenType::DELIMITER(Delimiters::DOT)]){
                let member_name = self.consume_identifier()?;
                expression = Expression::MemberAccess(MemberAccess{
                    object: Box::new(expression),
                    member: member_name,
                });
            // } else if self.match_token(&[TokenType::DELIMITER(Delimiters::LPAR)]) {
            //     // Consomme '('
            //     let arguments = self.parse_arguments_list()?;
            //     expression = Expression::FunctionCall(FunctionCall {
            //         name: Box::new(expression),
            //         arguments: arguments,
            //     });
            //     self.consume(TokenType::DELIMITER(Delimiters::RPAR))?; // Consomme ')'
            // } else {
                break;
            }
        }
        Ok(expression)
    }

    /// fonction pour parser les parametres

    fn parse_arguments_list(&mut self) -> Result<Vec<Expression>, ParserError> {
        todo!()
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<(String, Type)>, ParserError> {
        println!("Début du parsing des paramètres de fonction");
        let mut parameters = Vec::new();

        if !self.match_token(&[TokenType::DELIMITER(Delimiters::RPAR)]) {
            loop {
                //let name = self.consume_parameter_name()?;
                let name = self.consume_identifier()?;
                println!("Nom du paramètre parsé : {}", name);

                if name == "self" {
                    // Si le paramètre est 'self', on n'attend pas de type
                    parameters.push((name, Type::Custom("Self".to_string())));
                } else {
                    self.consume(TokenType::DELIMITER(Delimiters::COLON))?;

                    let param_type = self.parse_type()?;
                    println!("Type du paramètre parsé : {:?}", param_type);
                    parameters.push((name, param_type));
                }
                if self.match_token(&[TokenType::DELIMITER(Delimiters::COMMA)]) {
                    self.consume(TokenType::DELIMITER(Delimiters::COMMA))?;
                } else {
                    break;
                }
            }
        } else {

        }
        //println!("Paramètres parsés : {:?}", parameters);
        Ok(parameters)
    }




    /// fonction pour parser les declarations

    fn parse_declaration(&mut self) -> Result<ASTNode, ParserError> {
        if self.match_token(&[TokenType::KEYWORD(Keywords::LET)]){
            self.parse_variable_declaration()
        } else if self.match_token(&[TokenType::KEYWORD(Keywords::FN)]) {
            self.parse_function_declaration()
        } else if self.match_token(&[TokenType::KEYWORD(Keywords::STRUCT)]) {
            self.parse_struct_declaration()
        } else if self.match_token(&[TokenType::KEYWORD(Keywords::ENUM)]) {
            self.parse_enum_declaration()
        } else if self.match_token(&[TokenType::KEYWORD(Keywords::TRAIT)]) {
            self.parse_trait_declaration()
        } else if self.match_token(&[TokenType::KEYWORD(Keywords::CLASS)]) {
            self.parse_class_declaration()
        } else{
            Err(ParserError::new(ExpectedDeclaration,self.current_position()))
        }


    }

    fn oo(&mut self) -> Result<ASTNode, ParserError> {
        let a: bool;


        todo!()
    }



    /// fonction pour parser les déclarations de variables
    /// Exemple: Brace Mode
    /// // let mut x: int = 5;
    /// // let y: float = 3.14;
    /// // let z = 42;
    /// // let a:bool = true;
    /// Exemple: Indentation Mode
    /// // let mut x: int = 5
    /// // let y: float = 3.14
    /// // let z = 42
    /// // let a:bool = true


    fn parse_variable_declaration(&mut self) -> Result<ASTNode, ParserError> {
        println!("Début du parsing de la déclaration de variable");
        let mutability = self.parse_mutability()?;
        let  name = self.consume_identifier()?;
        let variable_type = if self.match_token(&[TokenType::DELIMITER(Delimiters::COLON)]) {
            Some(self.parse_type()?)
        } else {
            None
        };

        todo!()

    }


    fn parse_const_declaration(&mut self) -> Result<ASTNode, ParserError> {
        println!("Début du parsing de la déclaration de constante");
        todo!()
    }

    fn parse_function_declaration(&mut self) -> Result<ASTNode, ParserError> {
        println!("Début du parsing de la déclaration de fonction");
        todo!()
    }

    fn parse_struct_declaration(&mut self) -> Result<ASTNode, ParserError> {
        todo!()
    }

    fn parse_enum_declaration(&mut self) -> Result<ASTNode, ParserError> {
        todo!()
    }

    fn parse_trait_declaration(&mut self) -> Result<ASTNode, ParserError> {
        todo!()
    }

    fn parse_class_declaration(&mut self) -> Result<ASTNode, ParserError> {
        todo!()
    }

    fn parse_methode_declaration(&mut self) -> Result<ASTNode, ParserError> {
        todo!()
    }

    /// fonction pour parser les types
    fn parse_type(&mut self) -> Result<Type, ParserError> {
        let token = self
            .current_token()
            .ok_or_else(|| ParserError::new(ExpectedTypeAnnotation, self.current_position()))?;

        println!("Parsing type: {:?}", token);

        match &token.token_type {
            TokenType::KEYWORD(Keywords::INT) => {
                self.advance(); // Consomme le token `int`
                Ok(Type::Int)
            }
            TokenType::KEYWORD(Keywords::FLOAT) => {
                self.advance(); // Consomme le token `float`
                Ok(Type::Float)
            }
            TokenType::KEYWORD(Keywords::BOOL) => {
                self.advance(); // Consomme le token `bool`
                Ok(Type::Bool)
            }
            TokenType::KEYWORD(Keywords::STR) => {
                self.advance(); // Consomme le token `string`
                Ok(Type::String)
            }
            TokenType::KEYWORD(Keywords::CHAR) => {
                self.advance(); // Consomme le token `char`
                Ok(Type::Char)
            }
            _ => {
                println!("Unexpected token: {:?}", token);
                // Si le token actuel n'est pas un type valide, renvoyer une erreur
                Err(ParserError::new(
                    InvalidTypeAnnotation,
                    self.current_position(),
                ))
            }
        }
    }

    fn parse_type_cast(&mut self,expr: Expression) -> Result<Expression, ParserError> {
        todo!()
    }


    fn parse_generic_type(&mut self) -> Result<Type, ParserError> {
        todo!()
    }

    /// fonction  pour parser la mutabilité et la visibilité
    fn parse_mutability(&mut self) -> Result<Mutability, ParserError> {
        if self.match_token(&[TokenType::KEYWORD(Keywords::MUT)]){
            Ok(Mutability::Mutable)
        } else {
            Ok(Mutability::Immutable)
        }
    }
    fn parse_visibility(&mut self) -> Result<Visibility, ParserError> {
        if self.match_token(&[TokenType::KEYWORD(Keywords::PUB)]){
            Ok(Visibility::Public)
        } else {
            Ok(Visibility::Private)
        }
    }

    /// fonction pour le gestion de structure de controle
    fn parse_if_statement(&mut self) -> Result<ASTNode, ParserError> {
        todo!()
    }
    fn parse_while_statement(&mut self) -> Result<ASTNode, ParserError> {
        todo!()
    }
    fn parse_for_statement(&mut self) -> Result<ASTNode, ParserError> {
        todo!()
    }
    fn parse_return_statement(&mut self) -> Result<ASTNode, ParserError> {
        todo!()
    }


    /// fonction pour la gestion des emprunts
    fn parse_borrow(&mut self) -> Result<Expression, ParserError> {
        if self.match_token(&[TokenType::OPERATOR(Operators::AMPER)]){
            let mutable = self.match_token(&[TokenType::KEYWORD(Keywords::MUT)]);
            let expression = self.parse_expression()?;
            Ok(Expression::UnaryOperation(UnaryOperation{
                operator: if mutable { UnaryOperator::ReferenceMutable} else {UnaryOperator::Reference},
                operand: Box::new(expression),
            }))
        } else {
            self.parse_primary_expression()
        }

    }

    // fn parse_annotation(&mut self) -> Result<Attribute, ParserError> {
    //     todo!()
    // }






    // fonction utilitaire pour aide au parsing

    fn is_operator(&self,token_type: &TokenType) -> bool {
        todo!()
    }

    fn get_operator_precedence(&self, operator: &Operator) -> u8 {
        match operator {
            Operator::Multiplication | Operator::Division | Operator::Modulo => 5,
            Operator::Addition | Operator::Substraction => 4,
            Operator::LessThan | Operator::GreaterThan | Operator::LesshanOrEqual | Operator::GreaterThanOrEqual => 3,
            Operator::Equal | Operator::NotEqual => 2,
            Operator::And | Operator::Or => 1,
            _ => 0,
        }
    }

    fn peek_operator(&self) -> Option<Operator> {
        match self.current_token()?.token_type {
            TokenType::OPERATOR(Operators::PLUS) => Some(Operator::Addition),
            TokenType::OPERATOR(Operators::MINUS) => Some(Operator::Substraction),
            TokenType::OPERATOR(Operators::STAR) => Some(Operator::Multiplication),
            TokenType::OPERATOR(Operators::SLASH) => Some(Operator::Division),
            TokenType::OPERATOR(Operators::PERCENT) => Some(Operator::Modulo),
            TokenType::OPERATOR(Operators::LESS) => Some(Operator::LessThan),
            TokenType::OPERATOR(Operators::GREATER) => Some(Operator::GreaterThan),
            TokenType::OPERATOR(Operators::LESSEQUAL) => Some(Operator::LesshanOrEqual),
            TokenType::OPERATOR(Operators::GREATEREQUAL) => Some(Operator::GreaterThanOrEqual),
            TokenType::OPERATOR(Operators::EQEQUAL) => Some(Operator::Equal),
            TokenType::OPERATOR(Operators::NOTEQUAL) => Some(Operator::NotEqual),
            TokenType::OPERATOR(Operators::AND) => Some(Operator::And),
            TokenType::OPERATOR(Operators::OR) => Some(Operator::Or),
            _ => None,
        }

    }

    /// fonction pour la gestion des


    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }
    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous_token()
    }

    fn peek_token(&self) -> Option<&Token>{
        self.tokens.get(self.current)
    }
    fn peek_next_token(&self) -> Option<&Token>{
        todo!()
    }

    fn previous_token(&self) -> Option<&Token> {
        if self.current > 0 {
            // &self.tokens(self.current - 1)
            Some(&self.tokens[self.current - 1])
        } else { None }
    }

    fn is_at_end(&self) -> bool{
        self.current >= self.tokens.len()
    }

    ///  Fonctions de Vérification et de Correspondance des Tokens

    fn match_token(&mut self,expected:&[TokenType]) -> bool {
        if self.check(expected){
            self.advance();
            return true
        } else {
            false
        }

    }

    fn check(&self,expected:&[TokenType]) -> bool {
        if let Some(token) = self.current_token(){
            expected.contains(&token.token_type)
        } else {
            false
        }
    }

    pub fn consume(&mut self, expected: TokenType) -> Result<Token, ParserError> {
        // on clone le token actuel pour ne pas avoir de problem avec le borrow checker
        let current_token = self.current_token().cloned().ok_or_else(|| {
            self.print_surrounding_tokens(); // Affiche les tokens autour de l'erreur
            ParserError::new(UnexpectedEOF, self.current_position())
        })?;

        if current_token.token_type == expected {
            self.advance(); // Avance au prochain token
            Ok(current_token.clone()) // Renvoie le token consommé
        } else {
            self.print_surrounding_tokens(); // Affiche les tokens autour de l'erreur
            Err(ParserError::new(UnexpectedToken, self.current_position()))
        }
    }

    /// fonctontion  pour aider a comsume les tokens

    fn consume_identifier(&mut self) -> Result<String, ParserError> {
        let current_token = self.current_token().ok_or_else(|| ParserError::new(UnexpectedEOF,self.current_position()))?;
        if let TokenType::IDENTIFIER {name:_} = &current_token.token_type{
            let name = current_token.text.clone();
            self.advance();
            Ok(name)
        } else { Err(ParserError::new(ExpectIdentifier,self.current_position())) }

    }

    /// Fonction pour afficher les tokens autour de l'erreur
    pub fn create_error_with_context(&self, error_type: ParserErrorType) -> ParserError {
        self.print_surrounding_tokens();
        ParserError::new(
            error_type,
            Position {
                index: self.current,
            },
        )
    }

    fn print_surrounding_tokens(&self) {
        let prev_token = if self.current > 0 {
            Some(&self.tokens[self.current - 1])
        } else {
            None
        };
        let current_token = self.current_token();
        let next_token = if self.current + 1 < self.tokens.len() {
            Some(&self.tokens[self.current + 1])
        } else {
            None
        };
        println!("");
        println!("---------------- Token Error Context--by-YmC ----------");
        if let Some(prev) = prev_token {
            println!("Previous Token: {:?}", prev);
        }
        if let Some(current) = current_token {
            println!("Current Token: {:?}", current);
        }
        if let Some(next) = next_token {
            println!("Next Token: {:?}", next);
        }
        println!("----------------------------------------------------------");
        println!("");
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous_token().map_or(false, |t| t.token_type == TokenType::DELIMITER(Delimiters::SEMICOLON)) {
                return;
            }

            match self.peek_token().map(|t| &t.token_type) {
                Some(TokenType::KEYWORD(Keywords::CLASS)) | Some(TokenType::KEYWORD(Keywords::FN)) | Some(TokenType::KEYWORD(Keywords::LET)) |
                Some(TokenType::KEYWORD(Keywords::FOR)) | Some(TokenType::KEYWORD(Keywords::IF)) | Some(TokenType::KEYWORD(Keywords::WHILE)) |
                Some(TokenType::KEYWORD(Keywords::RETURN)) => return,
                _ => {}
            }

            self.advance();
        }
    }










}

//by YmC






///////////////////////fin essai//////////////////////////////

// ////////////////////fin de mon  parse/////////////////////// */
